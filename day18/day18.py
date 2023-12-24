
import networkx as nx
import matplotlib.pyplot as plt

direction_translate = {'R':1, 'D':1j, 'L':-1, 'U':-1j}
input_commands = [(direction_translate[direction],int(distance),colour[1:-1]) for direction,distance,colour in 
                  (line.strip().split(' ') for line in open("day18/day18.txt"))]
#print(input_commands)

dig_graph = nx.Graph()
current_position = 0
for offset,distance,colour in input_commands:
    for _ in range(distance):
        dig_graph.add_edge(current_position, (current_position := current_position + offset), colour=colour)

node_circuit = [0]
node_circuit.append(next(dig_graph.neighbors(0)))
while node_circuit[-1] != 0:
    node_circuit += [node for node in dig_graph.neighbors(node_circuit[-1]) if node != node_circuit[-2]]

area = 0.5 * abs(sum((first.real * second.imag - first.imag * second.real for first,second in 
                      ((node_circuit[x], node_circuit[(x+1)%len(node_circuit)]) for x in range(len(node_circuit))))))
#print(area)
internals = int(area) + 1 - len(node_circuit) // 2
print(len(node_circuit)-1 + internals)

pos = {coord:(coord.real,-coord.imag) for coord in dig_graph}
nx.draw_networkx_nodes(dig_graph, pos, node_size=10)
nx.draw_networkx_edges(dig_graph, pos)
plt.show()