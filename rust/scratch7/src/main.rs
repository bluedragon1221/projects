use scratch7::card::{Card, CardError};
use scratch7::solver::possible_points;

fn main() -> Result<(), CardError> {
    let card_strs: Vec<_> = std::env::args().collect::<Vec<_>>()[1..].to_vec();

    let cards = card_strs
        .into_iter()
        .map(|x| Card::from_str(x.as_str()).unwrap())
        .collect::<Vec<Card>>();

    possible_points(cards).into_iter().for_each(|x| {
        println!(
            "{:2}: {}",
            x.total_points,
            x.cards_used
                .into_iter()
                .map(|y| y.fmt())
                .collect::<Vec<_>>()
                .join(" ")
        )
    });

    Ok(())
}
