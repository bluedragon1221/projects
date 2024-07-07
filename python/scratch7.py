from itertools import combinations
from typing import Collection

type Card = int
type PossiblePlay = Collection[int, Collection[Card]]
type PossiblePlays = Collection[PossiblePlay]

def possible_plays(cards: Collection[Card]) -> PossiblePlays:
    ext_cards=cards+[0]*len(cards)
    return {
        (int(sum(i)/7), i)
        for i in combinations(ext_cards, len(cards))
        if sum(i) % 7 == 0
    }

card_values = { "A": 1, "J": 11, "Q": 12, "K": 13 }
rev_card_values = { k: v for v, k in card_values.items() }

def fmt_cards(card: Collection[Card] | Card) -> str:
    match card:
        case int():
            a = rev_card_values.get(card, card)
            return f"[{a}]"
        case list() | tuple():
            return ', '.join([fmt_cards(i) for i in card])


def fmt_play(play: PossiblePlay) -> str:
    points, cards = play
    cards = [i for i in sorted(cards, reverse=True) if i != 0]
    return f"{points} points: {fmt_cards(cards)}"

def from_str(card: str) -> Card:
    a = card_values.get(card)
    if not a: a = int(card)
    return a

import sys
if __name__ == "__main__":
    args=sys.argv[1:]
    plays = possible_plays([from_str(i) for i in args])

    for i in sorted(plays, reverse=True, key=lambda x: x[0]):
        print(fmt_play(i))
