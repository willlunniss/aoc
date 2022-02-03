use std::collections::{HashSet, VecDeque};

/// Determines if an IP address supports TLS (transport-layer snooping)
fn supports_tls(address: &str) -> bool {
    let mut has_abba = false;
    let mut in_hypernet = false;
    // Go through each char keeping a rolling history of the last 4
    let mut hist = VecDeque::from(['?', '?', '?', '?']);
    for c in address.chars() {
        hist.rotate_right(1);
        hist[0] = c;
        match c {
            '[' => in_hypernet = true,
            ']' => in_hypernet = false,
            _ => {
                // Look for an ABBA style sequence that isn't inside a hypernet sequence
                if hist[0] == hist[3] && hist[1] == hist[2] && hist[0] != hist[1] {
                    if in_hypernet {
                        return false;
                    }
                    has_abba = true;
                }
            }
        }
    }
    has_abba
}

/// Determines if an IP address supports SSL (super-secret listening)
fn supports_ssl(address: &str) -> bool {
    let mut in_hypernet = false;
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    // Go through each char keeping a rolling history of the last 3
    let mut hist = VecDeque::from(['?', '?', '?']);
    for c in address.chars() {
        hist.rotate_right(1);
        hist[0] = c;
        match c {
            '[' => in_hypernet = true,
            ']' => in_hypernet = false,
            _ => {
                // Look for an ABA style sequence
                if hist[0] == hist[2] && hist[0] != hist[1] {
                    // Store the first two chars (AB/BA)
                    if in_hypernet {
                        // Swap them round for BABs to always give in AB form
                        babs.insert((hist[1], hist[0]));
                    } else {
                        abas.insert((hist[0], hist[1]));
                    }
                }
            }
        }
    }
    // Supports SSL if there is a matching ABA / BAB pair
    abas.iter().any(|pair| babs.contains(pair))
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|address| supports_tls(address))
        .count()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|address| supports_ssl(address))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
        assert!(!supports_tls("abba[aaabba]asb"));
        assert!(supports_tls("abba[aababa]asb"));
    }

    #[test]
    fn test_part2_example() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
