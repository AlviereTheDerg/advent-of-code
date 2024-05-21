
import networkx as nx

item_rules = {
    '.':lambda x: [x],
    '/':lambda x: [-x.imag - x.real * 1j],
    '\\':lambda x: [x.imag + x.real * 1j],
    '|':lambda x: [x] if x.real == 0 else [x.real * 1j, -x.real * 1j],
    '-':lambda x: [x] if x.imag == 0 else [x.imag, -x.imag]
}

contraption_items = {(x+y*1j):char for y,line in enumerate(open("day16/day16.txt")) for x,char in enumerate(line.strip())}
lower_edge = int(max(coord.imag for coord in contraption_items))
right_edge = int(max(coord.real for coord in contraption_items))

def energized(destination, travel_direction):
    graph = nx.DiGraph()
    search_stack = [(destination, next_travel_direction) for next_travel_direction in item_rules[contraption_items[destination]](travel_direction)]
    while len(search_stack) > 0:
        destination, travel_direction = search_stack.pop()
        if destination + travel_direction not in contraption_items:
            continue # if it would go off the board
        if graph.has_edge(destination, destination+travel_direction):
            continue # if it's already gone this way before
        graph.add_edge(destination, destination + travel_direction)
        destination += travel_direction
        search_stack += [(destination, next_travel_direction) for next_travel_direction in item_rules[contraption_items[destination]](travel_direction)]
    return graph.number_of_nodes()

print(energized(0, 1))

print(max(max(energized(index*1j                ,  1 ) for index in range(lower_edge)), 
          max(energized(index                   ,  1j) for index in range(right_edge)), 
          max(energized(right_edge + index*1j   , -1 ) for index in range(lower_edge)), 
          max(energized(index + lower_edge*1j   , -1j) for index in range(right_edge))))