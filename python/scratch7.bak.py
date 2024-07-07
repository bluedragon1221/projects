import sys
from itertools import combinations

# nums is a list with length of 3..=7
def possible_points(cards: list[int]):
    # a play looks like this: (points, [card1, card2, ...])
    possible_plays=[]

    # 1. order doesn't matter
    # 2. Not all elements used (as many zeros as you want)
    # 3. no duplicates
    x = 7
    cards += [0]*x
    for i in set(combinations(cards, x)):
        add=sum(i)
        if add % 7 == 0:
            possible_plays.append((int(add/7), list(i)))

    return possible_plays
    
def fmt_cards(card):
    if type(card) is int:
        match card:
            case 11: card = 'J'
            case 12: card = 'Q'
            case 13: card = 'K'
        return f"[{card}]"
    elif type(card) is list:
        return ', '.join([fmt_cards(i) for i in card])

def to_int(text):
    try:
        return int(text)
    except ValueError:
        match text:
            case 'J': return 11
            case 'Q': return 12
            case 'K': return 13

if __name__ == "__main__":
    args=sys.argv[1:]
    # print(f"cards: {fmt_cards(args)}")
    possible_plays = possible_points([to_int(i) for i in args])

    possible_plays.sort(reverse=True, key=lambda x: x[0])
    play, play_cards = possible_plays[0]
    print(f"best play: {play} points: {fmt_cards([i for i in play_cards if i != 0])}")
