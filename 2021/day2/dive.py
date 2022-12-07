import os

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "instructions.txt"), "r") as file:
    lines = file.readlines()
    horizontal_position = 0
    depth = 0
    for i in range(len(lines)):
        cmd = lines[i].split(" ")[0]
        value = int(lines[i].split(" ")[1])
        if cmd == "forward":
            horizontal_position += value
        elif cmd == "up":
            depth -= value
        elif cmd == "down":
            depth += value

    print(horizontal_position * depth)