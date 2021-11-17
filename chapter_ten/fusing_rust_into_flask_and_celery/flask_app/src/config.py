import os
import sys
from typing import Dict, List

import yaml


class GlobalParams(dict):

    def __init__(self) -> None:
        super().__init__()
        self.update(self.get_yml_file())

    @staticmethod
    def get_yml_file() -> Dict:
        file_name = sys.argv[-1]
        if ".yml" not in file_name:
            file_name = "config.yml"

        if os.path.isfile(file_name):
            with open("./{}".format(file_name)) as file:
                data = yaml.load(file,
                                 Loader=yaml.FullLoader)
            return data
        raise FileNotFoundError(
            "{} config file is not available".
                format(file_name)
        )
    @property
    def database_meta(self) -> Dict[str, str]:
        db_string: str = self.get("DB_URL")
        buffer: List[str] = db_string.split("/")
        second_buffer: List[str] = buffer[-2].split(":")
        third_buffer: List[str] = second_buffer[1].split("@")
        return {
            "DB_URL": db_string,
            "DB_NAME": buffer[-1],
            "DB_USER": second_buffer[0],
            "DB_PASSWORD": third_buffer[0],
            "DB_LOCATION": f"{third_buffer[1]}:{second_buffer[-1]}",
        }
