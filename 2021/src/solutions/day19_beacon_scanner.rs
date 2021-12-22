use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: HashSet<Pos>,
    mappings: Option<Mapping>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos(isize, isize, isize);

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.0.cmp(&other.0);
        if x == Ordering::Equal {
            let y = self.1.cmp(&other.1);
            if x == Ordering::Equal {
                self.2.cmp(&other.2)
            } else {
                y
            }
        } else {
            x
        }
    }
}

#[derive(Debug, Clone)]
struct Mapping(isize, isize, isize);

lazy_static! {
    static ref ALL_MAPPINGS: Vec<Mapping> = Mapping::all();
}

impl Mapping {
    /// Returns all 24 different mappings that can be used
    fn all() -> Vec<Self> {
        [
            (1, 2, 3), // Z on top
            (2, -1, 3),
            (-1, -2, 3),
            (-2, 1, 3),
            (-3, 2, 1), // X on top
            (2, 3, 1),
            (3, -2, 1),
            (-2, -3, 1),
            (-3, -1, 2), // Y on top
            (-1, 3, 2),
            (3, 1, 2),
            (1, -3, 2),
            (1, -2, -3), // -Z on top
            (-2, -1, -3),
            (-1, 2, -3),
            (2, 1, -3),
            (-3, -2, -1), // -X on top
            (-2, 3, -1),
            (3, 2, -1),
            (2, -3, -1),
            (1, 3, -2), // -Y on top
            (3, -1, -2),
            (-1, -3, -2),
            (-3, 1, -2),
        ]
        .iter()
        .map(|(a, b, c)| Self(*a, *b, *c))
        .collect()
    }
}

impl Pos {
    fn new(s: &str) -> Self {
        let (x, y, z) = s
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        Self(x, y, z)
    }

    const fn delta(&self, next: &Self) -> Self {
        Self(self.0 - next.0, self.1 - next.1, self.2 - next.2)
    }

    const fn shift(&self, shift: &Self) -> Self {
        Self(self.0 + shift.0, self.1 + shift.1, self.2 + shift.2)
    }

    const fn manhattan_distance(&self, other: &Self) -> isize {
        isize::abs(self.0 - other.0) + isize::abs(self.1 - other.1) + isize::abs(self.2 - other.2)
    }

    fn get_mapped(&self, map: isize) -> isize {
        match map {
            1 => self.0,
            -1 => -self.0,
            2 => self.1,
            -2 => -self.1,
            3 => self.2,
            -3 => -self.2,
            _ => unreachable!(),
        }
    }

    fn map(&self, mapping: &Mapping) -> Self {
        Self(
            self.get_mapped(mapping.0),
            self.get_mapped(mapping.1),
            self.get_mapped(mapping.2),
        )
    }
}

impl Scanner {
    const fn new(id: usize, beacons: HashSet<Pos>) -> Self {
        Self {
            id,
            beacons,
            mappings: None,
        }
    }

    /// Extend a scanner with additional beacons, mapping and shifting them as needed
    fn extend(&mut self, beacons: &HashSet<Pos>, mapping: &Mapping, shift: &Pos) {
        self.beacons
            .extend(beacons.iter().map(|b| b.map(mapping).shift(shift)));
    }

    /// Checks to see if the candidate scanner overlaps in any orientation
    ///
    /// Returns the mappings and shift that need to be applied to it if it does overlap
    fn overlaps(&self, candidate: &Self, required_hits: usize) -> Option<(Mapping, Pos)> {
        // Try every mapping until we find one which has enough hits
        ALL_MAPPINGS.par_iter().find_map_any(|mapping| {
            let mut hits = HashMap::new();
            // For all beacons, see what the distance is to all beacons from the candidate
            // scanner with this mapping
            for beacon in &self.beacons {
                for mapped in candidate.beacons.iter().map(|d| d.map(mapping)) {
                    *hits.entry(beacon.delta(&mapped)).or_insert(0) += 1;
                }
            }
            // If we see the same delta between points enough, then they overlap
            let (delta, max) = hits.iter().max_by_key(|(_, v)| *v).unwrap();
            if *max >= required_hits {
                return Some((mapping.clone(), delta.clone()));
            }
            None
        })
    }
}

#[aoc_generator(day19)]
fn gen(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(id, group)| {
            Scanner::new(
                id,
                group.lines().skip(1).map(|line| Pos::new(line)).collect(),
            )
        })
        .collect()
}

/// Groups the scanners returning (beacon locations, scanner locations)
fn group(input: &Vec<Scanner>) -> (HashSet<Pos>, Vec<Pos>) {
    static REQUIRED_MATCHES: usize = 12;
    let mut unplaced = HashMap::new();
    for scanner in input.clone() {
        unplaced.insert(scanner.id, scanner);
    }
    // Start off with scanner 0
    let mut placed = unplaced.remove(&0).unwrap();
    let mut scanner_positions = vec![Pos(0, 0, 0); 0];

    loop {
        // Keep looping through unplaced scanners, trying to find one that overlaps
        let mut found = false;
        let ids = unplaced.keys().copied().collect::<Vec<_>>();
        for id in &ids {
            let scanner = unplaced.get(id).unwrap();
            if let Some((mappings, delta)) = placed.overlaps(scanner, REQUIRED_MATCHES) {
                // Found a scanner that overlaps!
                // Merge it into the placed scanner
                let new = &scanner.beacons.clone();
                placed.extend(new, &mappings, &delta);
                // And record how far it was
                scanner_positions.push(delta);
                unplaced.remove(id);
                found = true;
            }
        }
        if unplaced.is_empty() {
            break;
        }
        if !found {
            // Didn't find an overlap, shouldn't happen
            panic!("Failed to find any overlapping scanners");
        }
    }
    (placed.beacons, scanner_positions)
}

#[aoc(day19, part1)]
fn part1(input: &Vec<Scanner>) -> usize {
    let (beacons, _) = group(input);
    beacons.len()
}

#[aoc(day19, part2)]
fn part2(input: &Vec<Scanner>) -> isize {
    let (_, scanners) = group(input);
    scanners
        .iter()
        .permutations(2)
        .map(|x| x[0].manhattan_distance(x[1]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    --- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 79);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 3621);
    }
}
