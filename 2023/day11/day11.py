

stars = [(x+y*1j) for y,line in enumerate(open("day11/day11.txt")) for x,char in enumerate(line) if char == '#']

# ID empty rows/columns, construct maps
rows = {int(coord.imag) for coord in stars}
row_map = dict()
count = 0
for row in range(max(rows)+1):
    if row in rows:
        row_map[row] = count
    else:
        count += 1

columns = {int(coord.real) for coord in stars}
column_map = dict()
count = 0
for column in range(max(columns)+1):
    if column in columns:
        column_map[column] = count
    else:
        count += 1

def expand(coord, expansion_coefficient):
    return coord.real + (expansion_coefficient - 1) * column_map[coord.real] + 1j * (coord.imag + (expansion_coefficient - 1) * row_map[coord.imag])

result = 0
for first_index in range(len(stars)):
    for second_index in range(first_index, len(stars)):
        distance = expand(stars[first_index], 2) - expand(stars[second_index], 2)
        result += int(abs(distance.real) + abs(distance.imag))
print(result)

result = 0
for first_index in range(len(stars)):
    for second_index in range(first_index, len(stars)):
        distance = expand(stars[first_index], 1000000) - expand(stars[second_index], 1000000)
        result += int(abs(distance.real) + abs(distance.imag))
print(result)