
from heapq import heappop, heappush

heat_loss = {(x+y*1j):int(char) for y,line in enumerate(open("day17/day17.txt")) for x,char in enumerate(line.strip())}

source = 0
destination = max(heat_loss, key=abs)

def steer_path(minimum_distance, maximum_distance):
    tie_breaker = 0
    search_heap = [(0,0,source,1), (0,0,source,1j)] # heat dissipation of this path, tie-breaker, location, current travel direction
    overall_seen = set() # coordinate, direction
    while len(search_heap) > 0:
        heat, _, coord, direction = heappop(search_heap)
        if coord == destination:
            print(heat)
            return
        
        if (coord, direction) in overall_seen:
            continue
        overall_seen.add((coord, direction))

        for direction in [1j/direction, -1j/direction]:
            heat_diff = sum(heat_loss[coord+offset*direction] for offset in range(1,minimum_distance) if coord+offset*direction in heat_loss)
            for distance in range(minimum_distance, maximum_distance):
                neighbour = coord + distance * direction
                if neighbour not in heat_loss:
                    break
                heat_diff += heat_loss[neighbour]
                heappush(search_heap, (heat + heat_diff, (tie_breaker := tie_breaker + 1), neighbour, direction))

steer_path(1, 4)
steer_path(4, 11)