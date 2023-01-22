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

            ranges_overlap(&halves[0], &halves[1]) as u32
        })
        .sum();

    println!(
        "The number of overlapping pairs is {}.",
        number_of_contained_ranges
    );
}

#[allow(dead_code)]
fn range_is_contained(first_half: &Vec<u32>, last_half: &Vec<u32>) -> bool {
    let first_half_contains_last = first_half[0] <= last_half[0] && first_half[1] >= last_half[1];
    let last_half_contains_first = last_half[0] <= first_half[0] && last_half[1] >= first_half[1];

    first_half_contains_last || last_half_contains_first
}

fn ranges_overlap(first_half: &Vec<u32>, last_half: &Vec<u32>) -> bool {
    !(last_half[1] < first_half[0] || last_half[0] > first_half[1])
}
