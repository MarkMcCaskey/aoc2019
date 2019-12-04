use std::collections::*;

#[aoc_generator(day3)]
fn input_gen(input: &str) -> (Vec<(char, usize, usize)>, Vec<(char, usize, usize)>) {
    let mut a = input.lines().map(|line| {
        let mut dist = 0;
        line.split(',')
            .map(|dir| {
                let d = dir[0..1].chars().next().unwrap();
                let amt = dir[1..].parse().unwrap();
                let res = (d, amt, dist);
                dist += amt;
                res
            })
            .collect::<Vec<(char, usize, usize)>>()
    });
    let line1 = a.next().unwrap();
    let line2 = a.next().unwrap();
    (line1, line2)
}

fn comp_location(
    mut pos1: isize,
    mut pos2: isize,
    dirs: &[(char, usize, usize)],
) -> BTreeSet<((isize, isize, usize), (isize, isize, usize))> {
    let mut set = BTreeSet::new();
    for (d, amt, dist) in dirs {
        let before = (pos1, pos2, *dist);
        let mut dist = *dist;
        match d {
            'U' => {
                pos2 += *amt as isize;
                dist += *amt;
            }
            'D' => {
                pos2 -= *amt as isize;
                dist += *amt;
            }
            'L' => {
                pos1 -= *amt as isize;
                dist += *amt;
            }
            'R' => {
                pos1 += *amt as isize;
                dist += *amt;
            }
            _ => panic!("bad direction"),
        }
        set.insert((before, (pos1, pos2, dist)));
    }
    set
}

fn point_map_from_move_set(
    set: BTreeSet<((isize, isize, usize), (isize, isize, usize))>,
) -> HashMap<(isize, isize), usize> {
    let mut point_set = HashMap::new();
    for ((bx, by, bd), (ax, ay, _)) in set {
        let x_range = if bx < ax { bx..=ax } else { ax..=bx };
        let mut dist = 0;
        for x in x_range {
            let y_range = if by < ay { by..=ay } else { ay..=by };
            for y in y_range {
                point_set.insert((x, y), bd + dist);
                dist += 1;
            }
        }
    }
    point_set.remove(&(0, 0));

    point_set
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<(char, usize, usize)>, Vec<(char, usize, usize)>)) -> usize {
    let set1 = comp_location(0, 0, &input.0);
    let set2 = comp_location(0, 0, &input.1);
    let point_map1 = point_map_from_move_set(set1);
    let point_map2 = point_map_from_move_set(set2);

    let ps1 = point_map1.keys().collect::<HashSet<_>>();
    let ps2 = point_map2.keys().collect::<HashSet<_>>();
    ps1.intersection(&ps2)
        .map(|k| (k.0.abs() + k.1.abs()) as usize)
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &(Vec<(char, usize, usize)>, Vec<(char, usize, usize)>)) -> usize {
    let set1 = comp_location(0, 0, &input.0);
    let set2 = comp_location(0, 0, &input.1);
    let point_map1 = point_map_from_move_set(set1);
    let point_map2 = point_map_from_move_set(set2);

    let ps1 = point_map1.keys().collect::<HashSet<_>>();
    let ps2 = point_map2.keys().collect::<HashSet<_>>();
    ps1.intersection(&ps2)
        .map(|k| point_map1[k] + point_map2[k])
        .min()
        .unwrap()
}
