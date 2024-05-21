
import networkx as nx
import matplotlib.pyplot as plt
from collections import deque
from copy import deepcopy

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

def initialize(graph: nx.DiGraph):
    status = dict()
    for node in graph.nodes():
        match graph.nodes[node].get('behavior'):
            case '%':
                status[node] = 0
            case '&':
                this_node_status = dict()
                for neighbour in graph.predecessors(node):
                    this_node_status[neighbour] = 0
                status[node] = this_node_status
            case _:
                pass
    return status

def button_press(status, graph):
    sent = {0:1, 1:0}
    propagation_queue = deque()
    propagation_queue.append(('broadcaster', 0, 'button'))
    while propagation_queue:
        reciever,signal,sender = propagation_queue.popleft()
        #print(sender,signal,reciever)
        match graph.nodes[reciever].get('behavior'):
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
                graph.add_node(reciever, recieved=graph.nodes[reciever].get('recieved',set())|{signal})
        for downstream in graph[reciever]:
            sent[sending] += 1
            propagation_queue.append((downstream, sending, reciever))
    return sent

total_presses = {0:0,1:0}
status = initialize(connections)
for _ in range(1000):
    for signal,count in button_press(status, connections).items():
        total_presses[signal] += count
#print(total_presses)
print(total_presses[0] * total_presses[1])

con_parent = next(connections.predecessors("rx"))
predecessor_graphs = dict()
predecessors = set(connections.predecessors(con_parent))
total_presses = 1
for predecessor in predecessors:
    predecessor_graph = deepcopy(connections)
    purge_list = deque(predecessors - {predecessor})

    while purge_list:
        next_purge = purge_list.popleft()
        if next_purge not in predecessor_graph:
            continue
        for parent in predecessor_graph.predecessors(next_purge):
            if parent == 'broadcaster':
                continue
            purge_list.append(parent)
        predecessor_graph.remove_node(next_purge)
    parent = next(predecessor_graph.predecessors(predecessor))
    predecessor_graph.add_edge(parent, "rx")
    predecessor_graph.remove_nodes_from((con_parent, predecessor))

    status = initialize(predecessor_graph)
    presses = 0
    while 0 not in predecessor_graph.nodes["rx"].get('recieved'):
        presses += 1
        button_press(status, predecessor_graph)
    #print(presses)
    total_presses *= presses
print(total_presses)