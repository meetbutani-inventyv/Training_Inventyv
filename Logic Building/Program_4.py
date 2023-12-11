# Program to find the sum of first N natural numbers

n = int(input("Enter the value of N: "))
sum = 0

for i in range(1, n+1):
    sum += i

print("Sum of first", n, "natural number is: ", sum)