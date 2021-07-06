from time import time

from flitton_fib_rs.flitton_fib_rs import fibonacci_number
from numba import jit


def python_fib_number(number: int) -> int:
    if number < 0:
        raise ValueError("Fibonacci has to be equal or above zero")
    elif number in [1, 2]:
        return 1
    else:
        return python_fib_number(number - 1) + python_fib_number(number - 2)


@jit(nopython=True)
def numba_fib_number(number: int) -> int:
    if number < 0:
        raise ValueError("Fibonacci has to be equal or above zero")
    elif number in [1, 2]:
        return 1
    else:
        return numba_fib_number(number - 1) + numba_fib_number(number - 2)


t0 = time()
for i in range(0, 30):
    numba_fib_number(35)
t1 = time()
print(f"the time taken for numba is: {t1-t0}")
t0 = time()
for i in range(0, 30):
    numba_fib_number(35)
t1 = time()
print(f"the time taken for numba is: {t1 - t0}")


t0 = time()
for i in range(0, 30):
    python_fib_number(35)
t1 = time()
print(f"the time taken for python is: {t1 - t0}")


t0 = time()
for i in range(0, 30):
    fibonacci_number(35)
t1 = time()
print(f"the time taken for rust is: {t1 - t0}")

