
rock_locations = [[]]
for line in open("day13/day13.txt"):
    if line == '\n':
        rock_locations.append([])
    else:
        rock_locations[-1].append(line.strip())

rock_maps = []
for block in rock_locations:
    rows = {y+1:set() for y in range(len(block))}
    cols = {x+1:set() for x in range(len(block[0]))}
    for y,row in enumerate(block, start=1):
        for x,char in enumerate(row, start=1):
            if char != '#':
                continue
            rows[y].add(x)
            cols[x].add(y)
    rock_maps.append((rows, cols))

def find_mirrored(mirror_map, start_smudges):
    for possibility in ((index,index+1) for index in mirror_map if index+1 in mirror_map):
        smudges = start_smudges
        offset = 0
        while possibility[0] - offset in mirror_map and possibility[1] + offset in mirror_map:
            blocked_ups = mirror_map[possibility[0] - offset]
            blocked_downs = mirror_map[possibility[1] + offset]
            smudges -= len(blocked_ups ^ blocked_downs)
            if smudges < 0:
                break
            offset += 1
        else:
            if smudges == 0:
                return possibility[0]
    return 0

print(sum(100*find_mirrored(rows, 0) + find_mirrored(cols, 0) for rows,cols in rock_maps))
print(sum(100*find_mirrored(rows, 1) + find_mirrored(cols, 1) for rows,cols in rock_maps))