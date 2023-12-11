# Program to find the greatest among 2 numbers

x = float(input("Enter the value of X: "))
y = float(input("Enter the value of Y: "))

if x == y:
    print("X and Y both are equal.")

else:
    if x > y:
        print("X is greater than Y.")
    else:
        print("Y is greater than X.")