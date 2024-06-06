use multizip::{zip2, zip3};

pub fn join2(str1: String, str2: String) -> String {
    let mut to_print: Vec<String> = Vec::new();
    zip2(str1.split('\n'), str2.split('\n')).for_each(|(a, b)| to_print.push(format!("{a}{b}")));

    to_print.join("\n")
}

pub fn join3(str1: String, str2: String, str3: String) -> String {
    let mut to_print: Vec<String> = Vec::new();
    zip3(str1.split('\n'), str2.split('\n'), str3.split('\n'))
        .for_each(|(a, b, c)| to_print.push(format!("{a} {b} {c}")));

    to_print.join("\n")
}
