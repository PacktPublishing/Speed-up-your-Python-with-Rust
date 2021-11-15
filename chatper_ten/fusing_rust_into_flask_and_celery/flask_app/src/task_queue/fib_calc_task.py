from data_access import dal
from fib_calcs.fib_calculation import FibCalculation
from models.database.fib_entry import FibEntry

from config import GlobalParams
from fib_calcs import calc_fib_num


def create_calculate_fib(input_celery):
    @input_celery.task()
    def calculate_fib(number):
        params = GlobalParams()
        fib_number, _ = calc_fib_num(input_number=number,
                                     method=params.get(
                                         "CELERY_METHOD",
                                         "rust")
                                     )
        fib_entry = FibEntry(input_number=number,
                             calculated_number=fib_number)
        dal.session.add(fib_entry)
        dal.session.commit()
    return calculate_fib

