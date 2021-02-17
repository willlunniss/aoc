use std::collections::HashMap;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    edges: Vec<String>,
    links: Vec<Option<(usize, usize)>>,
}

impl Tile {
    pub fn is_corner(&self) -> bool {
        self.links.iter().filter(|link| link.is_some()).collect::<Vec<_>>().len() == 2
    }
}

impl FromStr for Tile {    
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {        
        // Extract the ID from the first line
        let id = s.lines().nth(0).unwrap()[5..9].parse().unwrap();
        let mut edges = vec![String::with_capacity(10); 4];

        // Read in the edges clockwise from the top left corner to normalise them ready for rotation
        // To find a matching title, we will need to find one with
        // an edge that is identical when reversed

        //        0 →
        //   L    TOP   R
        // ↑ E          I 1
        // 3 F          G ↓
        //   T          H 
        //      BOTTOM  T
        //       ← 2

        // 0 and 2 edges are the first and last lines (reversed) of the image
        edges[0] = s.lines().nth(1).unwrap().to_string();
        edges[2] = s.lines().last().unwrap().chars().rev().collect();
        // The 1 and 3 edges are the first (reversed) and last chars of each line of the image
        let mut left = Vec::new();
        let mut right = Vec::new();
        for i in 0..10 {
            let line = s.lines().nth(1 + i).unwrap();
            left.push(line.chars().nth(0).unwrap());
            right.push(line.chars().last().unwrap());
        }
        edges[1] = left.into_iter().rev().collect();
        edges[3] = right.into_iter().collect();
        return Ok(Tile { id: id, edges: edges, links: vec![None; 4] });
    }
}

/// Assemble by filling in the links between tiles with matching edges
fn assemble(tiles: &mut HashMap<usize, Tile>) {
    // We can rely on an assumption that there is at zero or one possible way to match each edge,
    // i.e. each edge is only found 1 or 2 times, to simplify the process

    // Create a map of edge -> Title id, edge index, edge reversed
    let mut edges: HashMap<String, Vec<(usize, usize, bool)>> = HashMap::new();
    // Add all edges for each tile to the map as both original and reversed
    for tile in tiles.values() {
        for (edge_index, edge) in tile.edges.iter().enumerate() {
            // Get the Vector for each edge, or a default empty one, and add the tile
            edges.entry(edge.to_string()).or_default().push((tile.id, edge_index, false));
            edges.entry(edge.chars().rev().collect::<String>()).or_default().push((tile.id, edge_index, true));
        }
    }

    // We can now build up the image by joining the tiles with matching edges
    for data in edges.values() {
        if data.len() == 1 { continue; } // Single edge (i.e. edge of image) - skip
        // Now we have ignored single edges assert that this is 2 matches for this edge
        assert_eq!(data.len(), 2, "Found more than 2 instances of the same edge in the input data");

        let (tile_id1, edge1, reversed1) = data[0];
        let (tile_id2, edge2, reversed2) = data[1];

        // Check the status of the links
        let linked1 = tiles.get(&tile_id1).unwrap().links[edge1].is_some();
        let linked2 = tiles.get(&tile_id2).unwrap().links[edge2].is_some();

        // Connect up links
        if !linked1 && !linked2 {
            tiles.get_mut(&tile_id1).unwrap().links[edge1] = Some((tile_id2, edge2));
            tiles.get_mut(&tile_id2).unwrap().links[edge2] = Some((tile_id1, edge1));
        } else if linked1 && linked2 {
            continue; // Ignore as already linked (will be the inverse pair)
        } else if linked1 {
            panic!("{}:{} is already linked, cannot link {}:{}({}) <-> {}:{}({})", tile_id1, edge1, tile_id1, edge1, reversed1, tile_id1, edge2, reversed2);
        } else if linked2 {
            panic!("{}:{} is already linked, cannot link {}:{}({}) <-> {}:{}({})", tile_id2, edge2, tile_id1, edge1, reversed1, tile_id2, edge2, reversed2);
        }
    }
}


#[aoc_generator(day20)]
pub fn gen(input: &str) -> Vec<Tile> {
    input.split("\r\n\r\n").map(|title| title.parse().unwrap()).collect()
}

#[aoc(day20, part1)]
fn part1(input: &Vec<Tile>) -> usize {
    let mut tiles : HashMap<usize, Tile> = input.iter().map(|tile| (tile.id, tile.clone())).collect();
    // Assemble the jigsaw by linking up all the edges
    assemble(&mut tiles);

    // Find just the corners and return the multiple of their ids
    return tiles.iter().filter(|(_, tile)| tile.is_corner()).map(|(id, _)| *id).collect::<Vec<usize>>().iter().fold(1, | acc, id| acc * id);
}

#[aoc(day20, part2)]
fn part2(input: &Vec<Tile>) -> usize {
    
    return 0;
}
