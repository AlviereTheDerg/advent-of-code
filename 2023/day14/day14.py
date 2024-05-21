
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

def get_load():
    return sum((total_row - int(rock.imag) for rock in rolling_rocks))

shift(-1j)
print(get_load())

def spin_cycle():
    shift(-1j)
    shift(-1)
    shift(1j)
    shift(1)

def draw():
    result_map = []
    for y in range(total_row):
        result_map.append([])
        for x in range(width):
            if x+y*1j in stationary_rocks:
                result_map[-1].append('#')
            elif x+y*1j in rolling_rocks:
                result_map[-1].append('O')
            else:
                result_map[-1].append('.')
        result_map[-1] = "".join(result_map[-1])
    print("\n".join(result_map))

current_spin = 0
max_spins = 1000000000
spin_map = dict()
while current_spin < max_spins:
    frozen_rocks = frozenset(rolling_rocks)
    if frozen_rocks in spin_map:
        difference = current_spin - spin_map[frozen_rocks]
        if (max_spins - current_spin) % difference == 0:
            break
    spin_map[frozen_rocks] = current_spin
    current_spin += 1
    spin_cycle()
print(get_load())