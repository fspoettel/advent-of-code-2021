use std::collections::HashMap;

fn calculate_step(w: i64, z: i64, a: i64, b: i64, c: i64) -> i64 {
    let x = ((z % 26 + b) != w) as i64;
    (z / a) * (25 * x + 1) + ((w + c) * x)
}

fn solve() -> Vec<i64> {
    let steps = [
        (1, 13, 10),
        (1, 11, 16),
        (1, 11, 0),
        (1, 10, 13),
        (26, -14, 7),
        (26, -4, 11),
        (1, 11, 11),
        (26, -3, 10),
        (1, 12, 16),
        (26, -12, 8),
        (1, 13, 15),
        (26, -12, 2),
        (26, -15, 5),
        (26, -12, 10),
    ];

    let mut step = 0;
    let mut z_values: HashMap<i64, Vec<i64>> = HashMap::new();

    for w in 1..=9 {
        let (a, b, c) = steps[step];
        z_values
            .entry(calculate_step(w, 0, a, b, c))
            .or_default()
            .push(w);
    }

    step += 1;

    while step < 14 {
        let values: Vec<(i64, Vec<i64>)> = z_values.drain().collect();

        values.iter().for_each(|(z, nums)| {
            let (a, b, c) = steps[step];

            for w in 1..=9 {
                let next_z = calculate_step(w, *z, a, b, c);
                // optimization: remove z values above threshold.
                // optimization: remove z values that do not shrink when divided / 26.
                if (a == 1 || next_z < *z) && next_z < 1000000 {
                    let c = w.to_string().chars().next().unwrap();
                    let entry = z_values.entry(next_z).or_default();

                    let next_nums: Vec<i64> = nums
                        .iter()
                        .map(|n| {
                            let mut digits: Vec<char> = n.to_string().chars().collect();
                            digits.push(c);
                            digits.iter().collect::<String>().parse().unwrap()
                        })
                        .collect();

                    entry.push(*next_nums.iter().min().unwrap());
                    entry.push(*next_nums.iter().max().unwrap());
                }
            }
        });

        step += 1;
    }

    z_values.get(&0).unwrap().to_owned()
}

pub fn part_one(_: &str) -> i64 {
    *solve().iter().max().unwrap()
}

pub fn part_two(_: &str) -> i64 {
    *solve().iter().min().unwrap()
}
