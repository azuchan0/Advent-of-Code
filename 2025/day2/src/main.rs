use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let inputs: Vec<&str> = file.trim().split(',').collect();
    let mut result: u64 = 0;

    for i in inputs {
        let (first_str, last_str) = i.split_once('-').unwrap();
        let first: u64 = first_str.parse().unwrap();
        let last: u64 = last_str.parse().unwrap();

        for number in first..=last {
            let number_str = number.to_string();
            let bytes = number_str.as_bytes();
            let length = bytes.len();
            let mut is_invalid = false;

            for chunk_len in 1..=(length / 2) {
                if length % chunk_len == 0 {
                    let first_chunk = &bytes[0..chunk_len];
                    if bytes.chunks(chunk_len).all(|chunk| chunk == first_chunk) {
                        is_invalid = true;
                        break;
                    }
                }
            }

            if is_invalid {
                result += number;
            }
            println!("{:?} {}", bytes, result);
        }
    }
}
