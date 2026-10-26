use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("file not read");
    let mut game_sum = 0;

    for (index, line) in content.lines().enumerate() {
        let id = index + 1;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        let (mut red, mut green, mut blue) = (0u32, 0u32, 0u32);
        for item in parts[1].split([',', ';']) {
            let words: Vec<&str> = item.split_whitespace().collect();
            if words.is_empty() {
                continue;
            }

            let count: u32 = words[0].parse().unwrap();
            let color = words[1];

            match color {
                "red" => red = red.max(count),
                "green" => green = green.max(count),
                "blue" => blue = blue.max(count),
                _ => {}
            }
        }
        game_sum += red * green * blue;
    }
    println!("{}", game_sum);
}
