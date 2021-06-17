from unittest import main, TestCase
from unittest.mock import patch

from flitton_fib_py.fib_calcs.fib_numbers \
    import calculate_numbers


class Test(TestCase):

    @patch("flitton_fib_py.fib_calcs.fib_numbers."
           "recurring_fibonacci_number")
    def test_calculate_numbers(self, mock_fib_calc):
        expected_outcome = [mock_fib_calc.return_value,
                            mock_fib_calc.return_value]
        self.assertEqual(expected_outcome,
                         calculate_numbers(numbers=[3, 4]))

        self.assertEqual(2, len(mock_fib_calc.call_args_list))
        self.assertEqual({'number': 3},
                         mock_fib_calc.call_args_list[0][1])
        self.assertEqual({'number': 4},
                         mock_fib_calc.call_args_list[1][1])

    def test_functional(self):
        self.assertEqual([2, 3, 5],
                         calculate_numbers(numbers=[3, 4, 5]))


if __name__ == "__main__":
    main()
