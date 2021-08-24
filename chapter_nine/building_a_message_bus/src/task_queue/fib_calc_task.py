from data_access import dal
from fib_calcs.fib_calculation import FibCalculation
from models.database.fib_entry import FibEntry


def create_calculate_fib(input_celery):
    @input_celery.task()
    def calculate_fib(number):
        calculation = FibCalculation(input_number=number)
        fib_entry = FibEntry(
            input_number=calculation.input_number,
            calculated_number=calculation.fib_number
        )
        dal.session.add(fib_entry)
        dal.session.commit()
    return calculate_fib
