
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
#print(x,y)

directions = {1, 1j, -1, -1j}
def coord_round(coord):
    return coord.real%(x+1) + (coord.imag%(y+1))*1j
def map_steps(step_count):
    cached_elves = {starting_point} if step_count%2 == 0 else set()
    elves = {starting_point}
    prev_elves = set()
    for this_step in range(step_count):
        next_elves = set()
        for elf in elves:
            for direction in directions:
                if coord_round(elf+direction) in plots and elf+direction not in prev_elves:
                    next_elves.add(elf+direction)
        prev_elves = elves
        elves = next_elves
        if this_step % 2 == (step_count+1) % 2:
            cached_elves |= elves
    return len(cached_elves)

print(map_steps(64))

final_goal = 26501365
a0 = map_steps(final_goal % (x+1))
a1 = map_steps(final_goal % (x+1) + (x+1))
a2 = map_steps(final_goal % (x+1) + (x+1) * 2)
truncated_goal = final_goal // (x+1)
print(a0 + (a1 - a0) * truncated_goal + (a2 - 2*a1 + a0) * (truncated_goal*(truncated_goal-1)//2))