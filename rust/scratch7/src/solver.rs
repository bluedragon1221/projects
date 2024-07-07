use crate::card::Card;
use itertools::Itertools;

const X: usize = 7;

#[derive(Debug)]
pub struct PossiblePlay {
    pub total_points: i32,
    pub cards_used: Vec<Card>,
}

pub type PossiblePlays = Vec<PossiblePlay>;

pub fn possible_points2(cards: Vec<Card>) -> PossiblePlays {}

pub fn possible_points(cards: Vec<Card>) -> PossiblePlays {
    let mut possible_plays: PossiblePlays = Vec::new();

    for i in cards
        .into_iter()
        .map(|x| x.get_value())
        .chain(vec![0; X].into_iter())
        .combinations(X)
        .map(|x| x.into_iter().sorted().collect::<Vec<_>>())
        .unique()
    {
        let total_points = i.iter().sum();
        if total_points % 7 == 0 {
            let cards_used = i
                .into_iter()
                .filter(|x| *x != 0)
                .sorted_by(|a, b| b.cmp(&a)) // reverse sort order
                .map(|x| Card::new(x).unwrap())
                .collect();

            possible_plays.push(PossiblePlay {
                total_points,
                cards_used,
            });
        };
    }

    possible_plays
        .into_iter()
        .sorted_by(|a, b| b.total_points.cmp(&a.total_points))
        .collect::<Vec<_>>()
}
