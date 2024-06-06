mod ascii;
mod clock;
mod util;

use chrono::{DateTime, Local};

fn main() {
    let now: DateTime<Local> = Local::now();

    println!("{}", clock::time_str(now).expect("Err"));
}
