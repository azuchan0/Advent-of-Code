use std::fs;

struct Dial {
    pos: i32,
    abs_pos: i32,
    size: i32,
}

impl Dial {
    fn new() -> Self {
        Self {
            pos: 50,
            abs_pos: 50,
            size: 100,
        }
    }

    fn seek(&mut self, movement: i32) -> (usize, usize) {
        let old_abs = self.abs_pos;
        self.abs_pos += movement;
        self.pos = self.abs_pos.rem_euclid(self.size);
        let crossings = if movement > 0 {
            (self.abs_pos.div_euclid(self.size) - old_abs.div_euclid(self.size)) as usize
        } else if movement < 0 {
            let ceil = |a: i32, n: i32| -> i32 { -((-a).div_euclid(n)) };
            (ceil(old_abs, self.size) - ceil(self.abs_pos, self.size)) as usize
        } else {
            0
        };
        (self.pos as usize, crossings)
    }
}

fn main() {
    let mut safe_dial = Dial::new();
    let vals: [usize; 100] = std::array::from_fn(|i| i);
    let mut result = 0;
    let file = fs::read_to_string("./input.txt").expect("file not read");
    println!("init pos {}", vals[safe_dial.pos as usize]);

    for line in file.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (dir, val_str) = line.split_at(1);

        let amount: i32 = match val_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", line);
                continue;
            }
        };

        let movement: i32 = match dir {
            "R" => amount,
            "L" => -amount,
            _ => {
                println!("line ignored, unknown direction: {}", line);
                continue;
            }
        };

        let (index, crossings) = safe_dial.seek(movement);

        result += crossings;

        println!(
            "Instruction {} -> pos is now {}, {}",
            line, vals[index], result
        );
    }
}
