# Program to find the greatest among 3 numbers

x = float(input("Enter the value of X: "))
y = float(input("Enter the value of Y: "))
z = float(input("Enter the value of Z: "))

if x > y:
    if x > z:
        print("The greatest number is:", x)
    else:
        print("The greatest number is:", z)
else:
    if y > z:
        print("The greatest number is:", y)
    else:
        print("The greatest number is:", z)