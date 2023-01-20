fn main() {
    let score = include_str!("../inputs.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let mut index_second: usize = 0;

            for char in first.chars() {
                while index_second < second.len() && second.as_bytes()[index_second] as char != char
                {
                    index_second += 1;
                }

                if index_second != second.len() {
                    if char >= 'A' && char <= 'Z' {
                        return (char as u32 - b'A' as u32 + 27) as u32;
                    }
                    return (char as u32 - b'a' as u32 + 1) as u32;
                }

                index_second = 0;
            }

            0
        })
        .sum::<u32>();

    println!(
        "The sum of the priorities of those item types is {}.",
        score
    );
}
