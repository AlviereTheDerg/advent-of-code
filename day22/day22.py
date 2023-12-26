
import networkx as nx
import matplotlib.pyplot as plt

block_coords = dict()
block_ids = dict()
for index,line in enumerate(open("day22/day22.txt")):
    start,end = line.strip().split('~')
    this_block = []
    this_block.append([int(coord) for coord in start.split(',')])
    this_block.append([int(coord) for coord in end.split(',')])
    
    block_ids[index] = set()
    for x in range(this_block[0][0], this_block[1][0]+1):
        for y in range(this_block[0][1], this_block[1][1]+1):
            for z in range(this_block[0][2], this_block[1][2]+1):
                block_coords[(x,y,z)] = index
                block_ids[index].add((x,y,z))

lowest_points = {block_id:min(coord[-1] for coord in block) for block_id,block in block_ids.items()}
fall_order = sorted(lowest_points, key=lowest_points.get)

def descend_coord(coord):
    return (coord[0], coord[1], coord[2] - 1)

stationary_coords = set()
layings = nx.DiGraph()
for index in fall_order:
    this_batch = block_ids[index]
    next_batch = set()
    while min(this_batch, key=lambda item: item[2])[2] > 0:
        next_batch = {descend_coord(coord) for coord in this_batch}
        if len(stationary_coords & next_batch) > 0:
            break
        this_batch = next_batch
    layings.add_node(index)
    for laying_id in {block_coords[coord] for coord in (stationary_coords & next_batch)}:
        layings.add_edge(index, laying_id)
    stationary_coords |= this_batch
    for coord in block_ids[index]:
        block_coords.pop(coord)
    block_ids[index] = this_batch
    block_coords.update({coord:index for coord in this_batch})

deletables = set(node for node in layings)
for node in layings:
    for parent in layings.predecessors(node):
        if layings.out_degree(parent) == 1:
            deletables.remove(node)
            break
print(len(deletables))