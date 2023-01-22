fn main() {
    let number_of_contained_ranges: u32 = include_str!("../inputs.txt")
        .lines()
        .map(|line| {
            let pair = line.split(',');

            let halves = pair
                .map(|half| {
                    half.split('-')
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            range_is_contained(&halves[0], &halves[1]) as u32
        })
        .sum();

    println!(
        "This is the number of fully contained ranges in pairs {}.",
        number_of_contained_ranges
    );
}

fn range_is_contained(first_half: &Vec<u32>, last_half: &Vec<u32>) -> bool {
    let first_half_contains_last = first_half[0] <= last_half[0] && first_half[1] >= last_half[1];
    let last_half_contains_first = last_half[0] <= first_half[0] && last_half[1] >= first_half[1];

    first_half_contains_last || last_half_contains_first
}
