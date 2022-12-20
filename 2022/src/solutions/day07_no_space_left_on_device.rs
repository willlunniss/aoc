use std::{collections::HashMap, path::PathBuf, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Item {
    File(PathBuf, usize),
    Directory(PathBuf),
}

#[derive(Debug, Clone)]
struct FileSystem {
    contents: HashMap<PathBuf, Vec<Item>>,
}

impl FromStr for FileSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut contents = HashMap::new();
        let mut directory = PathBuf::from(r"/");

        // Create an iterator over lines
        let mut lines = s.lines();
        let mut line = lines.next();
        while let Some(command) = line {
            let parts = command.split_ascii_whitespace().collect::<Vec<_>>();
            assert_eq!(parts[0], "$", "Should always be a command");
            match parts[1] {
                "cd" => {
                    // Change to directory
                    match parts[2] {
                        "/" => {
                            directory = PathBuf::from(r"/");
                        }
                        ".." => {
                            directory.pop();
                        }
                        _ => {
                            directory = directory.join(PathBuf::from(parts[2]));
                        }
                    };
                    line = lines.next(); // Advance to next command
                }
                "ls" => {
                    // List contents
                    line = lines.next();
                    let mut items = Vec::new();
                    while let Some(item) = line {
                        let parts = item.split_ascii_whitespace().collect::<Vec<_>>();
                        match parts[0] {
                            "$" => {
                                break; // End of ls output - do NOT advance to the next line
                            }
                            "dir" => items.push(Item::Directory(PathBuf::from(parts[1]))),
                            _ => items.push(Item::File(
                                PathBuf::from(parts[1]),
                                parts[0].parse().unwrap(),
                            )),
                        }
                        line = lines.next(); // Advance to next line
                    }
                    // Record items in our file system
                    contents.insert(directory.clone(), items);
                }
                _ => unreachable!(),
            }
        }
        Ok(Self { contents })
    }
}

impl FileSystem {
    /// Recursively computes the size of a path
    fn size(&self, path: &PathBuf) -> usize {
        self.contents
            .get(path)
            .unwrap()
            .iter()
            .map(|item| match item {
                Item::File(_, size) => *size,
                Item::Directory(name) => self.size(&path.join(name)),
            })
            .sum()
    }
}

#[aoc_generator(day7)]
fn gen(input: &str) -> FileSystem {
    input.parse().unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &FileSystem) -> usize {
    input // Return the sum of all directories which are no bigger than 100000
        .contents
        .keys()
        .map(|path| input.size(path))
        .filter(|size| *size <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &FileSystem) -> usize {
    // Work out how much space needs to be freed based on what can be used
    let used = input.size(&PathBuf::from(r"/"));
    // Can use total - required for the update
    let can_use = 70_000_000 - 30_000_000;
    let to_free = used - can_use;
    // Find the smallest directory that will free up at least enough space
    input
        .contents
        .keys()
        .map(|path| input.size(path))
        .sorted()
        .find(|size| *size >= to_free)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 95437);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 24_933_642);
    }
}
