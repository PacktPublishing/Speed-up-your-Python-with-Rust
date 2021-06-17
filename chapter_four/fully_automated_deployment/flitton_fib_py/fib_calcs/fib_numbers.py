from typing import List

from .fib_number import recurring_fibonacci_number


def calculate_numbers(numbers: List[int]) -> List[int]:
    """
    Calculates a range of Fibonacci numbers from a list.

    :param numbers: (List[int]) the Fibonacci numbers to be calculated
    :return: (List[int]) the calculated Fibonacci numbers
    """
    return [recurring_fibonacci_number(number=i) for i in numbers]
