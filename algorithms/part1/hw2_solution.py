from collections import deque
from math import floor
from typing import *


def num_inversions(data: List) -> tuple[int, deque]:
    n = len(data) 

    if n == 1:
        return 0, deque(data)
    else:
        m = floor(n / 2)
        left_inv, left = num_inversions(data[:m])
        right_inv, right = num_inversions(data[m:])
        total_inv = left_inv + right_inv
        full_data = deque()

        for _ in range(n):
            match (left, right):
                case ([l, *_], [r, *_]):
                    if l < r:
                        full_data.append(left.popleft())
                    else:
                        full_data.append(right.popleft())
                        total_inv += len(left)
                case ([], r):
                    full_data.extend(r)
                    return total_inv, full_data
                case (l, []):
                    full_data.extend(l)
                    return total_inv, full_data



datafile = open('hw2_integer_array.txt').read().split('\n')[:-1]
my_data = [int(x) for x in datafile]

n_inversions, _ = num_inversions(my_data)
print(n_inversions)
