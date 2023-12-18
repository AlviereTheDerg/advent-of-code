
from itertools import cycle

with open("day08/day08.txt") as file:
    directions = file.readline().strip()
    file.readline()
    pathways = {data[0]: {"L":data[2][1:-1], "R":data[3][:-1]} for data in (line.split() for line in file)}

current = 'AAA'
for steps,direction in enumerate(cycle(directions)):
    if current == 'ZZZ':
        print(steps)
        break
    current = pathways[current][direction]