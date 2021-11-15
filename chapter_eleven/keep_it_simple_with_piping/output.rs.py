import sys


for i in sys.stdin.readlines():
    number = i.replace('\n', '')
    try:
        processed_number = int(number)
        print(f"recieving: {processed_number}")
    except ValueError:
        pass
