
import networkx as nx
from queue import SimpleQueue

spots_and_pipes = {(x+1j*y):char for y,line in enumerate(open("day10/day10.txt")) for x,char in enumerate(line) if char not in {'.','\n'}}

pipes = nx.Graph()
propagation_plans = {'|':{1j, -1j}, '-':{1, -1}, 'L':{-1j, 1}, 'J':{-1j, -1}, '7':{1j, -1}, 'F':{1j, 1}}
def propagate(coord):
    for neighbour in propagation_plans.get(spots_and_pipes.get(coord, '.'), set()):
        pipes.add_edge(coord, coord + neighbour)

start = list(spots_and_pipes.keys())[list(spots_and_pipes.values()).index('S')]
for neighbour in [1, 1j, -1, -1j]:
    propagate(start + neighbour)

distance = "distance"
pipes.nodes[start][distance] = 0
search_queue = SimpleQueue()
search_queue.put(start)
searched = set()
furthest = 0
while not search_queue.empty():
    here = search_queue.get()
    if here in searched:
        continue
    searched.add(here)
    furthest = max(furthest, pipes.nodes[here][distance])
    for neighbour in pipes[here]:
        if neighbour in searched:
            continue
        propagate(neighbour)
        search_queue.put(neighbour)
        pipes.nodes[neighbour][distance] = pipes.nodes[here][distance] + 1
print(furthest)