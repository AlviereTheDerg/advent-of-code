
direction_translate = {'R':1, 'D':1j, 'L':-1, 'U':-1j}
input_commands = [(direction_translate[direction],int(distance),colour[1:-1]) for direction,distance,colour in 
                  (line.strip().split(' ') for line in open("day18/day18.txt"))]
#print(input_commands)

node_circuit = [0]
for offset,distance,_ in input_commands:
    for _ in range(distance):
        node_circuit.append(node_circuit[-1] + offset)

area = 0.5 * abs(sum((first.real * second.imag - first.imag * second.real for first,second in 
                      ((node_circuit[x], node_circuit[(x+1)%len(node_circuit)]) for x in range(len(node_circuit))))))
#print(area)
internals = int(area) + 1 - len(node_circuit) // 2
print(len(node_circuit)-1 + internals)