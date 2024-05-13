use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

use crate::util::join3;

#[rustfmt::skip]
lazy_static! {
    pub static ref ASCII_NUMS: HashMap<char, String> = hashmap![
        '0' => r"  ___  
 / _ \ 
| | | |
| |_| |
 \___/ ".to_string(),
        '1' => r" _ 
/ |
| |
| |
|_|".to_string(),
        '2' => r" ____
|___ \ 
  __) |
 / __/ 
|_____|".to_string(),
        '3' => r" _____ 
|___ / 
  |_ \ 
 ___) |
|____/ ".to_string(),
        '4' => r" _  _   
| || |  
| || |_ 
|__   _|
   |_|  ".to_string(),
        '5' => r" ____  
| ___| 
|___ \ 
 ___) |
|____/ ".to_string(),
        '6' => r" __
 / /_  
| '_ \ 
| (_) |
 \___/ ".to_string(),
        '7' => r" _____
|___  |
   / / 
  / /  
 /_/   ".to_string(),
        '8' => r"  ___  
 ( _ ) 
 / _ \ 
| (_) |
 \___/ ".to_string(),
        '9' => r"  ___
 / _ \ 
| (_) |
 \__, |
   /_/ ".to_string(),
    ];
    pub static ref COLON: String = "   
 _ 
(_)
 _ 
(_)"
    .to_string();
}

pub fn colonate(str1: String, str2: String) -> String {
    join3(str1, COLON.to_string(), str2)
}

pub fn to_ascii(inp: impl ToString) -> Option<Vec<String>> {
    let mut to_print: Vec<String> = Vec::new();
    for i in inp.to_string().chars() {
        if let Some(a) = ASCII_NUMS.get(&i) {
            to_print.push(a.clone());
        } else {
            return None;
        }
    }

    Some(to_print)
}
