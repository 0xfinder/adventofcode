from pathlib import Path
import os

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "measurements.txt"), "r") as file:
    lines = file.readlines()
    count = 0
    for i in range(len(lines) - 1):
        if int(lines[i]) < int(lines[i + 1]):
            count += 1

print(count)