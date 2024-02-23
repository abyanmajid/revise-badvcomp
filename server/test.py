import math
import numpy as np
import random
import matplotlib.pyplot as plt

##### Chapter 2

### Type 1: given a complex number, calculate its principal argument
a = random.randint(1, 10) + random.randint(1, 10)*1j
print("Question 1:")
print("Q: " + str(a))
print("A: " + str(np.arccos(np.real(a)/((np.real(a)**2+np.imag(a)**2)**(1/2)))))
print("--------------")

### Type 2: Given principal argument and modulus, calculate the complex number
m = random.randint(1, 10)
arg = random.random()*2*math.pi - math.pi
print("Question 2:")
print("Q: m = " + str(m) + ", arg = " + str(arg))
print("A: " + str(m*math.cos(arg) + m*math.sin(arg)*1j))
print("--------------")

### Type 3: Write a complex number in standard polar form and vice versa
m = random.randint(1, 10)
arg = random.random()*2*math.pi - math.pi
a = str(m)+"(cos(" + str(arg) + ") + i*sin(" + str(arg) + "))"
b = str(m*math.cos(arg) + m*math.sin(arg)*1j)
ls = [a, b]
seed = random.randint(0, 2)
print("Question 3:")
if seed == 0:
    print("Q: " + ls[1])
    print("A: " + ls[0])
else:
    print("Q: " + ls[0])
    print("A: " + ls[1])
print("--------------")

### Type 4: Write a complex number in exponential polar form and vice versa
m = random.randint(1, 10)
arg = random.random()*2*math.pi - math.pi
a = str(m)+ "e^(" + str(arg) + "i)"
b = str(m*math.cos(arg) + m*math.sin(arg)*1j)
ls = [a, b]
seed = random.randint(0, 2)
print("Question 4:")
if seed == 0:
    print("Q: " + ls[1])
    print("A: " + ls[0])
else:
    print("Q: " + ls[0])
    print("A: " + ls[1])
print("--------------")

### Type 5: given a complex number, calculate its power
a = random.randint(1, 10) + random.randint(1, 10)*1j
b = random.randint(1, 10)
print("Question 5:")
print("Q: " + str(a) + "^" + str(b))
print("A: " + str(a**b))
print("--------------")

### Type 6: solve quadratics and other types of polynomials with real coefficients given one root
seed = random.randint(2, 5)
print("Question 6:")
if seed == 2:
    a = random.randint(1, 10)
    b = random.randint(1, 10)*1j
    root = [(a+b), (a-b)]
    equation = "x^2 + " + str((a+b)*(a-b))
    print("Q: " + equation + " = 0")
    print("R: " + str(root[0]))
    print("A: " + str(root[0]) + ", " + str(root[1]))
elif seed == 3:
    a = random.randint(1, 10)
    b = random.randint(1, 10)*1j
    c = random.randint(1, 10)
    root = [(a+b), (a-b), c]
    equation = "x^3 - " + str(c) + "x^2 +" + str((a+b)*(a-b)) + "x - " + str(c*(a+b)*(a-b))
    print("Q: " + equation + " = 0")
    print("R: " + str(root[0]))
    print("A: " + str(root[0]) + ", " + str(root[1]) + ", " + str(root[2]))
else:
    a = random.randint(1, 10)
    b = random.randint(1, 10)*1j
    c = random.randint(1, 10)
    d = random.randint(1, 10)*1j
    root = [(a+b), (a-b), (c+d), (c-d)]
    equation = "x^4 + " + str((a+b)*(a-b) + (c+d)*(c-d)) + "x^2 + " + str((a+b)*(a-b)*(c+d)*(c-d))
    print("Q: " + equation + " = 0")
    print("R: " + str(root[0]))
    print("A: " + str(root[0]) + ", " + str(root[1]) + ", " + str(root[2]) + ", " + str(root[3]))
print("--------------")
