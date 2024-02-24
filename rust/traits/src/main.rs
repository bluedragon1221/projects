struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

trait Summary {
    fn Summarize(&self) -> String;
}

impl Summary for NewsArticle {}

fn main() {
    println!("Hello, world!");
}
