
starting_point = 0
plots = set()
for y,line in enumerate(open("day21/day21.txt")):
    for x,char in enumerate(line.strip()):
        match char:
            case '#':
                pass
            case 'S':
                starting_point = x + y * 1j
                plots.add(starting_point)
            case '.':
                plots.add(x + y * 1j)

directions = {1, 1j, -1, -1j}
def map_steps(step_count):
    elves = {starting_point}
    for _ in range(step_count):
        next_elves = set()
        for elf in elves:
            for direction in directions:
                if elf+direction in plots:
                    next_elves.add(elf+direction)
        elves = next_elves
    return len(elves)

print(map_steps(64))