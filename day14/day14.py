
stationary_rocks = set()
rolling_rocks = dict()
total_row = 0
width = 0
for line in open("day14/day14.txt"):
    width = max(width, len(line.strip()))
    rolling_rocks[total_row] = set()
    for x,char in enumerate(line.strip()):
        match char:
            case '#':
                stationary_rocks.add((x,total_row))
            case 'O':
                rolling_rocks[total_row].add(x)
    total_row += 1

for row in rolling_rocks:
    for rock in list(rolling_rocks[row]):
        rolling_rocks[row].remove(rock)
        new_row = row - 1
        while new_row >= 0:
            if (rock,new_row) in stationary_rocks:
                break
            if rock in rolling_rocks[new_row]:
                break
            new_row -= 1
        rolling_rocks[new_row+1].add(rock)

result = 0
for row in rolling_rocks:
    result += (total_row - row) * len(rolling_rocks[row])
print(result)