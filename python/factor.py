from math import sqrt, gcd

def get_factors(value):
    factors = []
    if value > 0:
        return [[i, value / i] for i in range(1, int(sqrt(value)) + 1) if value % i == 0]
    elif value < 0:
        return [[i, value / i] for i in range(value, 0) if value % i == 0]


def which_factor(a, b, c):
    return [i for i in get_factors(a * c) if i[0] + i[1] == b][0]

def factor(a, b, c):
    """
    0. 15x^2 + 11x + 3
    1. 15x^2 + 5x + 6x + 3
    2. 5x(3x + 1) + 2(3x + 1)
    3. (5x + 2)(3x + 1)
    """

    # 1
    grouping = [int(i) for i in which_factor(a, b, c)]
    print(f"{a}x^2 + {grouping[0]}x + {grouping[1]}x + {c}")

    # 2
    factor1 = gcd(a, grouping[0])
    factor2 = gcd(grouping[1], c)
    
    print(f"{factor1}x({a / factor1}x + {grouping[0] / factor1}) + {factor2}({grouping[1] / factor2}x + {c / factor2})")

    

### TESTS ###

a, b, c = map(int, input("Enter the coefficients (a, b, c): ").split(','))

factor(a, b, c)
