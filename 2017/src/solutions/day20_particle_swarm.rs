use itertools::Itertools;

#[derive(Debug, Clone)]
struct Particle {
    id: usize,
    p: [isize; 3],
    v: [isize; 3],
    a: [isize; 3],
}

impl Particle {
    fn new(id: usize, s: &str) -> Self {
        // p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
        let (p, v, a) = s
            .split(", ")
            .map(|coord| {
                coord[3..coord.len() - 1]
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Self { id, p, v, a }
    }

    /// Steps the `Particle` forward in time
    const fn step(&self) -> Self {
        // Calculate new velocity based on previous velocity and acceleration
        let v = [
            self.v[0] + self.a[0],
            self.v[1] + self.a[1],
            self.v[2] + self.a[2],
        ];
        // Calculate new position based on previous position and new velocity
        let p = [self.p[0] + v[0], self.p[1] + v[1], self.p[2] + v[2]];

        // Return new particle values
        Self {
            id: self.id,
            p,
            v,
            a: self.a,
        }
    }
}

#[aoc_generator(day20)]
fn gen(input: &str) -> Vec<Particle> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| Particle::new(id, line))
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &Vec<Particle>) -> usize {
    let mut particles = input.clone();
    for _ in 0..1000 {
        particles = particles.iter().map(Particle::step).collect();
    }
    // Return the id of the particle with the minimum position
    particles
        .iter()
        .min_by_key(|part| part.p.iter().map(|n| isize::abs(*n)).sum::<isize>())
        .unwrap()
        .id
}

#[aoc(day20, part2)]
fn part2(input: &Vec<Particle>) -> usize {
    let mut particles = input.clone();
    for _ in 0..1000 {
        // Count particles at each position
        let counts = particles.iter().map(|p| p.p).counts();
        // Keeping only particles which haven't collided, step forward
        particles = particles
            .iter()
            .filter(|p| *counts.get(&p.p).unwrap() == 1)
            .map(Particle::step)
            .collect();
    }
    particles.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_P1: &str = indoc!(
        "
        p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
        p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
        "
    );

    static EXAMPLE_INPUT_P2: &str = indoc!(
        "
        p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
        p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
        p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
        p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT_P1)), 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT_P2)), 1);
    }
}
