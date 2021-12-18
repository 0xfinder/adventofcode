from pathlib import Path
import os

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "measurements.txt"), "r") as file:
    lines = file.readlines()
    count = 0
    for i in range(len(lines) - 3):
        if sum([int(e) for e in lines[i:i+3]]) < sum([int(e) for e in lines[i+1:i+4]]):
            count += 1

print(count)