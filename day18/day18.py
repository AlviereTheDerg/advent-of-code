
input_commands = [(direction,distance,colour[1:-1]) for direction,distance,colour in 
                  (line.strip().split(' ') for line in open("day18/day18.txt"))]

def get_lagoon_size(dig_instructions):
    node_count = 1
    area = 0
    current_node = 0
    for offset,distance in dig_instructions:
        node_count += distance
        next_node = current_node + offset * distance
        area += current_node.real * next_node.imag - current_node.imag * next_node.real
        current_node = next_node
    
    area = 0.5 * int(area)
    internals = int(area) + 1 - node_count // 2
    print(node_count-1 + internals)

get_lagoon_size(({'R':1, 'D':1j, 'L':-1, 'U':-1j}[offset],int(distance)) for offset,distance,_ in input_commands)
get_lagoon_size(({'0':1, '1':1j, '2':-1, '3':-1j}[colour[-1]],int(colour[1:-1],16)) for _,_,colour in input_commands)