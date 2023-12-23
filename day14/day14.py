
stationary_rocks = set()
rolling_rocks = set()
total_row = 0
width = 0
for line in open("day14/day14.txt"):
    width = max(width, len(line.strip()))
    for x,char in enumerate(line.strip()):
        match char:
            case '#':
                stationary_rocks.add(x + total_row*1j)
            case 'O':
                rolling_rocks.add(x + total_row*1j)
    total_row += 1

def shift(direction_offset):
    for rock in list(rolling_rocks):
        rolling_rocks.remove(rock)
        while rock.real >= 0 and rock.real < total_row and rock.imag >= 0 and rock.imag < width and rock not in stationary_rocks:
            if rock not in rolling_rocks:
                room_to_go = rock
            rock += direction_offset
        rolling_rocks.add(room_to_go)

result = 0
shift(-1j)
for rock in rolling_rocks:
    result += total_row - int(rock.imag)
print(result)