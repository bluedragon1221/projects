from functools import lru_cache

@lru_cache
def fib(n):
    return 1 if n <= 1 else fib(n-2) + fib(n-1)

print(fib(100))
