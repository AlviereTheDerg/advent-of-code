
import networkx as nx
from queue import SimpleQueue
from functools import reduce
import operator

spots_and_pipes = {(x+1j*y):char for y,line in enumerate(open("day10/day10.txt")) for x,char in enumerate(line) if char not in {'.','\n'}}

pipes = nx.Graph()
propagation_plans = {'|':{1j, -1j}, '-':{1, -1}, 'L':{-1j, 1}, 'J':{-1j, -1}, '7':{1j, -1}, 'F':{1j, 1}}
def propagate(coord):
    for neighbour in propagation_plans.get(spots_and_pipes.get(coord, '.'), set()):
        pipes.add_edge(coord, coord + neighbour)

# connect starting neighbours
start = list(spots_and_pipes.keys())[list(spots_and_pipes.values()).index('S')]
for neighbour in [1, 1j, -1, -1j]:
    propagate(start + neighbour)

# strip any that don't actually connect to the start (may be readded later)
pipes.remove_nodes_from([node for node in pipes.nodes if node != start and node not in pipes[start]])

# convert to an ordered list
nodes_list = [start]
next_node = next(pipes.neighbors(start))
while next_node != start:
    nodes_list.append(next_node)
    propagate(next_node)
    next_node = [node for node in pipes.neighbors(next_node) if node != nodes_list[-2]][0]

# list is total circuit of pipes, so half the length is furthest pipe
print(len(nodes_list) // 2)

# shoelace formula (get the total enclosed area)
area = 0.5 * abs(reduce(operator.add, 
                    (first.real * second.imag - first.imag * second.real for first,second in 
                     ((nodes_list[x], nodes_list[(x+1)%len(nodes_list)]) for x in range(len(nodes_list))))))

# use pick's theorem to get interior points
print(int(area) + 1 - len(nodes_list) // 2)