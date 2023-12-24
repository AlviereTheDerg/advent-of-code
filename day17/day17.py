
from heapq import heappop, heappush

heat_loss = {(x+y*1j):int(char) for y,line in enumerate(open("day17/day17.txt")) for x,char in enumerate(line.strip())}

source = 0
destination = max(heat_loss, key=abs)
tie_breaker = 0
straight_line_max = 4
search_heap = [(0,0,source,1,{source}), (0,0,source,1j,{source})]
# heat dissipation of this path, tie-breaker, location, current travel direction, nodes in path
overall_seen = set()
# coordinate, direction, time until direction change
while len(search_heap) > 0:
    heat, _, coord, direction, visited = heappop(search_heap)
    if coord == destination:
        print(heat)
        nodes_traversed = visited
        break
    
    if (coord, direction) in overall_seen:
        continue
    overall_seen.add((coord, direction))

    match direction:
        case 1 | -1:
            directions = {1j, -1j}
        case 1j | -1j:
            directions = {1, -1}

    for direction in directions:
        heat_diff = 0
        extra_visited = set()
        for distance in range(1,straight_line_max):
            neighbour = coord + distance * direction
            if neighbour not in heat_loss or neighbour in visited:
                continue
            heat_diff += heat_loss[neighbour]
            extra_visited.add(neighbour)
            heappush(search_heap, (heat + heat_diff, (tie_breaker := tie_breaker + 1), neighbour, direction, visited | extra_visited))


"""import networkx as nx
import matplotlib.pyplot as plt
factory_graph = nx.Graph()
factory_graph.add_nodes_from(coord for coord in heat_loss)
factory_graph.add_edges_from((coord,coord+direction) for direction in [1,1j,-1,-1j] for coord in heat_loss if coord+direction in heat_loss)
pos = {coord:(coord.real, -coord.imag) for coord in heat_loss}
nx.draw_networkx_nodes(factory_graph, pos, nodes_traversed, node_color = 'red')
nx.draw_networkx_nodes(factory_graph, pos, heat_loss.keys() - nodes_traversed)
nx.draw_networkx_edges(factory_graph, pos)
nx.draw_networkx_labels(factory_graph, pos, heat_loss)
plt.show()#"""