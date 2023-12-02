
chars = open("input.txt", "r")
count = 0
pos = 0

for c in chars.read():
    pos += 1

    if c == '(':
        count += 1
    elif c == ')':
        count -= 1

    if count == -1:
        break

chars.close()

print(pos)