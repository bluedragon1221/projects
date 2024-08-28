import string

all_letters = list(string.ascii_uppercase)
all_letters.insert(0, " ")

for i in "HELLO WORLD":
    index = all_letters.index(i)
    print(f"{index:05b}")
