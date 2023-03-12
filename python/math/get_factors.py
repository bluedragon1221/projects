from math import sqrt

def get_factors(value: int):
    if value > 0:
        return [[i, value / i] for i in range(1, int(sqrt(value)) + 1) if value % i == 0]
    elif value < 0:
        return [[i, value / i] for i in range(value, 0) if value % i == 0]

