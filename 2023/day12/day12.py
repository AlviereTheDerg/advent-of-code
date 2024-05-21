
from tqdm import tqdm
import time

def compress(record_string: str):
    result_list = []
    for char in record_string.strip('.'):
        if char == '.' and result_list[-1] == '.':
            continue
        result_list.append(char)
    return "".join(result_list)

records = [(row[0], [int(item) for item in row[1].split(',')]) for row in (line.split() for line in open("day12/day12.txt"))]

def recursive_nonagram_row(record, notes, memo=dict()):
    if (record,notes) in memo:
        return memo[record,notes]
    if ((len(notes) == 0 and (len(record) == 0 or '#' not in record))
        or (len(notes) == 1 and len(record) == notes[0] and '.' not in record)):
            return 1
    elif len(notes) == 0 or (len(notes) > 0 and len(record) == 0):
        return 0
    results = 0
    for index in range(len(record) - sum(notes[1:]) - len(notes[1:])):
        if (index+notes[0] <= len(record) and '.' not in record[index:index+notes[0]]) and (index+notes[0] == len(record) or record[index+notes[0]] != '#'):
            results += recursive_nonagram_row(compress(record[index+notes[0]+1:]), notes[1:], memo)
        if record[index] == '#':
            break
    memo[record,notes] = results
    return results

def iterative_nonagram_row(record, notes):
    blocks_and_paths = {0:1}
    for note_index, note_value in enumerate(notes):
        new_blocks_and_paths = dict()
        for block_start, paths_count in blocks_and_paths.items():
            for index in range(block_start, len(record) - sum(notes[note_index + 1:]) - len(notes[note_index + 1:])):
                if ((index+note_value <= len(record) and '.' not in record[index:index+note_value]) 
                    and (index+note_value == len(record) or record[index+note_value] != '#') 
                    and (note_index < len(notes) - 1 or '#' not in record[index + note_value + 1:])):
                    new_blocks_and_paths[index+note_value+1] = paths_count + new_blocks_and_paths.get(index+note_value+1,0)
                if record[index] == '#':
                    break
        blocks_and_paths = new_blocks_and_paths
    return sum(blocks_and_paths.values())

result = 0
start_time = time.time()
for record, notes in records:
    result += recursive_nonagram_row(compress(record), tuple(notes))
end_time = time.time()
print(end_time-start_time)
print(result)

result_2 = 0
start_time = time.time()
for record, notes in records:
    result_2 += iterative_nonagram_row(compress(record), notes)
end_time = time.time()
print(end_time-start_time)
print(result_2)

second_result = 0
start_time = time.time()
for record,notes in tqdm(records):
    second_result += recursive_nonagram_row(compress("?".join([record]*5)), tuple(notes*5))
end_time = time.time()
print(end_time-start_time)
print(second_result)

second_result_2 = 0
start_time = time.time()
for record, notes in records:
    second_result_2 += iterative_nonagram_row(compress("?".join([record]*5)), notes*5)
end_time = time.time()
print(end_time-start_time)
print(second_result_2)