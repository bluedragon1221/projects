use staar_ai_lee::{Error, RawParagraph};

fn main() -> Result<(), Error> {
    let essay = RawParagraph::new(
        "\"\"...
asdfsadf",
    )
    .parse()?;
    println!("{:#?}", essay.passes());

    Ok(())
}
