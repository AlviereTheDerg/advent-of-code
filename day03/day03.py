
raws = {(x + y*1j):item for y,line in enumerate(open("day03/day03.txt")) for x,item in enumerate(line) if item not in {'.', '\n'}}
#print(raws)

parts = {coord for coord,item in raws.items() if item not in {'0','1','2','3','4','5','6','7','8','9'}}
#print(parts)

# find sets of numbers
number_raws = {coord:item for coord,item in raws.items() if item in {'0','1','2','3','4','5','6','7','8','9'}}
checked = set()
number_batches = []
for coord in number_raws:
    if coord in checked:
        continue
    new_discovery = [coord]
    
    # check things to the left
    check = coord - 1
    while check in number_raws:
        new_discovery.insert(0, check)
        check -= 1
    
    # check things to the right
    check = coord + 1
    while check in number_raws:
        new_discovery.append(check)
        check += 1
    
    number_batches.append(new_discovery)
    checked.update(new_discovery)

result = 0
#print(number_batches)
for batch in number_batches:
    number = int("".join((number_raws[coord] for coord in batch)))
    #print(batch, number)
    bounding_box = set()
    for coord in batch:
        bounding_box.add(coord + 1j)
        bounding_box.add(coord - 1j)
    bounding_box.add(batch[0] - 1)
    bounding_box.add(batch[0] - 1 - 1j)
    bounding_box.add(batch[0] - 1 + 1j)
    bounding_box.add(batch[-1] + 1)
    bounding_box.add(batch[-1] + 1 - 1j)
    bounding_box.add(batch[-1] + 1 + 1j)
    #print(number, bounding_box)
    if len(bounding_box.intersection(parts)) > 0:
        result += number

print(result)