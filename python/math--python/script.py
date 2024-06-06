from get_factors import get_factors
from which_factor import which_factor

a, b, c = map(int, input("Enter the coefficients (a, b, c): ").split(','))

print(f"All factors of {a*c}:", get_factors(a * c))
print("Chosen Factor:", which_factor(a, b, c))