# Program to print the pattern: 1, 6, 120, 540, ...

n = int(input("Enter the value of N: "))
j = 1
factorial = 1

for i in range(1, n+1):
    factorial *= j
    print(factorial)

    j = j + 1
    factorial *= j
    j = j + 1