use aoc_2021::vector::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    hash::Hash,
};

type Input = Vec<Scanner>;

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));

    let rots = all_rotations(Vector3::new([1, 2, 3]))
        .into_iter()
        .collect::<HashSet<_>>();
    assert_eq!(24, rots.len());

    let a = find_differences(&input[3]);
    let b = find_differences(&input[1]);

    let differences = input
        .iter()
        .map(find_differences)
        .enumerate()
        .collect::<Vec<_>>();

    for (scanner_1, set_1) in &differences {
        for (scanner_2, set_2) in &differences {
            if scanner_1 <= scanner_2 {
                continue;
            }
            if is_match(set_1, set_2).is_some() {
                println!("match: {} - {}", scanner_1, scanner_2);
            }
        }
    }

    // println!("Ex1: {}", exercise_1(&input));
    // println!("Ex2: {}", exercise_2(&input));
}

fn is_match(a: &Vec<HashSet<DiffHash>>, b: &Vec<HashSet<DiffHash>>) -> Option<usize> {
    let mut count = 0;
    for scanner_1_beacon in a {
        let mut sum = 0;
        for scanner_2_beacon in b {
            let inters = scanner_1_beacon.intersection(scanner_2_beacon).count();
            if inters > 0 {
                sum += 1;
            }
        }
        if sum > 0 {
            count += 1;
        }
    }

    if count >= 12 {
        // let result = MatchResult {

        // };
        Some(count)
    } else {
        None
    }
}

struct MatchResult {
    offset: Vector3,
    scanner: Vector3,
}

// 464 too high

fn exercise_1(scanners: &Input) -> usize {
    let mut results = scanners
        .iter()
        .enumerate()
        .map(|(i, s)| GrowResult::from_scanner(i, s))
        .collect::<Vec<_>>();

    grow_from(results)
}

fn grow_from(results: Vec<GrowResult>) -> usize {
    let mut matches = HashMap::new();
    let mut connections = HashMap::new();

    for a in 0..results.len() {
        for b in 0..results.len() {
            if a <= b {
                continue;
            }
            if let Some(offset) = combine_pair(&results[a], &results[b]) {
                println!("match: {} - {}", a, b);
                matches.insert((a, b), offset);
                connections.entry(a).or_insert_with(|| Vec::new()).push(b);
                connections.entry(b).or_insert_with(|| Vec::new()).push(a);

                let data = combine_pair(&results[b], &results[a]).unwrap();
                matches.insert((b, a), data);
            }
        }
    }

    let mut growth = results[0].clone();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front((0usize, Vector3::zero()));

    while let Some((node, offset)) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }

        // Collect pairs for next round
        for child in &connections[&node] {
            // There is a pair
            if let Some((child_grow, child_offset)) = matches.get(&(node, *child)) {
                queue.push_front((*child, offset + child_offset.offset));
            }
        }

        growth = combine_pair(&growth, &results[node]).unwrap().0;
    }

    let max = growth
        .scanners
        .values()
        .map(|(_, pos)| {
            growth
                .scanners
                .values()
                .map(|(_, pos2)| Vector3::manhattan(pos, pos2))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("max: {}", max);

    growth.space.len()
}

fn combine_pair(a: &GrowResult, b: &GrowResult) -> Option<(GrowResult, DirectionMatchInfo)> {
    let rotation_space = b.rotation_space();

    let found = (0..24)
        .filter_map(|x| is_directional_match(&a.space, &rotation_space[x]).map(|v| (x, v)))
        .next();

    if let Some((rot, info)) = found {
        let mut new_scanners = a.scanners.clone();
        for (bscan, (dir, loc)) in &b.scanners {
            let position = info.source_beacon; // go from scanner 0 to beacon
            let position = position - info.target_beacon;
            new_scanners.insert(*bscan, (*dir, position));
        }

        let mut new_space = a.space.clone();
        for bvecs in &rotation_space[rot] {
            new_space.insert(info.offset + *bvecs);
        }

        Some((
            GrowResult {
                scanners: new_scanners,
                space: new_space,
            },
            info,
        ))
    } else {
        None
    }
}

struct DirectionMatchInfo {
    offset: Vector3,
    source_beacon: Vector3,
    target_beacon: Vector3,
}

fn is_directional_match(
    space: &HashSet<Vector3>,
    beacon_rotations: &Vec<Vector3>,
) -> Option<DirectionMatchInfo> {
    for source_beacon in space {
        for matched_beacon in beacon_rotations {
            let offset = *source_beacon - *matched_beacon;
            let matching = beacon_rotations
                .par_iter()
                .filter(|x| space.contains(&(**x + offset)))
                .count();

            if matching >= 12 {
                return Some(DirectionMatchInfo {
                    offset,
                    source_beacon: *source_beacon,
                    target_beacon: *matched_beacon,
                });
            }
        }
    }
    None
}

fn find_differences(a: &Scanner) -> Vec<HashSet<DiffHash>> {
    a.beacons
        .iter()
        .map(|v1| {
            a.beacons
                .iter()
                .filter_map(|v2| {
                    if v1 == v2 {
                        None
                    } else {
                        Some(DiffHash::from(*v1 - *v2))
                    }
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct DiffHash(BTreeSet<Vector3>);

impl From<Vector3> for DiffHash {
    fn from(v: Vector3) -> Self {
        DiffHash(all_rotations(v).into_iter().collect())
    }
}

// // First two values have to be positive.
// fn compare_directions(a: Vector3, b: Vector3) -> Vec<usize> {
//     all_rotations(a)
//         .into_iter()
//         .enumerate()
//         .filter_map(|(i, v)| if b == v { Some(i) } else { None })
//         .collect()
// }

fn all_rotations(a: Vector3) -> Vec<Vector3> {
    let b = rotate_x_clockwise(a);
    let c = rotate_x_clockwise(b);
    let d = rotate_x_clockwise(c);
    let e = rotate_y_clockwise(a);
    let f = rotate_y_clockwise(e);
    let g = rotate_y_clockwise(f);

    vec![a, b, c, d, e, g]
        .into_iter()
        .map(z_rotations)
        .flatten()
        .collect()
}

// n, 1, 2
// n, 2, -1
fn rotate_x_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[0], a[2], -a[1]])
}

// 1, n, 2
// 2, n, -1
fn rotate_y_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[2], a[1], -a[0]])
}

// 1, 2, n
// 2, -1, n
fn rotate_z_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[1], -a[0], a[2]])
}

fn z_rotations(a: Vector3) -> [Vector3; 4] {
    let b = rotate_z_clockwise(a);
    let c = rotate_z_clockwise(b);
    let d = rotate_z_clockwise(c);
    [a, b, c, d]
}

fn exercise_2(input: &Input) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Vector3>,
}

#[derive(Debug, Clone)]
struct GrowResult {
    space: HashSet<Vector3>,
    scanners: HashMap<usize, (usize, Vector3)>,
}

impl GrowResult {
    pub fn from_scanner(index: usize, scanner: &Scanner) -> GrowResult {
        let space = scanner.beacons.iter().cloned().collect();
        let mut scanners = HashMap::new();
        scanners.insert(index, (0, Vector3::new([0, 0, 0])));
        GrowResult { space, scanners }
    }

    pub fn rotation_space(&self) -> Vec<Vec<Vector3>> {
        let mut result = vec![vec![]; 24];

        for beacon in &self.space {
            for (rotation, vec) in all_rotations(*beacon).iter().enumerate() {
                result[rotation].push(*vec);
            }
        }

        result
    }
}

fn parse_input(input: &str) -> Input {
    let mut scanners = Vec::new();
    let mut lines = input.lines();

    while let Some(scanner_line) = lines.next() {
        let beacons = lines
            .by_ref()
            .take_while(|x| !x.is_empty())
            .map(|line| {
                let mut split = line.split(',');
                let x = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();
                let y = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();
                let z = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();

                Vector3::new([x, y, z])
            })
            .collect();
        scanners.push(Scanner { beacons });
    }

    scanners
}
