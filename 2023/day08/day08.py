
from itertools import cycle
from math import lcm

with open("day08/day08.txt") as file:
    directions = file.readline().strip()
    file.readline()
    pathways = {data[0]: {"L":data[2][1:-1], "R":data[3][:-1]} for data in (line.split() for line in file)}

def get_steps_needed(start, destinations):
    current = start
    for steps,direction in enumerate(cycle(directions)):
        if current in destinations:
            return steps
        current = pathways[current][direction]

print(get_steps_needed('AAA', {'ZZZ'}))

destinations = {value for value in pathways if value[-1] == 'Z'}
print(lcm(*[get_steps_needed(source, destinations) for source in pathways if source[-1] == 'A']))