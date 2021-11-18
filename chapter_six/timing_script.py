from flitton_fib_rs.flitton_fib_rs import FibProcessor
import time


class PythonFibProcessor:

	def __init__(self, number, numbers):
		self.number = number
		self.numbers = numbers
		self.numbers_results = None
		self.number_results = None
		self._process()

	def _process(self):
		self.numbers_results = [self.calculate_numbers(i) for i in self.numbers]
		self.number_results = self.calculate_numbers(self.number)

	def fibonacci_number(self, number):
		if number < 0:
			return None
		elif number <= 2:
			return 1
		else:
			return self.fibonacci_number(number - 1) + self.fibonacci_number(number - 2)

	def calculate_numbers(self, numbers):
		return [self.fibonacci_number(i) for i in numbers]


t_one = time.time()
test = FibProcessor([11, 12, 13, 14], [[11, 12], [13, 14], [15, 16]])
t_two = time.time()
print(t_two - t_one)


t_one = time.time()

test = PythonFibProcessor([11, 12, 13, 14], [[11, 12], [13, 14], [15, 16]])
t_two = time.time()
print(t_two - t_one)
