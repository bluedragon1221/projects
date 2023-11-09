from math import *

def attrs(a, b, c):
    magic = -b / (2 * a)

    magic_plug = (a*(magic**2)) + (b*magic) + c

    # roots = [
    #     (-b + sqrt(b**2 - (4 * a * c))) / (2 * a),
    #     (-b - sqrt(b**2 - (4 * a * c))) / (2 * a)
    # ]

    if a > 0:
        direction = ">="
        point = "min"
    elif a < 0:
        direction = "<="
        point = "max"



    return {
        "y-int": (0, c),
        "aos": f"x = {magic}",
        "vertex": (magic, magic_plug),
        # "x-int": roots,
        # "domain": "all reals",
        # "range": f"y {direction} {magic_plug}"
        "max/min": point
    }

if __name__ == "__main__":
    # print(attrs(-2, 0, 0))
    print(attrs(1, -4, 4))
