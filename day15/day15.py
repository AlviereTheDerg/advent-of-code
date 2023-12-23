
initialization_sequence = open("day15/day15.txt").readline().strip().split(',')

def HASH(command):
    current_value = 0
    for char in command:
        current_value = ((ord(char) + current_value) * 17) % 256
    return current_value

print(initialization_sequence)

print(HASH("HASH"))
print(sum(HASH(command) for command in initialization_sequence))