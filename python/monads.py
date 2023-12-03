from __future__ import annotations
from typing import Any, Callable

class Functor:
    def __init__(self, value):
        self.value = value

    def map(self, func: Callable) -> Functor:
        return Functor(func(self.value))

    def __repr__(self):
        return str(self.value)

class ListFunctor:
    def __init__(self, value: list):
        self.value = value

    def map(self, func: Callable) -> ListFunctor:
        return ListFunctor([func(i) for i in self.value])
