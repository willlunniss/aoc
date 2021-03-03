use std::collections::HashMap;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    data: Vec<Vec<char>>,
    edges: Vec<String>,
    links: Vec<Option<(usize, usize, bool)>>,
}

impl Tile {
    /// Checks if a Tile is a corner (has 2/4 connected edges)
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

        for line in s.lines().skip(1) {
            left.push(line.chars().nth(0).unwrap());
            right.push(line.chars().last().unwrap());
        }
        edges[3] = left.into_iter().rev().collect();
        edges[1] = right.into_iter().collect();

        // And then finally read in the actual image data (by discarding edges)
        let tile_size = 8;
        let data = s.lines().skip(1+1).take(tile_size).map(|line| line.chars().skip(1).take(tile_size).collect::<Vec<char>>()).collect();

        return Ok(Tile { id: id, data: data, edges: edges, links: vec![None; 4] });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileLayout {
    id: usize,
    orientation: usize,
    flipped: bool
}

impl TileLayout {
    fn required_rotation(&self) -> usize {
        match &self.orientation {
            0 => 0,
            1 => 3,
            2 => 2,
            3 => 1,
            _ => panic!("Unexpected orientation")
        }
    }
}

/// Rotates a 2d nested vector 90 degrees clockwise
fn rotate<T>(data: &mut Vec<Vec<T>>) {
    data.reverse();
    for i in 1..data.len() {
        let (left, right) = data.split_at_mut(i);
        for (j, left_item) in left.iter_mut().enumerate().take(i) {
            std::mem::swap(&mut left_item[i], &mut right[0][j]);
        }
    }
}

fn flip<T>(data: &mut Vec<Vec<T>>) {
    for row in data.iter_mut() {
        row.reverse();
    }
}

/// Connects up tiles by filling in the links between tiles with matching edges
fn link_edges(tiles: &mut HashMap<usize, Tile>) {
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
            tiles.get_mut(&tile_id1).unwrap().links[edge1] = Some((tile_id2, edge2, reversed2));
            tiles.get_mut(&tile_id2).unwrap().links[edge2] = Some((tile_id1, edge1, reversed1));
        } else if linked1 && linked2 {
            continue; // Ignore as already linked (will be the inverse pair)
        } else if linked1 {
            panic!("{}:{} is already linked, cannot link {}:{}({}) <-> {}:{}({})", tile_id1, edge1, tile_id1, edge1, reversed1, tile_id1, edge2, reversed2);
        } else if linked2 {
            panic!("{}:{} is already linked, cannot link {}:{}({}) <-> {}:{}({})", tile_id2, edge2, tile_id1, edge1, reversed1, tile_id2, edge2, reversed2);
        }
    }
}

/// Lays out the tiles on a grid returning a 2d vector of tile layout info
pub fn layout_grid(tiles: &HashMap<usize, Tile>) -> Vec<Vec<TileLayout>> {
    let grid_size = f64::sqrt(tiles.len() as f64) as usize;
    let mut grid = vec![vec![None; grid_size]; grid_size];

    // Start from the top left corner tile
    // We therefore start with everything orientated normally to start (up is 0 and not flipped)
    // Note: When calculating a direction from another one e.g. up -> left we always use + x %4 to avoid it going negative
    let mut up = 0;
    let mut right = 1;
    let mut down = 2;
    let mut left = 3;
    let mut flipped = false;
    let mut row = 0;

    let corners = tiles.iter().filter(|(_, tile)| tile.is_corner()).collect::<HashMap<_,_>>();
    let mut top_left = corners.iter().filter(|(_, tile)| tile.links[0].is_none() && tile.links[3].is_none()).collect::<HashMap<_,_>>();
    if top_left.is_empty() {
        // Can't find the one we want
        // Try again assuming it is flipped
        top_left = corners.iter().filter(|(_, tile)| tile.links[0].is_none() && tile.links[1].is_none()).collect::<HashMap<_,_>>();
        flipped ^= true;
        right = (right + 2) % 4;
        left = (left + 2) % 4;
    }
    let mut tile = tiles.get(top_left.keys().next().unwrap()).unwrap();

    // TODO: Re-write as a single loop that switches modes to turn around when it hits an edge

    loop {
        // Save the id of the first tile in the row so we can jump back when we hit the end
        let row_start = tile.id;
        let row_start_flipped = flipped;
        let row_start_down = down;
        let mut coll = 0;
        loop {
            // Save the tile details to the grid
            //println!("Located {} {:?} {} {}{}{}{}", tile.id, tile.links, flipped, up, right, down, left);
            grid[row][coll] = Some(TileLayout {id: tile.id, orientation: up, flipped: flipped});
            if row == 0 {
                assert_eq!(tile.links[up], None);
            } else {
                assert_eq!(tile.links[up].unwrap().0, grid[row - 1][coll].unwrap().id);
            }
            if tile.links[right].is_none() {
                break; // End of the row
            }
            coll += 1;
            // Advance right to the next tile
            let (next_id, edge, reversed) = tile.links[right].unwrap();
            //println!("{} <-> {}", right, edge);
            tile = tiles.get(&next_id.clone()).unwrap();
            // Then work out where we can go next after that
            // Next right will be by going to the link opposite the edge that was connected
            left = edge;
            right = (edge + 2) % 4;
            // We normally expect to match one edge with the reversed form of it
            // If they are both reversed, or neither reversed, then we have had to flip a tile over
            if tile.links[left].unwrap().2 == reversed {
                // Yes we did, invert flipped state
                flipped ^= true;
            }
            up = (if flipped {right} else {left} + 1) % 4;
            down = (up + 2) % 4;
        }
        // Reset to start of the row
        tile = tiles.get(&row_start).unwrap();
        down = row_start_down;
        flipped = row_start_flipped;
        if tile.links[down].is_none() {
            // Can't go down any further, must have been the last row
            break;
        }
        // Then advance down one
        row += 1;
        let (next_id, edge, reversed) = tile.links[down].unwrap();
        //println!("{} <-DOWN-> {}", down, edge);
        tile = tiles.get(&next_id.clone()).unwrap();
        // Next down will be the link opposite the edge that we used to go down to this tile
        up = edge;
        down = (edge + 2) % 4;
        // We normally expect to match one edge with the reversed form of it
        // If they are both reversed, or neither reversed, then we have had to flip a tile over
        if tile.links[up].unwrap().2 == reversed {
            // Yes we did, invert flipped state
            flipped ^= true;
        }
        // Next right is a bit more complicated due to tiles potentially being reversed
        right = (if flipped {down} else {up} + 1) % 4;
        left = (right + 2) % 4;
        //println!("Moved down to {} {:?} {} {}{}{}{}", tile.id, tile.links, flipped, up, right, down, left);
        assert_eq!(tile.links[left], None);
    }
    return grid.iter().map(|row| row.iter().map(|cell| cell.unwrap().clone()).collect()).collect();
}

/// Uses tile layout information to assembles the tiles into a complete image
fn build_image(tiles: &HashMap<usize, Tile>, layout: &Vec<Vec<TileLayout>>) -> Vec<Vec<char>> {
    let tile_size = tiles.get(&layout[0][0].id).unwrap().data.len();
    let image_size = layout.len() *  tile_size;
    let mut image : Vec<Vec<char>> = vec![vec!['?'; image_size]; image_size];

    for (y, row) in layout.iter().enumerate() {
        for (x, tl) in row.iter().enumerate() {
            let mut data = tiles.get(&tl.id).unwrap().data.clone();
            // Rotate the data in 90 degree steps
            // TODO: Implement direct 180/270 rotation
            for _ in 0..tl.required_rotation() {
                rotate(&mut data);
            }
            // If needed, flip it
            if tl.flipped {
                flip(&mut data);
            }
            // Work out where we need to insert the image data
            let y_offset = y * tile_size;
            let x_offset = x * tile_size;
            // Add it to the image
            for j in 0..tile_size {
                for i in 0..tile_size {
                    image[y_offset + j][x_offset + i] = data[j][i];
                }
            }
        }
    }
    return image;
}

fn build_sea_monster_markers() -> Vec<(usize, usize)> {    
    let monster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ".to_owned();
    // Turn the monster pattern into a series of marker coordinates to check
    let mut markers = Vec::new();
    for (y, line) in monster.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                markers.push((y, x));
            }
        }
    }
    return markers;
}

/// Finds sea monsters in an image by searching for supplied # markers
fn find_sea_monsters(image: &Vec<Vec<char>>, markers: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let width = markers.iter().map(|(_, x)| x).max().unwrap();
    let height = markers.iter().map(|(y, _)| y).max().unwrap();
    let mut monsters = Vec::new();
    for y_base in 0..image.len() - height {
        for x_base in 0..image.len() - width {
            // For each possible starting position (monsters can't wrap around the image)
            let mut found = true;
            for point in markers.iter() {
                // Check to see if any of the points don't look like a potential monster
                if image[y_base + point.0][x_base + point.1] != '#' {
                    // No monster here, abandon search
                    found = false;
                    break;
                }
            }
            // All points matched, there is a monster here!
            if found {
                monsters.push((y_base, x_base));
            }
        }
    }
    return monsters;
}

#[allow(dead_code)]
/// Highlights sea monsters in an image by setting them to O for debugging
fn highlight_sea_monsters(image: &mut Vec<Vec<char>>, markers: &Vec<(usize, usize)>, monsters: &Vec<(usize, usize)> ){
    for monster in monsters {
        for point in markers.iter() {
            image[monster.0 + point.0][monster.1 + point.1] = 'O';
        }
    }
}

#[allow(dead_code)]
/// Prints the image with tiles separated by spaces for debugging
fn print_image(image: &Vec<Vec<char>>) {
    for (y, row) in image.iter().enumerate() {
        if y % 8 == 0 {
            println!("{: <1$}", "", row.len() + (row.len()/8));
        }
        for (x, cell) in row.iter().enumerate() {
            if x % 8 == 0 {
                print!(" ");
            }
            print!("{}", cell);
        }
        println!();
    }
}

#[aoc_generator(day20)]
pub fn gen(input: &str) -> Vec<Tile> {
    input.split("\r\n\r\n").map(|tile| tile.parse().unwrap()).collect()
}

#[aoc(day20, part1)]
fn part1(input: &Vec<Tile>) -> usize {
    let mut tiles : HashMap<usize, Tile> = input.iter().map(|tile| (tile.id, tile.clone())).collect();
    // Link up all the edges of tiles
    link_edges(&mut tiles);

    // Find just the corners and return the multiple of their ids
    let corners = tiles.iter().filter(|(_, tile)| tile.is_corner()).collect::<HashMap<_,_>>();
    assert_eq!(corners.len(), 4, "Expected to have 4 corner tiles");
    return corners.iter().map(|(id, _)| **id).collect::<Vec<usize>>().iter().fold(1, | acc, id| acc * id);
}

#[aoc(day20, part2)]
fn part2(input: &Vec<Tile>) -> usize {
    let mut tiles : HashMap<usize, Tile> = input.iter().map(|tile| (tile.id, tile.clone())).collect();
    // Link up all the edges of tiles
    link_edges(&mut tiles);

    // Layout the tiles on a grid
    let layout = layout_grid(&tiles);

    // Use the data for each tile to build the completed image
    let mut image  = build_image(&tiles, &layout);
    let wave_tiles = image.iter().flat_map(|row| row.iter().filter(|c| **c == '#')).collect::<Vec<_>>().len();

    // Build the search pattern we need to find monsters
    let markers = build_sea_monster_markers();

    // We finally have an image and can search for sea monsters!
    // We have to search in 8 modes (4 rotations flipped and not flipped)
    // until we find them
    let mut monsters = Vec::new();
    for mode in 0..8 {
        monsters = find_sea_monsters(&image, &markers);
        if !monsters.is_empty() {
            // Found the monsters!
            break;
        }

        // Didn't find any, move on to next mode
        rotate(&mut image);
        // After going through all rotations flip
        if mode == 3 {
            flip(&mut image);
        }
    }

    // Final solution is number of # - (monsters * monster tiles)
    return wave_tiles - (monsters.len() * markers.len());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_str() {
        let tile : Tile = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###".parse().unwrap();
        
        assert_eq!(tile.id, 2311);
        assert_eq!(tile.edges[0], "..##.#..#.");
        assert_eq!(tile.edges[1], "...#.##..#");
        assert_eq!(tile.edges[2], "###..###..");
        assert_eq!(tile.edges[3], ".#..#####.");

        assert_eq!(tile.data.concat().into_iter().collect::<String>(), "#..#.......##..####.#...#.##.####...#.###.#.#..#.#....#.##...#.#");
    }
}


