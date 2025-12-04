pub fn part1() -> i32 {
    let input = std::fs::read_to_string("src/days/inputs/01.txt").expect("Couldn't read the file");
    let mut dial = 50;

    input
        .lines()
        .filter(|line| {
            let m = line[1..].parse::<i32>().unwrap();
            dial = (dial + if line.as_bytes()[0] == b'R' { m } else { -m }).rem_euclid(100);
            dial == 0
        })
        .count() as i32
}

pub fn part2() -> i32 {
    let input = std::fs::read_to_string("src/days/inputs/01.txt").expect("Couldn't read the file");
    let mut dial = 50;

    input
        .lines()
        .map(|line| {
            let m = line[1..].parse::<i32>().unwrap();
            let delta = if line.as_bytes()[0] == b'R' { m } else { -m };

            let previous = dial;
            dial += delta;

            if delta > 0 {
                dial.div_euclid(100) - previous.div_euclid(100)
            } else {
                (previous).div_euclid(100) - (dial).div_euclid(100) // -1 for zero landings
            }
        })
        .sum::<i32>()
}
