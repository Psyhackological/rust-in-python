"""
PyCalc CLI using Python Fire    
"""

import fire
from libpycalc_cli import (
    add_as_string,
    subtract_as_string,
    sum_as_string,
    divide_as_string,
    multiply_as_string,
)

class Calculator(object):
    """Rust Calculator Class"""

    def add(self, num1, num2):
        """Add two numbers"""
        return add_as_string(num1, num2)

    def subtract(self, num1, num2):
        """Subtract two numbers"""
        return subtract_as_string(num1, num2)

    def multiply(self, num1, num2):
        """Multiply two numbers"""
        return multiply_as_string(num1, num2)

    def divide(self, num1, num2):
        """divide two numbers"""
        try:
            result = divide_as_string(num1, num2)
            return result
        except ZeroDivisionError as e:
            print(f"Caught exception: {e}")
            

if __name__ == "__main__":
    fire.Fire(Calculator)
    
