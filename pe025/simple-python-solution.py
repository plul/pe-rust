"""
A simple solution prototype in Python 3.

What makes this problem so exceptionally easy to solve in Python is the combination of three things:
1. Numbers do not overflow.
2. Computation of logarithms are readily available for these non-overflowing numbers (the math.log() function).
3. Numbers are immutable, so operations on a number x (such as addition; x + y) does not mutate x, it just
   produces a new data structure for the result. So numbers can be shared without worrying about mutability or
   ownership.

1. and 3. are at the cost of performance, obviously, but this program only takes a second to run anyway.
I am impressed that I can even do 2., because with Rust and its BigUint type, logarithms are not implemented at
the moment, and apparantly bigint data types in Java do not have support for logarithms either. It's apparantly
non-trivial to implement.
"""

import math

fib = 0
fib_next = 1
idx = 0

while True:
    fib, fib_next = fib_next, fib + fib_next
    idx += 1

    log = math.log(fib, 10)
    digits = math.floor(log) + 1

    if digits >= 1000:
        print(idx)
        break
