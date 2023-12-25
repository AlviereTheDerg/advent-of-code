
import networkx as nx
import matplotlib.pyplot as plt
from collections import deque

connections = nx.DiGraph()
for line in open("day20/day20.txt"):
    line = line.strip().split(' -> ')
    this_node = line[0]
    if this_node == 'broadcaster':
        behavior = this_node
    else:
        behavior = this_node[0]
        this_node = this_node[1:]
    connections.add_node(this_node, behavior=behavior)
    for neighbour in line[1].split(', '):
        connections.add_edge(this_node, neighbour)

# setup
status = dict()
for node in connections.nodes():
    match connections.nodes[node].get('behavior'):
        case '%':
            status[node] = 0
        case '&':
            this_node_status = dict()
            for neighbour in connections.predecessors(node):
                this_node_status[neighbour] = 0
            status[node] = this_node_status
        case _:
            pass

def button_press():
    sent = {0:1, 1:0}
    propagation_queue = deque()
    propagation_queue.append(('broadcaster', 0, 'button'))
    while propagation_queue:
        reciever,signal,sender = propagation_queue.popleft()
        #print(sender,signal,reciever)
        match connections.nodes[reciever].get('behavior'):
            case '%':
                if signal == 1:
                    continue
                status[reciever] = status[reciever] ^ 1
                sending = status[reciever]
            case '&':
                status[reciever][sender] = signal
                if set(status[reciever].values()) == {1}:
                    sending = 0
                else:
                    sending = 1
            case 'broadcaster':
                sending = signal
            case _:
                continue
        for downstream in connections[reciever]:
            sent[sending] += 1
            propagation_queue.append((downstream, sending, reciever))
    return sent

#pos = nx.spring_layout(connections)
#nx.draw_networkx_nodes(connections, pos)
#nx.draw_networkx_edges(connections, pos)
#nx.draw_networkx_labels(connections, pos)
#plt.show()

total_presses = {0:0,1:0}
for _ in range(1000):
    for signal,count in button_press().items():
        total_presses[signal] += count
print(total_presses)
print(total_presses[0] * total_presses[1])