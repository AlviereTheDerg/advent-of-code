
import networkx as nx

connections = nx.Graph()
for line in open("day25/day25.txt"):
    line = line.strip()
    this_node = line[:line.find(':')]
    connections.add_node(this_node)
    neighbours = line[line.find(':')+2:].split(' ')
    for other_node in neighbours:
        connections.add_edge(this_node, other_node, original=(this_node,other_node))

edge_tuples = {('bqq','rxt'), ('vfx','bgl'), ('qxr','btp')}
for edge in edge_tuples:
    print(edge, connections.has_edge(*edge))
connections.remove_edges_from(edge_tuples)

components = [component for component in nx.connected_components(connections)]
print(len(components), len(components[0]), len(components[1]), len(components[0])*len(components[1]))