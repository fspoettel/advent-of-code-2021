use itertools::Itertools;

type Pattern = Vec<char>;
type Signal = [Pattern; 10];
type Output = [Pattern; 4];

fn parse_input(input: &str) -> Vec<(Signal, Output)> {
    input
        .lines()
        .map(|l| {
            let patterns: Vec<Pattern> = l
                .replace(" |", "")
                .split(' ')
                .map(|s| s.chars().collect())
                .collect();
            (
                (patterns[0..10]).to_owned().try_into().unwrap(),
                (patterns[10..14]).to_owned().try_into().unwrap(),
            )
        })
        .collect()
}

// c,f
fn is_one(pattern: &[char]) -> bool {
    pattern.len() == 2
}

// b,c,d,f
fn is_four(pattern: &[char]) -> bool {
    pattern.len() == 4
}

// a,c,f
fn is_seven(pattern: &[char]) -> bool {
    pattern.len() == 3
}

// a,b,c,d,e,f,g
fn is_eight(pattern: &[char]) -> bool {
    pattern.len() == 7
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .flat_map(|(_, output)| output)
        .filter(|p| is_one(p) || is_four(p) || is_seven(p) || is_eight(p))
        .count()
}

/// The display is made up of segments `a-g` mapped as follows:
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
#[derive(Clone, Debug)]
struct Display {
    known_segments: Vec<char>,
    a: Option<char>,
    b: Option<char>,
    c: Option<char>,
    d: Option<char>,
    e: Option<char>,
    f: Option<char>,
    g: Option<char>,
}

impl Display {
    fn new() -> Display {
        Display {
            known_segments: Vec::new(),
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
        }
    }

    /// The signal has to be reconstructed segment-by-segment before the display can decode digits.
    /// In order to do so, we track a partial state and append segments once identified.
    fn set(&mut self, target: char, c: char) {
        self.known_segments.push(c);
        match c {
            'a' => self.a = Some(target),
            'b' => self.b = Some(target),
            'c' => self.c = Some(target),
            'd' => self.d = Some(target),
            'e' => self.e = Some(target),
            'f' => self.f = Some(target),
            'g' => self.g = Some(target),
            _ => {}
        }
    }

    fn has(&self, key: &char) -> bool {
        self.known_segments.contains(key)
    }

    /// Once a display is fully reconstructed, we can decode digits with it.
    /// Individual digits are returned as strings since we need to join 4 digits in the caller.
    fn decode(&self, pattern: &[char]) -> char {
        if is_one(pattern) {
            '1'
        } else if is_four(pattern) {
            '4'
        } else if is_seven(pattern) {
            '7'
        } else if is_eight(pattern) {
            '8'
        } else {
            let displayed: String = pattern
                .iter()
                .map(|c| match c {
                    'a' => self.a.unwrap(),
                    'b' => self.b.unwrap(),
                    'c' => self.c.unwrap(),
                    'd' => self.d.unwrap(),
                    'e' => self.e.unwrap(),
                    'f' => self.f.unwrap(),
                    'g' => self.g.unwrap(),
                    val => panic!("tring to decode unknown segment: {}", val),
                })
                .sorted_unstable()
                .collect();

            match displayed.as_ref() {
                "abcefg" => '0',
                "acdeg" => '2',
                "acdfg" => '3',
                "abdfg" => '5',
                "abdefg" => '6',
                "abcdfg" => '9',
                val => panic!("unexpected decoded pattern: {}", val),
            }
        }
    }
}

/// Helper trait to make `.find()`ing the first digits less verbose.
trait Searchable {
    fn find_by(&self, find_fn: impl Fn(&[char]) -> bool) -> Pattern;
}

impl Searchable for Signal {
    fn find_by(&self, find_fn: impl Fn(&[char]) -> bool) -> Pattern {
        self.iter().find(|x| find_fn(x)).unwrap().to_owned()
    }
}

pub fn part_two(input: &str) -> u32 {
    fn is_six(six_segment_pattern: &[char], one_pattern: &[char]) -> bool {
        one_pattern.iter().any(|c| !six_segment_pattern.contains(c))
    }

    fn is_zero(six_segment_pattern: &[char], four_pattern: &[char]) -> bool {
        four_pattern
            .iter()
            .any(|c| !six_segment_pattern.contains(c))
    }

    parse_input(input)
        .iter()
        .map(|(signal, outputs)| {
            let mut display = Display::new();

            let one = signal.find_by(is_one);
            let seven = signal.find_by(is_seven);
            let four = signal.find_by(is_four);
            let eight = signal.find_by(is_eight);

            // once we know `c` and `f`, we can isolate `a` by looking at `7`
            display.set('a', *seven.iter().find(|c| !&one.contains(c)).unwrap());

            // at this point, we can decode the full signal by looking at six-segment components.
            for pattern in signal.iter().filter(|x| x.len() == 6) {
                if is_six(pattern, &one) {
                    for val in &one {
                        if pattern.contains(val) {
                            display.set('f', *val);
                        } else {
                            display.set('c', *val);
                        }
                    }
                } else if is_zero(pattern, &four) {
                    for val in &four {
                        if !pattern.contains(val) {
                            display.set('d', *val);
                        } else if !&one.contains(val) {
                            display.set('b', *val);
                        }
                    }
                } else {
                    for val in &eight {
                        if !pattern.contains(val) {
                            display.set('e', *val);
                        }
                    }
                }
            }

            // whatever segment is left over maps to the last needed segment `g`.
            // we can use `eight` to identify it since it has all segments.
            for val in &eight {
                if !display.has(val) {
                    display.set('g', *val);
                }
            }

            // the display is ready for decoding now.
            // We decode the 4-digit number to a string and then parse it to an int.
            let num = outputs.iter().fold(String::new(), |mut acc, pattern| {
                acc.push(display.decode(pattern));
                acc
            });

            num.parse::<u32>().unwrap()
        })
        .sum()
}

#[test]
fn test_part_one() {
    use aoc::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_one(&input), 26);
}

#[test]
fn test_part_two() {
    use aoc::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_two(&input), 61229);
}
