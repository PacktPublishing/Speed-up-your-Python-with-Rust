from typing import Union, List, Optional

from .flitton_fib_rs import fibonacci_number, fibonacci_numbers
from .counter import Counter


class FlittonFibNumberAdapter:

    def __init__(self, number_input: Union[int, List[int]]) -> None:
        self.input: Union[int, List[int]] = number_input
        self.success: bool = False
        self.result: Optional[Union[int, List[int]]] = None
        self.error_message: Optional[str] = None
        self._counter: Counter = Counter()
        self._process_input()

    def _define_success(self) -> None:
        self.success = True
        self._counter.increase_count()

    def _process_input(self) -> None:
        if isinstance(self.input, int):
            self.result = fibonacci_number(n=self.input)
            self._define_success()

        elif isinstance(self.input, list):
            self.result = fibonacci_numbers(numbers=self.input)
            self._define_success()
        else:
            self.error_message = "input needs to be a list of ints or an int"

    @property
    def count(self) -> int:
        return self._counter.value
