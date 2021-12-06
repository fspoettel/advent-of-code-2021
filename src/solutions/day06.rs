use std::collections::HashMap;

static REPRO_INTERVAL_INITIAL: u64 = 9;
static REPRO_INTERVAL: u64 = 7;

type Generations = HashMap<u64, u64>;

fn get_initial_members(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn project_adult_generation(
    generations: &mut Generations,
    generation_size: u64,
    current_interval: u64,
    interval_count: u64,
) {
    let mut spawns_at = current_interval + REPRO_INTERVAL;
    // increase the generation_size at {interval} by the current generation_size for rest of observed interval.
    while spawns_at <= interval_count {
        *generations.entry(spawns_at).or_default() += generation_size;
        spawns_at += REPRO_INTERVAL;
    }
}

fn project_population(members: Vec<usize>, interval_count: u64) -> u64 {
    let mut generations: Generations = HashMap::new();
    let mut pop: u64 = members.len() as u64;

    // for each fish in the OG generation, spawn an offspring at {rest_timer} and project the fish's adult life.
    members.iter().for_each(|_i| {
        let i = *_i as u64;
        *generations.entry(i).or_default() += 1;
        // once the OG fish generation spawns their first offspring, they continue reproducing.
        project_adult_generation(&mut generations, 1, i, interval_count);
    });

    // a generation of fishes hatches every day. since there is no variance in reproduction rate, we can project it as a whole.
    for i in 0..interval_count {
        let generation_size = *generations.entry(i).or_default();
        // add hatched fishes to the population.
        pop += generation_size;

        let adults_at = i + REPRO_INTERVAL_INITIAL;

        // initial reproduction is slightly delayed for hatched fishes.
        // we handle it here before projecting the rest of interval_count in a loop.
        if adults_at <= interval_count {
            *generations.entry(adults_at).or_default() += generation_size;
            // Once fishes are adults, we can project the rest of {interval_count} in a loop.
            project_adult_generation(&mut generations, generation_size, adults_at, interval_count);
        }
    }

    pop
}

pub fn part_one(input: &str) -> u64 {
    let members = get_initial_members(input);
    project_population(members, 80)
}

pub fn part_two(input: &str) -> u64 {
    let members = get_initial_members(input);
    project_population(members, 256)
}

#[test]
fn test_part_one() {
    use aoc2021::read_file;
    let input = read_file("examples", 6);
    assert_eq!(part_one(&input), 5934);
}

#[test]
fn test_part_two() {
    use aoc2021::read_file;
    let input = read_file("examples", 6);
    assert_eq!(part_two(&input), 26984457539);
}
