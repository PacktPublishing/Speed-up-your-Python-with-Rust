from .singleton import Singleton


class Counter(metaclass=Singleton):

    def __init__(self, initial_value: int = 0) -> None:
        self._value: int = initial_value

    def increase_count(self) -> None:
        self._value += 1

    @property
    def value(self) -> int:
        return self._value
