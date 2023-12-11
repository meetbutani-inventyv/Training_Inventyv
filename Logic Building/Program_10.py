# Program to find the greatest among 4 numbers

a = float(input("Enter the value of A: "))
b = float(input("Enter the value of B: "))
c = float(input("Enter the value of C: "))
d = float(input("Enter the value of D: "))

if a > b:
    if a > c:
        if a > d:
            print("The greatest number is:", a)
        else:
            print("The greatest number is:", d)
    else:
        if c > d:
            print("The greatest number is:", c)
        else:
            print("The greatest number is:", d)
else:
    if b > c:
        if b > d:
            print("The greatest number is:", b)
        else:
            print("The greatest number is:", d)
    else:
        if c > d:
            print("The greatest number is:", c)
        else:
            print("The greatest number is:", d)