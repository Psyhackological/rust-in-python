#!/usr/bin/env python3

import pyo3_example

(a, b) = (12, 34)
sum_ints_as_string = pyo3_example.sum_as_string(a, b)

print(f"{a} + {b} = {sum_ints_as_string}")
print(f"type({a}) = {type(a)}")
print(f"type({b}) = {type(b)}")
print(f"type({sum_ints_as_string}) = {type(sum_ints_as_string)}")
