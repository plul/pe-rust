"""
A simple solution prototype in Python 3.

For large problem sizes, such as n=100000 this is actually faster than the Rust solution.

What makes this problem so exceptionally easy to solve in Python is the combination of two things:
1. Standard numbers in Python have arbitrary precision.
2. Computation of logarithms are readily available for these arbitrary precision numbers via the math.log() function.
"""

import math
from time import time


def problem(n):
    fib = 0
    fib_next = 1
    idx = 0

    while True:
        fib, fib_next = fib_next, fib + fib_next
        idx += 1

        # Use the logarithm to compute the number of digits that would be used, were the number to be represented in
        # base 10.
        base_10_length = math.floor(math.log(fib, 10)) + 1

        if base_10_length >= n:
            return idx


t_0 = time()
result = problem(1000)
t_1 = time()

print("Result:", result)
print("Time:  ", t_1 - t_0, "seconds")
