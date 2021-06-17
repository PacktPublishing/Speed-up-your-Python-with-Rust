from unittest import main, TestCase

from flitton_fib_py.fib_calcs.fib_number \
    import recurring_fibonacci_number


class RecurringFibNumberTest(TestCase):

    def test_zero(self):
        self.assertEqual(
            0, recurring_fibonacci_number(number=0)
        )

    def test_negative(self):
        self.assertEqual(
            None, recurring_fibonacci_number(number=-1)
        )

    def test_one(self):
        self.assertEqual(
            1, recurring_fibonacci_number(number=1)
        )

    def test_two(self):
        self.assertEqual(
            1, recurring_fibonacci_number(number=2)
        )

    def test_twenty(self):
        self.assertEqual(
            6765, recurring_fibonacci_number(number=20)
        )


if __name__ == "__main__":
    main()
