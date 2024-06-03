from manim import *
import math

class Quadratic(Scene):
    def construct(self):
        functions = [
            MathTex(r"2x^2 + x - 6"),
            MathTex(r"2x^2 + 4x - 3x - 6"),
            MathTex(r"2x(x + 2) - 3(x + 2)"),
            MathTex(r"(2x - 3)(x + 2)"),
        ]

        graph = FunctionGraph(lambda x: x*x + x + 0)

        for index, function in enumerate(functions):
            if index == 0:
                self.play(Create(function))
                self.wait()
            else:
                self.play(ReplacementTransform(functions[index-1], function))
                self.wait()

class Sines(Scene):
    def construct(self):
        plane = NumberPlane().add_coordinates()

        points = [[i * PI, 0] for i in [-2, -1, 0, 1, 2]]
        dots = []
        for i in points:
            dots += Dot(plane.coords_to_point(i[0], i[1]))
        dot_labels = []
        for val, dot in zip([-2, -1, 0, 1, 2], dots):
            dot_labels += MathTex(f"{val}\\pi").next_to(dot, UR, 0.1)

        sines = [FunctionGraph(lambda x: 0.01*i * math.sin(x)) for i in range(25, 200, 25)]
   
        self.add(plane)
        self.add(*sins)
        self.add(*dots)
        self.add(*dot_labels)
        self.add()

class MovingSine(Scene):
    def construct(self):
        plane = NumberPlane().add_coordinates()

        sine = VGroup(FunctionGraph(lambda x: math.sin(x)))

        self.add(plane, sine)
        sine.animate.shift(RIGHT * 2)
        self.wait()
