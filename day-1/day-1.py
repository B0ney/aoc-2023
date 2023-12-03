def calibration(input: str):
    first = None
    last = None

    for char in input:
        if char.isdigit():
            if first is None:
                first = int(char)
            else:
                last = int(char)

    if last is None:
        last = first

    return (first * 10) + last


# part 1
with open("input.txt", "r") as data:
    total = 0

    for line in data.readlines():
        total += calibration(line)

    print(total)


mapping = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]

def replace(old: str):
    result = ""

    for x in range(len(old)):
        for idx, i in enumerate(mapping):
            if old[x:].startswith(i):
                result += str(idx + 1)

        if old[x].isdigit():
            result += old[x]

    return result


# part 2
with open("input.txt", "r") as data:
    total = 0

    for line in data.readlines():
        total += calibration(replace(line))

    print(total)
