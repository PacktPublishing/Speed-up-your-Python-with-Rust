class FibCalculation:

    def __init__(self, input_number: int) -> None:
        self.input_number: int = input_number
        self.fib_number: int = self.recur_fib(
            n=self.input_number
        )

    @staticmethod
    def recur_fib(n: int) -> int:
        if n <= 1:
            return n
        else:
            return (FibCalculation.recur_fib(n - 1) +
                    FibCalculation.recur_fib(n - 2))
