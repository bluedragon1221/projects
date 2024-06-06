from get_factors import get_factors

def which_factor(a: int, b: int, c: int):
    return [i for i in get_factors(a * c) if i[0] + i[1] == b]
