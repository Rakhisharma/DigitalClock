use std::time;

use chrono::{Local};

// https://www.w3.org/TR/xml-entity-names/025.html
const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

fn main() {
    loop {
        let t = Local::now();
        let time = t.format("%H:%M:%S").to_string();
        for row in &DIGITS {
            for c in time.chars() {
               // now that we have chars of time, we want to match the pattern
               let col = match c {
                '0'..='9' => c as usize - '0' as usize, //this will give an integer
                ':' => 10,
                 _ => 10,
               };

               print!("{} ", row[col]);
            }
            println!();
        }
        std::thread::sleep(time::Duration::from_millis(999))
    }
}
