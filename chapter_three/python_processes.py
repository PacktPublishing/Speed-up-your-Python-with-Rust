import time
from multiprocessing import Pool


def recur_fibo(n: int) -> int:
    if n <= 1:
        return n
    else:
        return (recur_fibo(n-1) + recur_fibo(n-2))


if __name__ == '__main__':
    start = time.time()
    recur_fibo(n=8)
    recur_fibo(n=12)
    recur_fibo(n=12)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=36)
    recur_fibo(n=46)
    recur_fibo(n=46)
    recur_fibo(n=46)
    finish = time.time()
    print(f"{finish - start} has elapsed")
    
    start = time.time()
    with Pool(4) as p:
        print(p.starmap(recur_fibo, [(8,), (12,), (12,), (20,), (20,), (20,), (20,), 
                                     (28,), (28,), (28,), (28,), (36,), (46,), (46,), (46,)]))
    finish = time.time()
    print(f"{finish - start} has elapsed")

