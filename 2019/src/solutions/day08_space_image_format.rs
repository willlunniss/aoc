use utils::ocr::OcrString;

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<usize> {
    return input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
}

#[aoc(day8, part1)]
fn part1(input: &[usize]) -> usize {
    let layer_size = 25 * 6;
    let mut lowest_0_count = layer_size; // Assume all pixels are 0 as the max
    let mut result = 0;
    for layer in input.chunks(layer_size) {
        // For each layer, count how many pixels are 0
        // If less than what we have seen before, save # 1s * #2s
        let mut count = vec![0; 3];
        layer.iter().for_each(|pixel| count[*pixel] += 1);
        if count[0] < lowest_0_count {
            lowest_0_count = count[0];
            result = count[1] * count[2];
        }
    }
    result
}

fn build_image(input: &[usize], width: usize, height: usize) -> Vec<usize> {
    // Initialise an all transparent image to start
    let mut image = vec![2; width * height];
    for id in 0..width * height {
        // Process each pixel individually
        for layer in input.chunks(width * height) {
            // Go through the layers until we find a black or white pixel
            match layer[id] {
                0 | 1 => {
                    // Black or white - set the pixel state and then move on to the next
                    image[id] = layer[id];
                    break;
                }
                _ => {} // Transparent, try the next layer
            }
        }
    }
    image
}

#[aoc(day8, part2)]
fn part2(input: &[usize]) -> String {
    let width = 25;
    let height = 6;
    // Build the image
    let image = build_image(input, width, height);
    // Decode the text
    // Convert to list of points where text is
    image
        .chunks(width)
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &pixel)| {
                    if pixel == 1 {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<OcrString>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_image() {
        assert_eq!(
            build_image(&(gen(&"0222112222120000".to_owned())), 2, 2),
            [0, 1, 1, 0]
        );
    }
}
