# Program to print the pattern: 1, -2, 3, -4, 5, -6, 7, ...

n = int(input("Enter the value of N: "))
multiplier = 1

for i in range(1, n+1):
    print(i * multiplier)

    multiplier = multiplier * (-1)