
rock_locations = [[]]
for line in open("day13/day13.txt"):
    if line == '\n':
        rock_locations.append([])
    else:
        rock_locations[-1].append(line.strip())

result = 0
for block in rock_locations:
    #prepare special notations
    coords = set()
    rows = {y+1:set() for y in range(len(block))}
    cols = {x+1:set() for x in range(len(block[0]))}
    for y,row in enumerate(block, start=1):
        for x,char in enumerate(row, start=1):
            if char != '#':
                continue
            coords.add((x,y))
            rows[y].add(x)
            cols[x].add(y)
    #print("coords", coords)
    #print("rows", rows)
    #print("cols", cols)

    rows_above = -1
    row_mirror_possibles = [(index,index+1) for index in rows if index+1 in rows and rows[index] == rows[index+1]]
    for possibility in row_mirror_possibles:
        offset = 1
        while possibility[0] - offset in rows and possibility[1] + offset in rows:
            blocked_ups = rows[possibility[0] - offset]
            blocked_downs = rows[possibility[1] + offset]
            if blocked_ups != blocked_downs:
                break
            offset += 1
        else:
            rows_above = possibility[0]
            break
    #print(rows_above)

    cols_left = -1
    col_mirror_possibles = [(index,index+1) for index in cols if index+1 in cols and cols[index] == cols[index+1]]
    for possibility in col_mirror_possibles:
        offset = 1
        while possibility[0] - offset in cols and possibility[1] + offset in cols:
            blocked_lefts = cols[possibility[0] - offset]
            blocked_rights = cols[possibility[1] + offset]
            if blocked_lefts != blocked_rights:
                break
            offset += 1
        else:
            cols_left = possibility[0]
            break
    #print(cols_left)
    #print(row_mirror_possibles)
    #print(col_mirror_possibles)

    result += (cols_left if cols_left != -1 else 0) + 100*(rows_above if rows_above != -1 else 0)
print(result)
