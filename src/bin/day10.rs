use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use advent_of_code_2023::read_lines;
use anyhow::{bail, Result};
use itertools::Itertools;
use petgraph::{graphmap::DiGraphMap, Direction};

const INPUT: &str = "input/day10.txt";

type PipeGraph = DiGraphMap<(usize, usize), ()>;
struct PipeInfo {
    graph: PipeGraph,
    start: (usize, usize),
    kinds: HashMap<(usize, usize), char>,
}

impl PipeInfo {
    fn read_input<P>(input: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut kinds = HashMap::new();
        let mut start = None;
        let mut graph = PipeGraph::from_edges(
            read_lines(input)?
                .enumerate()
                .map(|(i, v)| (i + 1, v))
                .flat_map(|(y, maybe_line)| {
                    let line = maybe_line.unwrap();
                    line.chars()
                        .enumerate()
                        .map(|(i, v)| (i + 1, v))
                        .flat_map(|(x, sym)| {
                            if sym != '.' && sym != 'S' {
                                kinds.insert((x, y), sym);
                            }
                            /*
                               | is a vertical pipe connecting north and south.
                               - is a horizontal pipe connecting east and west.
                               L is a 90-degree bend connecting north and east.
                               J is a 90-degree bend connecting north and west.
                               7 is a 90-degree bend connecting south and west.
                               F is a 90-degree bend connecting south and east.
                               . is ground; there is no pipe in this tile.
                               S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                            */
                            match sym {
                                '|' => vec![((x, y), (x, y + 1)), ((x, y), (x, y - 1))],
                                '-' => vec![((x, y), (x + 1, y)), ((x, y), (x - 1, y))],
                                'L' => vec![((x, y), (x + 1, y)), ((x, y), (x, y - 1))],
                                'J' => vec![((x, y), (x - 1, y)), ((x, y), (x, y - 1))],
                                '7' => vec![((x, y), (x - 1, y)), ((x, y), (x, y + 1))],
                                'F' => vec![((x, y), (x + 1, y)), ((x, y), (x, y + 1))],
                                '.' => vec![],
                                'S' => {
                                    start = Some((x, y));
                                    vec![]
                                }
                                _ => panic!(),
                            }
                        })
                        .collect::<Vec<_>>()
                }),
        );

        let start = start.unwrap();

        // We need to "patch" the start by adding inverted edges for all incoming edges
        let edges_to_insert = graph
            .edges_directed(start, Direction::Incoming)
            .map(|(from, to, _)| (to, from))
            .collect::<Vec<_>>();
        for (from, to) in edges_to_insert.iter() {
            graph.add_edge(*from, *to, ());
        }

        if edges_to_insert.contains(&(start, (start.0, start.1 - 1)))
            && edges_to_insert.contains(&(start, (start.0, start.1 + 1)))
        {
            kinds.insert(start, '|');
        } else if edges_to_insert.contains(&(start, (start.0 - 1, start.1)))
            && edges_to_insert.contains(&(start, (start.0 + 1, start.1)))
        {
            kinds.insert(start, '-');
        } else if edges_to_insert.contains(&(start, (start.0 + 1, start.1)))
            && edges_to_insert.contains(&(start, (start.0, start.1 - 1)))
        {
            kinds.insert(start, 'L');
        } else if edges_to_insert.contains(&(start, (start.0 - 1, start.1)))
            && edges_to_insert.contains(&(start, (start.0, start.1 - 1)))
        {
            kinds.insert(start, 'J');
        } else if edges_to_insert.contains(&(start, (start.0 - 1, start.1)))
            && edges_to_insert.contains(&(start, (start.0, start.1 + 1)))
        {
            kinds.insert(start, '7');
        } else if edges_to_insert.contains(&(start, (start.0 + 1, start.1)))
            && edges_to_insert.contains(&(start, (start.0, start.1 + 1)))
        {
            kinds.insert(start, 'F');
        } else {
            bail!("Could not detect start type");
        }

        Ok(PipeInfo {
            graph,
            start,
            kinds,
        })
    }

    fn get_loop(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut cur = self.start;
        let mut prev = self.start;

        loop {
            res.push(cur);
            let next = self
                .graph
                .neighbors(cur)
                .filter(|n| *n != prev)
                .next()
                .unwrap();
            prev = cur;
            cur = next;
            if cur == self.start {
                return res;
            }
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let puzzle_input = PipeInfo::read_input(input)?;
    let loop_coords = puzzle_input.get_loop();
    Ok(loop_coords.len() / 2)
}

// This will contain a tilemap version of our pipe world.
// Each pipe has its own 3x3 tile area in the tile map, so coords in pipe_tiles will be scaled by
// a factor of 3.
struct TileMap {
    unscaled_loop: Vec<(usize, usize)>,
    pipe_tiles: HashSet<(usize, usize)>,
}

impl TileMap {
    fn build_from_pipeinfo(pipe_info: &PipeInfo) -> Self {
        let loop_pipes = pipe_info.get_loop();
        let pipe_map = HashSet::from_iter(loop_pipes.iter().flat_map(|pos| {
            let scaled_pos = (pos.0 * 3, pos.1 * 3);
            match pipe_info.kinds.get(pos).unwrap() {
                '|' => [
                    (scaled_pos.0, scaled_pos.1 - 1),
                    scaled_pos,
                    (scaled_pos.0, scaled_pos.1 + 1),
                ],
                '-' => [
                    (scaled_pos.0 - 1, scaled_pos.1),
                    scaled_pos,
                    (scaled_pos.0 + 1, scaled_pos.1),
                ],
                'L' => [
                    (scaled_pos.0 + 1, scaled_pos.1),
                    scaled_pos,
                    (scaled_pos.0, scaled_pos.1 - 1),
                ],
                'J' => [
                    (scaled_pos.0 - 1, scaled_pos.1),
                    scaled_pos,
                    (scaled_pos.0, scaled_pos.1 - 1),
                ],
                '7' => [
                    (scaled_pos.0 - 1, scaled_pos.1),
                    scaled_pos,
                    (scaled_pos.0, scaled_pos.1 + 1),
                ],
                'F' => [
                    (scaled_pos.0 + 1, scaled_pos.1),
                    scaled_pos,
                    (scaled_pos.0, scaled_pos.1 + 1),
                ],
                _ => panic!(),
            }
        }));
        TileMap {
            unscaled_loop: loop_pipes,
            pipe_tiles: pipe_map,
        }
    }

    fn find_enclosed_tiles(&self) -> Vec<(usize, usize)> {
        // The not enclosed tiles (in pipe-world coords, i.e. NOT scaled by 3)
        let mut not_enclosed = HashSet::<(usize, usize)>::new();
        // Tile world coordinates that are currently scheduled to be checked
        // We start with 0,0 since we know that it will never be enclosed
        let mut to_check: Vec<(usize, usize)> = vec![(0, 0)];
        // Tile world coordinates that already were checked and can be discarded
        let mut checked = HashSet::<(usize, usize)>::new();

        let bounds = (
            *self.pipe_tiles.iter().map(|(x, _)| x).max().unwrap() + 1,
            *self.pipe_tiles.iter().map(|(_, y)| y).max().unwrap() + 1,
        );

        // Helper function to calculate valid neighbors for the given coords
        let neighbors = |(x, y): (usize, usize)| -> Vec<(usize, usize)> {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(dx, dy)| {
                    x.checked_add_signed(*dx)
                        .and_then(|new_x| y.checked_add_signed(*dy).map(|new_y| (new_x, new_y)))
                })
                .filter(|(new_x, new_y)| *new_x <= bounds.0 && *new_y <= bounds.1)
                .collect::<Vec<_>>()
        };

        // Do a DFS to find all unenclosed tiles (essentially like a flood fill in paint)
        while let Some(pos) = to_check.pop() {
            checked.insert(pos);
            if pos.0 % 3 == 0 && pos.1 % 3 == 0 {
                not_enclosed.insert((pos.0 / 3, pos.1 / 3));
            }
            if !self.pipe_tiles.contains(&pos) {
                to_check.extend(neighbors(pos).iter().filter(|n| !checked.contains(n)));
            }
        }

        // Now we now all tiles that are NOT enclosed, so we can now just iterate over all tiles
        // and collect the ones that are not in our not enclosed set
        (0..=self.unscaled_loop.iter().map(|(x, _)| *x).max().unwrap())
            .cartesian_product(0..=self.unscaled_loop.iter().map(|(_, y)| *y).max().unwrap())
            .filter(|cand| {
                !not_enclosed.contains(cand) && !self.pipe_tiles.contains(&(cand.0 * 3, cand.1 * 3))
            })
            .collect()
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let puzzle_input = PipeInfo::read_input(input)?;
    let tile_map = TileMap::build_from_pipeinfo(&puzzle_input);
    let enclosed_tiles = tile_map.find_enclosed_tiles();
    Ok(enclosed_tiles.len())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day10 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example01() {
        let (dir, file) = create_example_file(
            indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 4);
        drop(dir);
    }

    #[test]
    fn test_example02() {
        let (dir, file) = create_example_file(
            indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 8);
        //assert_eq!(part2(&file).unwrap(), 2);
        drop(dir);
    }

    #[test]
    fn test_example_part2_01() {
        let (dir, file) = create_example_file(
            indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 4);
        drop(dir);
    }

    #[test]
    fn test_example_part2_02() {
        let (dir, file) = create_example_file(
            indoc! {"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 4);
        drop(dir);
    }

    #[test]
    fn test_example_part2_03() {
        let (dir, file) = create_example_file(
            indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 8);
        drop(dir);
    }
}
