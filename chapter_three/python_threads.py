from threading import Thread 
from time import sleep
from typing import Optional


class ExampleThread(Thread):
    
    def __init__(self, seconds: int, name: str) -> None:
        super().__init__()
        self.seconds: int = seconds
        self.name: str = name
        self._return: Optional[int] = None
        
    def run(self):
        print(f"thread {self.name} is running")
        sleep(self.seconds)
        print(f"thread {self.name} has finished")
        self._return = self.seconds
    
    def join(self):
        Thread.join(self)
        return self._return
    

one: ExampleThread = ExampleThread(seconds=5, name="one")
two: ExampleThread = ExampleThread(seconds=5, name="two")
three: ExampleThread = ExampleThread(seconds=5, name="three")

import time

start = time.time()
one.start()
two.start()
three.start()
print("we have started all of our threads")
one_result = one.join()
two_result = two.join()
three_result = three.join()
finish = time.time()
print(f"{finish - start} has elapsed")
print(one_result)
print(two_result)
print(three_result)