from flask import Flask

from fib_calcs.fib_calculation import FibCalculation
from data_access import dal
from models.database.fib_entry import FibEntry
from task_queue.engine import make_celery


app = Flask(__name__)
celery = make_celery(app)

from task_queue.fib_calc_task import create_calculate_fib
calculate_fib = create_calculate_fib(input_celery=celery)


@app.route("/")
def home():
    return "home for the fib calculator"


@app.route("/calculate/<int:number>")
def calculate(number):
    fib_calc = dal.session.query(FibEntry).filter_by(
                           input_number=number).one_or_none()
    if fib_calc is None:
        if number < 31:
            calc = FibCalculation(input_number=number)
            new_calc = FibEntry(input_number=number,
                                calculated_number=calc.
                                fib_number)
            dal.session.add(new_calc)
            dal.session.commit()

            return f"you entered {calc.input_number} " \
                   f"which has a Fibonacci number of " \
                   f"{calc.fib_number}"
        calculate_fib.delay(number)
        return "calculate fib sent to queue because " \
               "it's above 30"
    return f"you entered {fib_calc.input_number} " \
           f"which has an existing Fibonacci number of " \
           f"{fib_calc.calculated_number}"


@app.teardown_request
def teardown_request(*args, **kwargs):
    dal.session.expire_all()
    dal.session.remove()
    dal.session.close()


if __name__ == "__main__":
    app.run(use_reloader=True, port=5002, threaded=True)
