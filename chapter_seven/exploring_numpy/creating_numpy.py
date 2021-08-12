import time
import numpy as np
from flitton_fib_rs import time_add_vectors


def python_function(total_vector_size: int) -> float:
    t1 = time.time()
    first_vector = range(total_vector_size)
    second_vector = range(total_vector_size)
    sum_vector = [first_vector[i] + second_vector[i] for i in
                  range(len(second_vector))]
    return time.time() - t1


def numpy_function(total_vector_size: int) -> float:
    t1 = time.time()
    first_vector = np.arange(total_vector_size)
    second_vector = np.arange(total_vector_size)
    sum_vector = first_vector + second_vector
    result = time.time() - t1
    if result > 0.001:
        result = 0.001
    return result


def rust_function(total_vector_size: int) -> float:
    t1 = time.time()
    sum_vector = time_add_vectors(total_vector_size)
    result = time.time() - t1
    if result > 0.001:
        result = 0.001
    return result


# print(python_function(1000))
# print(numpy_function(1000))

# python_results = [python_function(i) for i in range(0, 10000)]
numpy_results = [numpy_function(i) for i in range(0, 10000)]
rust_results = [rust_function(i) for i in range(0, 10000)]
print(rust_results[3])


import matplotlib.pyplot as plt


plt.plot(rust_results, linestyle='solid')
plt.plot(numpy_results, linestyle='dashdot')
plt.show()
