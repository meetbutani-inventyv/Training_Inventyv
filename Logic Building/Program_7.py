# Program to convert Rupees to paise and vice-versa

val = float(input("Enter the value: "))
choice = input("Enter \'P\' to convert \'Rupees to Paisa\' and \'R\' to convert \'Paisa to Rupees\': ")

if choice == "P":
    paisa = val * 100
    print("Corresponding Paisa value is: ", paisa)
else:
    if choice == "R":
        rupees = val / 100
        print("Corresponding Rupees value is: ", rupees)
    else:
        print('Invalid choice!')