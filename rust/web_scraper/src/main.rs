use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;

fn main() {
    let url = "https://www.google.com/search?q=phone";
    let response = get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&response);

    let img_selector = Selector::parse("img").unwrap();
    let links: Vec<String> = document
        .select(&img_selector)
        .map(|element| element.value().attr("src").unwrap().to_string())
        .collect();

    for (index, item) in links.iter().enumerate() {
        let mut file = File::create(format!("dl/{:2>0}.jpg", index).as_str()).unwrap();
        get(item).unwrap().copy_to(&mut file).unwrap();
    }

    println!("{:#?}", links);
}
