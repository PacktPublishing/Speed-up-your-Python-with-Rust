import argparse
import yaml
import os
from pprint import pprint

from .flitton_fib_rs import run_config


def config_number_command() -> None:
    parser = argparse.ArgumentParser(
        description='Calculate Fibonacci numbers '
                    'using a config file')
    parser.add_argument('--path', action='store',
                        type=str, required=True,
                        help="path to config file")
    args = parser.parse_args()

    with open(str(os.getcwd()) + "/" + args.path) as f:
        config_data: dict = yaml.safe_load(f)

    print("Here is the config data: ")
    pprint(config_data)
    print(f"Here is the result:")
    pprint(run_config(config_data))
