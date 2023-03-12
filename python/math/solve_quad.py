import sympy

x = sympy.Symbol('x')

a, b, c = map(int, input("Enter the coefficients (a, b, c): ").split(','))

print(sympy.solve(sympy.Eq(a*x**2+b*x+c,0)))