use crate::util;

pub fn get_oxygen_co2(lines: &[String]) -> (u32, u32) {
    let bit_count = lines[0].len();

    let mut most_common: Vec<_> = lines.iter().collect();
    let mut least_common = most_common.clone();

    let mut oxygen_found = false;
    let mut co2_found = false;

    for i in 0..bit_count {
        let size = most_common.len();
        let one_count = most_common
            .iter()
            .filter(|s| s.chars().nth(i).unwrap() == '1')
            .count();
        let zero_count = size - one_count;
        let most_common_char = if one_count >= zero_count { '1' } else { '0' };
        most_common = most_common
            .iter()
            .filter(|&s| s.chars().nth(i).expect("i bad index") == most_common_char)
            .copied()
            .collect();

        if most_common.len() == 1 {
            oxygen_found = true;
            break;
        }
    }

    for i in 0..bit_count {
        let size = least_common.len();
        let one_count = least_common
            .iter()
            .filter(|s| s.chars().nth(i).unwrap() == '1')
            .count();
        let zero_count = size - one_count;
        let least_common_char = if one_count < zero_count { '1' } else { '0' };
        least_common = least_common
            .iter()
            .filter(|&s| s.chars().nth(i).expect("i bad index") == least_common_char)
            .copied()
            .collect();

        if least_common.len() == 1 {
            co2_found = true;
            break;
        }

        println!(
            "pos {} ones: {} zeroes: {} least common {}\n>> {:?}\n\n",
            i, one_count, zero_count, least_common_char, least_common
        );
    }

    println!("{}", least_common[0]);

    if !co2_found || !oxygen_found {
        return (0, 0);
    }

    let oxygen_str = most_common[0];
    let oxygen = oxygen_str
        .chars()
        .fold(0, |acc, ch| acc * 2 + ch.to_digit(10).expect("nonbinary"));

    let co2_str = least_common[0];
    let co2 = co2_str
        .chars()
        .fold(0, |acc, ch| acc * 2 + ch.to_digit(10).expect("nonbinary"));

    (oxygen, co2)
}

pub fn get_gamma_epsilon(lines: &[String]) -> (usize, usize) {
    let mut one_counts = vec![0usize; lines[0].len()];

    lines.iter().for_each(|s| {
        s.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                one_counts[i] += 1;
            }
        })
    });

    let half = lines.len() / 2;

    // most common bits
    let gamma = one_counts
        .iter()
        .map(|&one_count| if one_count > half { 1 } else { 0 })
        .fold(0, |acc, bit| acc * 2 + bit);

    let eps = one_counts
        .iter()
        .map(|&one_count| if one_count > half { 0 } else { 1 })
        .fold(0, |acc, bit| acc * 2 + bit);

    (gamma, eps)
}

pub fn part1() -> usize {
    let lines = util::read_lines("inputs/day3").unwrap();

    let (gamma, eps) = get_gamma_epsilon(&lines);

    gamma * eps
}

pub fn part2() -> usize {
    let lines = util::read_lines("inputs/day3").unwrap();

    let (oxygen, co2) = get_oxygen_co2(&lines);

    println!("oxy {} co2 {}", oxygen, co2);

    (oxygen * co2) as usize
}
