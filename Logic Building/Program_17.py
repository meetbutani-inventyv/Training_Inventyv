# Program to print the pattern: 1, 232, 34543, 4567654, 567898765, 67890109876, ...

n = int(input("Enter the value of N: "))
i = 1

while i<=n:
    j = i

    while j <= (2*i)-1:
        print(j%10, end=" ")
        j = j+1

    j = j-2

    while j >= i:
        print(j%10, end=" ")
        j = j-1

    print()
    i = i+1