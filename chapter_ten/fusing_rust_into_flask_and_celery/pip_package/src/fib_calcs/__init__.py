import time
from flitton_fib_rs.flitton_fib_rs import fibonacci_number
from fib_calcs.enums import CalculationMethod
from fib_calcs.fib_calculation import FibCalculation


def _time_process(processor, input_number):
    start = time.time()
    calc = processor(input_number)
    finish = time.time()
    time_taken = finish - start
    return calc, time_taken


def _process_method(input_method):
    calc_enum = CalculationMethod._value2member_map_.get(input_method)
    if calc_enum is None:
        raise ValueError(
            f"{input_method} is not supported, "
            f"please choose from "
            f"{CalculationMethod._value2member_map_.keys()}")
    return calc_enum


def calc_fib_num(input_number, method):
    if isinstance(method, str):
        method = _process_method(input_method=method)

    if method == CalculationMethod.PYTHON:
        calc, time_taken = _time_process(
            processor=FibCalculation,
            input_number=input_number
        )
        return calc.fib_number, time_taken

    elif method == CalculationMethod.RUST:
        calc, time_taken = _time_process(
            processor=fibonacci_number,
            input_number=input_number
        )
        return calc, time_taken
