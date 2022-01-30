use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type ComponentId = u64;
type Pins = usize;

#[derive(Debug, Clone)]
struct Component {
    id: ComponentId,
    pins1: Pins,
    pins2: Pins,
}

impl Component {
    fn new(id: ComponentId, s: &str) -> Self {
        let pins: (Pins, Pins) = s
            .split('/')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self {
            id,
            pins1: pins.0,
            pins2: pins.1,
        }
    }

    const fn pins(&self) -> [Pins; 2] {
        [self.pins1, self.pins2]
    }

    const fn strength(&self) -> usize {
        self.pins1 + self.pins2
    }

    /// Given the pins at one end, returns the pins at the other end
    fn other_end(&self, pins: Pins) -> Pins {
        if pins == self.pins1 {
            self.pins2
        } else if pins == self.pins2 {
            self.pins1
        } else {
            panic!("{:?} does not have {} pins at either end", self, pins)
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Bridge {
    length: usize,
    strength: usize,
}

impl Bridge {
    /// Joins a `Component` to a `Bridge`
    const fn join(&self, component: &Component) -> Self {
        Self {
            length: self.length + 1,
            strength: self.strength + component.strength(),
        }
    }
}

#[aoc_generator(day24)]
fn gen(
    input: &str,
) -> (
    HashMap<ComponentId, Component>,
    HashMap<Pins, HashSet<ComponentId>>,
) {
    // Map to look up which components could be used to connect to a port with a given number of pins
    let mut pins_to_ids: HashMap<Pins, HashSet<ComponentId>> = HashMap::new();
    // Map of all components
    // Assign each component a unique id as a bit field in a u64 (OK as there are less than 64 components)
    // This makes it trivial to compare available components by summing up all ids
    let mut components: HashMap<ComponentId, Component> = HashMap::new();
    for component in input
        .lines()
        .enumerate()
        .map(|(idx, line)| Component::new(1 << idx, line))
    {
        for pins in component.pins() {
            (*pins_to_ids.entry(pins).or_default()).insert(component.id);
        }
        components.insert(component.id, component);
    }
    (components, pins_to_ids)
}

/// Recursively builds the best bridge possible from the available components that will connect
/// to the port based on it's number of pins using the supplied comparison function
fn build_bridge<F>(
    cache: &mut HashMap<u64, Bridge>,
    components: &HashMap<ComponentId, Component>,
    pins_to_ids: &HashMap<Pins, HashSet<ComponentId>>,
    port_pins: Pins,
    compare: F,
) -> Bridge
where
    F: Copy + Fn(&Bridge, &Bridge) -> Ordering,
{
    // See if we have already built a bridge with these components
    // NOTE: Can't use cache.entry().or_insert_with() as we need to pass cache into the recursive call
    let available_components = components.keys().sum::<u64>();
    if let Some(bridge) = cache.get(&available_components) {
        return bridge.clone();
    }

    let bridge = pins_to_ids
        .get(&port_pins)
        .unwrap()
        .iter()
        .map(|id| {
            // For all remaining components that have the target number of pins at one end
            let mut components = components.clone();
            // Remove that component from consideration
            let component = components.remove(id).unwrap();
            let mut pins_to_ids = pins_to_ids.clone();
            for pins in component.pins() {
                pins_to_ids.get_mut(&pins).unwrap().remove(id);
            }
            // and then join it onto the best bridge we can build with what is left
            build_bridge(
                cache,
                &components,
                &pins_to_ids,
                component.other_end(port_pins),
                compare,
            )
            .join(&component)
        })
        .max_by(|x, y| compare(x, y)) // Picking the best bridge based on the supplied comparison
        .unwrap_or_default();
    cache.insert(available_components, bridge.clone());
    bridge
}

#[aoc(day24, part1)]
fn part1(
    input: &(
        HashMap<ComponentId, Component>,
        HashMap<Pins, HashSet<ComponentId>>,
    ),
) -> usize {
    // Build the strongest bridge
    build_bridge(&mut HashMap::new(), &input.0, &input.1, 0, |x, y| {
        x.strength.cmp(&y.strength)
    })
    .strength
}

#[aoc(day24, part2)]
fn part2(
    input: &(
        HashMap<ComponentId, Component>,
        HashMap<Pins, HashSet<ComponentId>>,
    ),
) -> usize {
    // Build the longest bridge, if there is a tie pick the strongest
    build_bridge(&mut HashMap::new(), &input.0, &input.1, 0, |x, y| {
        let length = x.length.cmp(&y.length);
        if length == Ordering::Equal {
            x.strength.cmp(&y.strength)
        } else {
            length
        }
    })
    .strength
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
        0/2
        2/2
        2/3
        3/4
        3/5
        0/1
        10/1
        9/10
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 31);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 19);
    }
}
