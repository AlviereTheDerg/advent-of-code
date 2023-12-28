
import networkx as nx
from collections import deque

map_input = {x+y*1j:char for y,line in enumerate(open("day23/day23.txt")) for x,char in enumerate(line.strip()) if char != '#'}
start = min(map_input, key=lambda coord: coord.imag)
end = max(map_input, key=lambda coord: coord.imag)

snow_island = nx.DiGraph()
for coord,symbol in map_input.items():
    if symbol in {'>','.'}:
        if coord+1 in map_input and map_input[coord+1] != '<':
            snow_island.add_edge(coord, coord+1)
    if symbol in {'v','.'}:
        if coord+1j in map_input and map_input[coord+1j] != '^':
            snow_island.add_edge(coord, coord+1j)
    if symbol in {'<','.'}:
        if coord-1 in map_input and map_input[coord-1] != '>':
            snow_island.add_edge(coord, coord-1)
    if symbol in {'^','.'}:
        if coord-1j in map_input and map_input[coord-1j] != 'v':
            snow_island.add_edge(coord, coord-1j)

# assumption: this collapses the maze to a DAG
distance = 'distance'
nx.set_edge_attributes(snow_island, 1, distance)
for node in list(snow_island.nodes):
    if node not in snow_island:
        continue
    neighbours = set(snow_island.successors(node)) | set(snow_island.predecessors(node))
    if len(neighbours) != 2:
        continue
    left,right = neighbours
    if snow_island.has_edge(left, node) and snow_island.has_edge(node, right):
        snow_island.add_edge(left, right, distance=(snow_island.edges[left,node][distance]+snow_island.edges[node,right][distance]))
    if snow_island.has_edge(right, node) and snow_island.has_edge(node, left):
        snow_island.add_edge(right, left, distance=(snow_island.edges[node,left][distance]+snow_island.edges[right,node][distance]))
    snow_island.remove_node(node)

topological = []
leaves = {node for node in snow_island.nodes if snow_island.out_degree(node) == 0}
snow_island_plucked = nx.DiGraph(snow_island)
while leaves:
    leaf = leaves.pop()
    topological.append(leaf)
    for parent in snow_island_plucked.predecessors(leaf):
        if snow_island_plucked.out_degree(parent) == 1:
            leaves.add(parent)
    snow_island_plucked.remove_node(leaf)

path_lengths = {coord:0 for coord in topological}
for node in reversed(topological):
    for child in snow_island.successors(node):
        path_lengths[child] = max(path_lengths[child], path_lengths[node] + snow_island.edges[node,child][distance])
print(max(path_lengths.values()))

snow_island_slopeless = nx.Graph(snow_island)
longest_path = 0
searching = deque()
searching.append((start, set(), list()))
search_iters = 0
last_find = 0
while searching:
    search_iters += 1
    if search_iters - last_find > 1000000:
        print("Popcorning done")
        break
    node,visited,path = searching.pop()
    visited.add(node)
    path.append(node)
    for child in snow_island_slopeless.neighbors(node):
        if child not in visited:
            searching.append((child, visited.copy(), path.copy()))
    
    if (new_longest_path := sum((snow_island_slopeless.edges[path[index],path[index+1]][distance] for index in range(len(path)-1)))) > longest_path and path[-1] == end:
        longest_path = new_longest_path
        last_find = search_iters
    visited.remove(node)
    path.pop()
print(longest_path)