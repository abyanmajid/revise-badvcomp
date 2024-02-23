import math
import numpy as np
import random
import matplotlib.pyplot as plt

##### Chapter 1

### Type 1: calculation between two complex number
sign = ["+", "-", "*", "/"]
string = ""
a = random.randint(1, 10) + random.randint(1, 10)*1j
string += str(a)
b = random.randint(0, 3)
string += str(sign[b])
a = random.randint(1, 10) + random.randint(1, 10)*1j
string += str(a)
# Question and result
print("Question 1:")
print("Q: " + string)
print("A: " + str(eval(string)))
print("--------------")

### Type 2: Find the real and imaginary component
a = random.randint(1, 10) + random.randint(1, 10)*1j
# Question and result
print("Question 2:")
print("Q: " + str(a))
print("real component: " + str(np.real(a)))
print("imaginary component: " + str(np.imag(a)))
print("--------------")

### Type 3: Calculate the modulus
a = random.randint(1, 10) + random.randint(1, 10)*1j
# Question and result
print("Question 3:")
print("Q: " + str(a))
print("A: " + str((np.real(a)**2+np.imag(a)**2)**(1/2)))
print("--------------")

### Type 4: Given z, find z-bar, minus z and minus z-bar or vice versa
a = random.randint(1, 10) + random.randint(1, 10)*1j
ls = [str(a), str(np.real(a) - np.imag(a)*1j), str(- np.real(a) + np.imag(a)*1j), str(- np.real(a) - np.imag(a)*1j)]
q = ["z", "z-bar", "minus z", "minus z-bar"]
seed = random.randint(0, 3)
# Question and result
print("Question 4:")
print("Q: given " + q[seed] + " = " + ls[seed] + ", find the rest.")
print("z: " + ls[0])
print("z-bar: " + ls[1])
print("minus z: " + ls[2])
print("minus z-bar: " + ls[3])
print("--------------")

