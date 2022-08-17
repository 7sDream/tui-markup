import sys


def element(x: int, fg: str) -> str:
    return f"<bg:{x}  <{fg} {x: >3}> >"

def block(start: int, end: int, fg: str) -> str:
    return "".join(map(lambda x: element(x, fg), range(start, end)))

with open("indexed.txt", "w") as sys.stdout:
    print("Standard ", block(0, 8, "white"))
    print(" Intense ", block(8, 16, "black"))
    print()

    for (l, r) in zip(range(16, 29, 6), range(34, 47, 6)):
        for i in range(0, 6):
            x = l + i * 36;
            y = r + i * 36;
            print(block(x, x + 6, "white"), block(y, y + 6, "black"), sep = "     ")
        print()

    print("Gray", block(232, 244, "white"))
    print("    ", block(244, 256, "black"))
