use std::collections::BinaryHeap;

fn main() {
    let file = include_str!("../inputs.txt");

    let calories_per_elves = divide_calories(file);

    let top = *calories_per_elves.peek().unwrap();
    let top_three_total = get_top_three_total(&calories_per_elves);

    println!("Max number of calories carried by an elf is {}", top);

    println!("Top 3 total calories is {}", top_three_total);
}

///
/// Divides calories of each elves.
///
fn divide_calories(buffer: &str) -> BinaryHeap<u32> {
    let mut calories_per_elves: BinaryHeap<u32> = BinaryHeap::new();
    let mut calories = 0;

    buffer.lines().into_iter().for_each(|line| {
        if line == "" {
            calories_per_elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    });

    calories_per_elves
}

///
/// Gets the top three calories quantities and returns the sum of them.
///
fn get_top_three_total(calories_per_elves: &BinaryHeap<u32>) -> u32 {
    calories_per_elves.into_iter().take(3).sum::<u32>()
}
