# Program to print the pattern: 1 1 1 3 2 5 3 7 5 9 8 11... (n=6)

n = int(input("Enter the value of N: "))
a = 1
b = 1
x = 1

while x<=n:
    print(a, end=" ")
    c = a + b
    a = b
    b = c
    
    print((2*x)-1, end=" ")
    x = x + 1