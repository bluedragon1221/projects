from math import sqrt
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

@dataclass(repr=True, eq=False, order=False, frozen=False)
class QuadAttrs:
    y_int: Point
    aos: str
    vertex: Point
    roots: list[int]
    has_max: bool


class Quadratic:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

    def get_roots(self) -> list[int]:
        a = self.a 
        b = self.b 
        c = self.c 

        discriminant = sqrt(b**2 - (4 * a * c))

        roots = [
            (-b + discriminant) / (2 * a),
            (-b - discriminant) / (2 * a)
        ]

        return roots

    def get_factors(self):
        a = self.a 
        b = self.b 
        c = self.c 

        if value > 0:
            return [(i, value / i) for i in range(1, int(sqrt(value)) + 1) if value % i == 0]
        elif value < 0:
            return [(i, value / i) for i in range(value, 0) if value % i == 0]

    def get_attrs(self) -> QuadAttrs:
        a = self.a 
        b = self.b 
        c = self.c 

        magic = -b / (2 * a)

        magic_plug = (a * (magic**2)) + (b * magic) + c

        discriminant = sqrt(b**2 - (4 * a * c))
        roots = [
            (-b + discriminant) / (2 * a),
            (-b - discriminant) / (2 * a)
        ]

        if a > 0:
            direction = ">="
            has_max = False
        elif a < 0:
            direction = "<="
            has_max = True

        return QuadAttrs(
            y_int=Point(0, c),
            aos=f"x = {magic}",
            vertex=Point(magic, magic_plug),
            roots=roots,
            has_max=has_max
        )

if __name__ == "__main__":

