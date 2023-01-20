fn main() {
    let score: u32 = include_str!("../inputs.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|elves| {
            let possible_badges = elves
                .first()
                .unwrap()
                .chars()
                .into_iter()
                .filter(|badge| elves[1].contains(*badge))
                .collect::<Vec<_>>();

            let badge = elves
                .last()
                .unwrap()
                .chars()
                .into_iter()
                .filter(|badge| possible_badges.contains(badge))
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .clone();

            if badge >= 'a' {
                return (badge as u8 - b'a') as u32 + 1 as u32;
            }

            return (badge as u8 - b'A') as u32 + 27 as u32;
        })
        .sum();

    println!(
        "The sum of the priorities of those item types is {}.",
        score
    );
}
