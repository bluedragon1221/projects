def adder(*args):
    match len(args):
        case 1:
            return lambda x: x + args[0]
        case _:
            return sum(args)

print(adder(1, 5))
plus_two = adder(2)
print(plus_two(10))
