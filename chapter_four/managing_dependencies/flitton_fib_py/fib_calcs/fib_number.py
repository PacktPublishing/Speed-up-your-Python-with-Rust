from typing import Optional


def recurring_fibonacci_number(number: int) -> Optional[int]:
    """
    Calculates the fibonacci number needed.

    :param number: (int) the Fibonacci number to be calculated
    :return: (Optional[int]) the calculated fibonacci number
    """
    if number < 0:
        return None
    elif number <= 1:
        return number
    else:
        return recurring_fibonacci_number(number - 1) + \
               recurring_fibonacci_number(number - 2)
