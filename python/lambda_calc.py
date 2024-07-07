from typing import Callable
# no data structures: only nums
# no if/else or advanced anything

def l_true[T](x: T, y: T) -> T:
    return x

def l_false[T](x: T, y: T) -> T:
    return y

def l_ifthen[T](b: Callable[[T, T], T], x: T, y: T) -> T:
    b(x, y)

cond = l_ifthen(l_true, 1, 2)
print(cond)
