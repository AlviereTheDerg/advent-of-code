
import networkx as nx

item_rules = {
    '.':lambda x: [x],
    '/':lambda x: [-x.imag - x.real * 1j],
    '\\':lambda x: [x.imag + x.real * 1j],
    '|':lambda x: [x] if x.real == 0 else [x.real * 1j, -x.real * 1j],
    '-':lambda x: [x] if x.imag == 0 else [x.imag, -x.imag]
}

contraption_items = {(x+y*1j):char for y,line in enumerate(open("day16/day16.txt")) for x,char in enumerate(line.strip())}

energized = nx.DiGraph()
def spread(destination, travel_direction):
    search_stack = [(destination, next_travel_direction) for next_travel_direction in item_rules[contraption_items[destination]](travel_direction)]
    while len(search_stack) > 0:
        destination, travel_direction = search_stack.pop()
        if destination + travel_direction not in contraption_items:
            continue # if it would go off the board
        if energized.has_edge(destination, destination+travel_direction):
            continue # if it's already gone this way before
        energized.add_edge(destination, destination + travel_direction)
        destination += travel_direction
        search_stack += [(destination, next_travel_direction) for next_travel_direction in item_rules[contraption_items[destination]](travel_direction)]

spread(0, 1)
print(energized.number_of_nodes())