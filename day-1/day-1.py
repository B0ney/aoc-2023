
chars = open("input.txt", "r")
count = 0


for c in chars:
    if c == '(':
        count += 1
    elif c == ')':
        count -= 1


print(count)