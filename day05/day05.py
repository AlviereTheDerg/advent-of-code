
rule_blocks = []
seeds = None
for line in open("day05/day05.txt"):
    if seeds == None:
        seeds = [int(item) for item in line[line.find(":")+2:].split()]
        continue
    if line == '\n':
        rule_blocks.append(list())
        continue
    rule_blocks[-1].append(line.strip())
    if rule_blocks[-1][-1][0] in {'0','1','2','3','4','5','6','7','8','9'}:
        rule_blocks[-1][-1] = [int(val) for val in rule_blocks[-1][-1].split()]

def process_seeds(seed_data):
    end = seed_data
    for rule_block in rule_blocks:
        start = end
        end = dict()
        for dest_range,source_range,range_length in rule_block[1:]:
            source_bounds = (source_range, source_range+range_length)
            for item in list(start):
                item_bounds = (item, item + start[item])
                marks = sorted([item_bounds[0], item_bounds[1], source_bounds[0], source_bounds[1]])
                chunks = list(zip(marks[:-1], marks[1:]))

                shifting = False
                storing = False
                for chunk in chunks:
                    if chunk[0] in source_bounds:
                        shifting = not shifting
                    if chunk[0] in item_bounds:
                        storing = not storing
                        if not storing:
                            break
                    
                    if not storing:
                        continue

                    if shifting:
                        start.pop(chunk[0], None)
                        end[chunk[0] - source_range + dest_range] = chunk[1] - chunk[0]
                    else:
                        start[chunk[0]] = chunk[1] - chunk[0]
        end.update(start)
    print(min(end))

process_seeds({item:0 for item in seeds})
process_seeds(dict(zip(seeds[0::2], seeds[1::2])))