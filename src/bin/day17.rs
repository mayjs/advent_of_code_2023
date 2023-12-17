use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    path::Path,
};

use advent_of_code_2023::read_lines;
use anyhow::Result;

const INPUT: &str = "input/day17.txt";

struct HeatLossMap(Vec<Vec<u32>>);

impl HeatLossMap {
    fn from_input<P: AsRef<Path>>(input: P) -> Result<Self> {
        Ok(Self(
            read_lines(input)?
                .map(|l| {
                    l.unwrap()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect(),
        ))
    }

    fn dims(&self) -> (usize, usize) {
        let height = self.0.len();
        let width = self.0[0].len();

        (height, width)
    }

    fn get(&self, coords: &(usize, usize)) -> u32 {
        self.0[coords.0][coords.1]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MovementState {
    Horizontal(usize, bool), // The first attribute is the number of fields we already moved in
    // this direction. If the bool is true, we are going right, else left.
    Vertical(usize, bool), // The first attribute is the number of fields we already moved in
                           // this direction. If the bool is true, we are going down, else right.
}

const MOVEMENT_LIMIT: usize = 3;
const ULTRA_MOVEMENT_LIMIT: usize = 10;
const ULTRA_MIN_MOVEMENT: usize = 4;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    coords: (usize, usize),
    state: MovementState,
}

impl Node {
    fn new(coords: (usize, usize), state: MovementState) -> Self {
        Self { coords, state }
    }

    fn start() -> Vec<Self> {
        vec![
            Self::new((0, 1), MovementState::Horizontal(1, true)),
            Self::new((1, 0), MovementState::Vertical(1, true)),
        ]
    }

    fn neighbors(&self, dims: &(usize, usize)) -> Vec<Self> {
        match self.state {
            MovementState::Horizontal(c, right) => {
                let mut result = [
                    self.coords.0.checked_sub(1).map(|ny| {
                        Self::new((ny, self.coords.1), MovementState::Vertical(1, false))
                    }),
                    self.coords
                        .0
                        .checked_add(1)
                        .and_then(|ny| if ny < dims.0 { Some(ny) } else { None })
                        .map(|ny| Self::new((ny, self.coords.1), MovementState::Vertical(1, true))),
                ]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>();

                if c < MOVEMENT_LIMIT {
                    if right {
                        result.extend(
                            self.coords
                                .1
                                .checked_add(1)
                                .and_then(|nx| if nx < dims.1 { Some(nx) } else { None })
                                .map(|nx| {
                                    Self::new(
                                        (self.coords.0, nx),
                                        MovementState::Horizontal(c + 1, true),
                                    )
                                }),
                        );
                    } else {
                        result.extend(self.coords.1.checked_sub(1).map(|nx| {
                            Self::new((self.coords.0, nx), MovementState::Horizontal(c + 1, false))
                        }));
                    }
                }

                result
            }
            MovementState::Vertical(c, down) => {
                let mut result = [
                    self.coords.1.checked_sub(1).map(|nx| {
                        Self::new((self.coords.0, nx), MovementState::Horizontal(1, false))
                    }),
                    self.coords
                        .1
                        .checked_add(1)
                        .and_then(|nx| if nx < dims.1 { Some(nx) } else { None })
                        .map(|nx| {
                            Self::new((self.coords.0, nx), MovementState::Horizontal(1, true))
                        }),
                ]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>();

                if c < MOVEMENT_LIMIT {
                    if !down {
                        result.extend(self.coords.0.checked_sub(1).map(|ny| {
                            Self::new((ny, self.coords.1), MovementState::Vertical(c + 1, down))
                        }));
                    } else {
                        result.extend(
                            self.coords
                                .0
                                .checked_add(1)
                                .and_then(|ny| if ny < dims.0 { Some(ny) } else { None })
                                .map(|ny| {
                                    Self::new(
                                        (ny, self.coords.1),
                                        MovementState::Vertical(c + 1, down),
                                    )
                                }),
                        );
                    }
                }

                result
            }
        }
    }

    fn ultra_neighbors(&self, dims: &(usize, usize)) -> Vec<Self> {
        match self.state {
            MovementState::Horizontal(c, right) => {
                let mut result = [
                    self.coords.0.checked_sub(1).map(|ny| {
                        Self::new((ny, self.coords.1), MovementState::Vertical(1, false))
                    }),
                    self.coords
                        .0
                        .checked_add(1)
                        .and_then(|ny| if ny < dims.0 { Some(ny) } else { None })
                        .map(|ny| Self::new((ny, self.coords.1), MovementState::Vertical(1, true))),
                ]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>();

                if c < ULTRA_MIN_MOVEMENT {
                    result.clear();
                }

                if c < ULTRA_MOVEMENT_LIMIT {
                    if right {
                        result.extend(
                            self.coords
                                .1
                                .checked_add(1)
                                .and_then(|nx| if nx < dims.1 { Some(nx) } else { None })
                                .map(|nx| {
                                    Self::new(
                                        (self.coords.0, nx),
                                        MovementState::Horizontal(c + 1, right),
                                    )
                                }),
                        );
                    } else {
                        result.extend(self.coords.1.checked_sub(1).map(|nx| {
                            Self::new((self.coords.0, nx), MovementState::Horizontal(c + 1, right))
                        }));
                    }
                }

                result
            }
            MovementState::Vertical(c, down) => {
                let mut result = [
                    self.coords.1.checked_sub(1).map(|nx| {
                        Self::new((self.coords.0, nx), MovementState::Horizontal(1, false))
                    }),
                    self.coords
                        .1
                        .checked_add(1)
                        .and_then(|nx| if nx < dims.1 { Some(nx) } else { None })
                        .map(|nx| {
                            Self::new((self.coords.0, nx), MovementState::Horizontal(1, true))
                        }),
                ]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>();
                if c < ULTRA_MIN_MOVEMENT {
                    result.clear();
                }

                if c < ULTRA_MOVEMENT_LIMIT {
                    if !down {
                        result.extend(self.coords.0.checked_sub(1).map(|ny| {
                            Self::new((ny, self.coords.1), MovementState::Vertical(c + 1, down))
                        }));
                    } else {
                        result.extend(
                            self.coords
                                .0
                                .checked_add(1)
                                .and_then(|ny| if ny < dims.0 { Some(ny) } else { None })
                                .map(|ny| {
                                    Self::new(
                                        (ny, self.coords.1),
                                        MovementState::Vertical(c + 1, down),
                                    )
                                }),
                        );
                    }
                }

                result
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NodeEntry(u32, Node);

impl PartialOrd for NodeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

fn find_shortest_path(map: &HeatLossMap, ultra: bool) -> (u32, Vec<(usize, usize)>) {
    let dims = map.dims();
    let mut nodes_to_investigate: BinaryHeap<NodeEntry> = Node::start()
        .into_iter()
        .map(|node| NodeEntry(map.get(&node.coords), node))
        .collect();
    let mut visited_nodes = HashSet::<Node>::new();
    let mut distances = HashMap::<Node, u32>::new();
    nodes_to_investigate.iter().for_each(|n| {
        distances.insert(n.1.clone(), n.0);
    });
    let mut prev = HashMap::<Node, Node>::new();

    loop {
        let NodeEntry(cur_heatloss, cur_node) = nodes_to_investigate.pop().unwrap();

        if cur_node.coords.0 == dims.0 - 1 && cur_node.coords.1 == dims.1 - 1 {
            let path = std::iter::successors(Some(cur_node), |node| prev.get(&node).cloned())
                .map(|n| n.coords)
                .collect::<Vec<_>>();
            return (cur_heatloss, path);
        }

        let neighbors = if !ultra {
            cur_node.neighbors(&dims)
        } else {
            cur_node.ultra_neighbors(&dims)
        };

        let updates: Vec<_> = neighbors
            .into_iter()
            .filter(|n| !visited_nodes.contains(n))
            .map(|n| (cur_heatloss + map.get(&n.coords), n))
            .filter(|(heatloss, node)| distances.get(node).map(|d| heatloss < d).unwrap_or(true))
            .collect();
        for (heatloss, node) in updates {
            distances.insert(node.clone(), heatloss);
            prev.insert(node.clone(), cur_node.clone());
            nodes_to_investigate.push(NodeEntry(heatloss, node));
        }
        visited_nodes.insert(cur_node);
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u32> {
    let map = HeatLossMap::from_input(input)?;
    let (heatloss, _path) = find_shortest_path(&map, false);
    // let mut renderer = GridRenderer::new();
    // for (y, x) in path {
    //     renderer.add_grid_tile(y, x);
    // }
    // renderer.store_svg("debug.svg");
    Ok(heatloss)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<u32> {
    let map = HeatLossMap::from_input(input)?;
    let (heatloss, _path) = find_shortest_path(&map, true);
    // let mut renderer = GridRenderer::new();
    // for (y, x) in path {
    //     renderer.add_grid_tile(y, x);
    // }
    // renderer.store_svg("debug2.svg");
    Ok(heatloss)
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day17 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {r"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 102);
        assert_eq!(part2(&file).unwrap(), 94);
        drop(dir);
    }
}
