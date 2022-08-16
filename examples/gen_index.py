import sys


def element(x: int) -> str:
    return f"<bg:{x}  {x: >3} >"

def block(start: int, end: int) -> str:
    return "".join(map(element, range(start, end)))

with open("indexed.txt", "w") as sys.stdout:
    print("Standard:", block(0, 8))
    print(" Intense:", block(8, 16))
    print()

    for (l, r) in zip(range(16, 29, 6), range(34, 47, 6)):
        for i in range(0, 6):
            x = l + i * 36;
            y = r + i * 36;
            print(block(x, x + 6), "     ", block(y, y + 6), sep = "")
        print()

    print("  Grays:", block(232, 244))
    print("        ", block(244, 256))
