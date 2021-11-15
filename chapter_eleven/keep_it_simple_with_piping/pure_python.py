
def recur_fib(n: int) -> int:
    if n <= 2:
        return 1
    else:
        return (recur_fib(n - 1) +
                recur_fib(n - 2))


for i in [5, 6, 7, 8, 9, 10, 15, 20, 25, 30]:
    print(recur_fib(i))
