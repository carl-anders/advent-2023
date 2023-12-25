import networkx as nx
graph = nx.Graph()

for line in open("input/25.txt").read().strip().split('\n'):
    left, right = line.split(": ")
    for to in right.split(" "):
        graph.add_edge(left, to, capacity=1.0)


cuts, (left, right) = nx.minimum_cut(graph, "zsj", "qln")
print("Left graph length: " + str(len(left)))
print("Right graph length: " + str(len(right)))
print("Total: " + str(len(left) * len(right)))


for edge in nx.minimum_edge_cut(graph):
    graph.remove_edge(*edge)
left, right = nx.connected_components(graph)
print("Total: " + str(len(left) * len(right)))
