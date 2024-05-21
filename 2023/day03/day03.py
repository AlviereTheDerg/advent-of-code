
import networkx as nx
import matplotlib.pyplot as plt
from functools import reduce
schematic = nx.Graph()

raws = {(x + y*1j):item for y,line in enumerate(open("day03/day03.txt")) for x,item in enumerate(line) if item not in {'.', '\n'}}
#print(raws)

number_chars = {'0','1','2','3','4','5','6','7','8','9'}
parts = {coord:item for coord,item in raws.items() if item not in number_chars}
gears = {coord:item for coord,item in parts.items() if item == '*'}
#print(parts)

possible_neighbours = [(a+b*1j) for a in [-1,0,1] for b in [-1,0,1] if a != 0 or b != 0]
schematic.add_nodes_from(((coord, dict(contents=item)) for coord,item in raws.items()))
for node in parts:
    for direction in possible_neighbours:
        if schematic.has_node(node+direction) and schematic.nodes[node+direction]["contents"] in number_chars:
            schematic.add_edge(node, node+direction)

checked = set()
numbers = set()
for coord,item in raws.items():
    if item not in number_chars or coord in checked:
        continue
    new_discovery = [coord]

    while (new_discovery[0] - 1) in raws and raws[new_discovery[0] - 1] in number_chars:
        new_discovery.insert(0, new_discovery[0] - 1)
    
    while (new_discovery[-1] + 1) in raws and raws[new_discovery[-1] + 1] in number_chars:
        new_discovery.append(new_discovery[-1] + 1)
    
    checked.update(new_discovery)
    neighbours = {neighbour for discovery in new_discovery for neighbour in schematic[discovery]}
    schematic.remove_nodes_from(new_discovery)
    new_node = (new_discovery[0] + new_discovery[-1]) / 2
    numbers.add(new_node)
    schematic.add_node(new_node, contents=int("".join((raws[coord] for coord in new_discovery))))
    schematic.add_edges_from(((new_node,neighbour) for neighbour in neighbours))

result = 0
for number in numbers:
    if len(parts.keys() & schematic[number]) > 0:
        result += schematic.nodes[number]["contents"]
print(result)

second_result = 0
for gear in gears:
    if len(schematic[gear]) == 2:
        second_result += reduce(lambda a,b: a*b, (schematic.nodes[item]["contents"] for item in schematic[gear]))
print(second_result)