from flask import Flask

from fib_calcs.fib_calculation import FibCalculation

app = Flask(__name__)


@app.route("/")
def home():
    return "home for the fib calculator"


@app.route("/calculate/<int:number>")
def calculate(number):
    calc = FibCalculation(input_number=number)
    return f"you entered {calc.input_number} " \
           f"which has a Fibonacci number of " \
           f"{calc.fib_number}"



if __name__ == "__main__":
    app.run(use_reloader=True, port=5002, threaded=True)
