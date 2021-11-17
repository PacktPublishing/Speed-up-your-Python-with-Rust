from typing import Dict
from sqlalchemy import Column, Integer
from data_access import dal


class FibEntry(dal.base):

    __tablename__ = "fib_entries"

    id = Column(Integer, primary_key=True)
    input_number = Column(Integer)
    calculated_number = Column(Integer)

    @property
    def package(self) -> Dict[str, int]:
        return {
            "input_number": self.input_number,
            "calculated_number": self.calculated_number
        }
