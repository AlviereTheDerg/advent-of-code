
from functools import reduce
import operator
inputs = [[int(item) for item in line.split()] for line in open("day09/day09.txt")]

def recursive_project(points_line):
    differential = [points_line[index+1] - points_line[index] for index in range(len(points_line) - 1)]
    for item in differential:
        if item != 0:
            break
    else:
        return points_line[0]
    projection = recursive_project(differential)
    return points_line[-1] + projection

print(reduce(operator.add, (recursive_project(line) for line in inputs)))