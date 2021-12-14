static REPRO_INTERVAL_INITIAL: usize = 9;
static REPRO_INTERVAL: usize = 7;

fn get_og_fishes(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn project_population(members: Vec<usize>, generations: &mut [usize]) -> usize {
    let interval_count = generations.len();
    let mut pop = members.len();

    // for each fish in the OG generation, spawn an offspring at {rest_timer}.
    // we have to project their adult life as well since OGs continue reproducing.
    members.iter().for_each(|f| {
        generations[*f] += 1;
        project_adult_generation(generations, 1, *f, interval_count);
    });

    // a generation of fishes hatches every day.
    // we can project them as a whole since there is no variance in repro. rate in a generation.
    for i in 0..interval_count {
        let generation_size = generations[i];
        // add hatched fishes to the population counter.
        pop += generation_size;

        // initial reproduction is delayed for hatched fishes. we handle it with a special case.
        let adults_at = i + REPRO_INTERVAL_INITIAL;

        if adults_at < interval_count {
            generations[adults_at] += generation_size;
            // Once fishes are adults, we can project the rest of {interval_count} in a loop.
            project_adult_generation(generations, generation_size, adults_at, interval_count);
        }
    }

    pop
}

fn project_adult_generation(
    generations: &mut [usize],
    generation_size: usize,
    current_interval: usize,
    interval_count: usize,
) {
    let mut spawns_at = current_interval + REPRO_INTERVAL;
    // increase the generation_size at {interval} by the current generation_size for rest of observed interval.
    while spawns_at < interval_count {
        generations[spawns_at] += generation_size;
        spawns_at += REPRO_INTERVAL;
    }
}

pub fn part_one(input: &str) -> usize {
    project_population(get_og_fishes(input), &mut [0; 80])
}

pub fn part_two(input: &str) -> usize {
    project_population(get_og_fishes(input), &mut [0; 256])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 5934);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 26984457539);
    }
}
