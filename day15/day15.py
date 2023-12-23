
initialization_sequence = open("day15/day15.txt").readline().split(',')

def HASH(command):
    current_value = 0
    for char in command:
        current_value = ((ord(char) + current_value) * 17) % 256
    return current_value

print(sum(HASH(command) for command in initialization_sequence))

boxes = {x:[] for x in range(256)}
for label,instruction in ((command[:-1],'-') if command[-1] == '-' else command.split('=') for command in initialization_sequence):
    box = boxes[HASH(label)]
    index = -1
    for lens_index in range(len(box)):
        if box[lens_index][0] == label:
            index = lens_index
            break
    
    if index != -1 and instruction == '-':
        box.pop(index)
    elif index != -1:
        box[index][1] = instruction
    elif instruction != '-':
        box.append([label, instruction])

focusing_power = 0
for box_index in boxes:
    for index in range(len(boxes[box_index])):
        focusing_power += (box_index + 1) * (index + 1) * int(boxes[box_index][index][1])
print(focusing_power)