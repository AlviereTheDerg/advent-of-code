
import math
from functools import reduce
import operator

with open("day06/day06.txt") as file:
    time = file.readline().strip()
    time = time[time.find(":")+1:].split()
    distance = file.readline().strip()
    distance = distance[distance.find(":")+1:].split()

def possible_times(total_time, distance_to_beat):
    low = (-total_time + (total_time**2 - (4 * -1 * -distance_to_beat))**0.5) / -2
    high = (-total_time - (total_time**2 - (4 * -1 * -distance_to_beat))**0.5) / -2
    return math.ceil(high) - math.floor(low) - 1

print(reduce(operator.mul, (possible_times(int(time_of), int(distance_of)) for time_of,distance_of in zip(time,distance))))
