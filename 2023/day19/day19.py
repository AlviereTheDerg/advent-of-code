
from copy import deepcopy

file = open("day19/day19.txt")
def process(workflow, item):
    for destination, determinator in workflow:
        if determinator == True:
            return [(destination,item)]
        category, comparator, bound = determinator
        if comparator == '<':
            if item[category] < bound:
                return [(destination,item)]
        else:
            if item[category] > bound:
                return [(destination,item)]

workflows = dict()
while (line := file.readline()) != '\n':
    label = line[:line.find('{')]
    rules = []
    for rule in line[line.find('{')+1:-2].split(','):
        if ':' not in rule:
            rules.append((rule, True))
        else:
            destination = rule[rule.find(':')+1:]
            category = rule[0]
            comparator = rule[1]
            bound = int(rule[2:rule.find(':')])
            rules.append((destination, (category, comparator, bound)))
    workflows[label] = rules

item_list = []
while (line := file.readline()):
    item = dict()
    
    for chunk in line[1:-2].split(','):
        item[chunk[0]] = int(chunk[2:])
    item_list.append(item)

def process_all(processing_method, input_items):
    item_map = {"in":input_items}
    while len(item_map.keys() - {'A','R'}) > 0:
        this_label = next(iter(item_map.keys() - {'A','R'}))
        for item in item_map.pop(this_label):
            that_stuff = processing_method(workflows[this_label], item)
            for that_label,that_thing in that_stuff:
                if this_label == that_label:
                    print("OH GOD OH FUCK")
                    exit()
                if that_label not in item_map:
                    item_map[that_label] = []
                item_map[that_label].append(that_thing)
    return item_map

print(sum(item['x']+item['m']+item['a']+item['s'] for item in process_all(process, item_list).get('A',[])))

def range_process(workflow, item):
    return_list = []
    for destination, determinator in workflow:
        if determinator == True:
            return_list.append((destination, item))
            return return_list
        category, comparator, bound = determinator
        current_bounds = item[category]
        if comparator == '<':
            if current_bounds[0] < bound and bound < current_bounds[1]:
                low_item = deepcopy(item)
                item[category][0] = bound
                low_item[category][1] = bound-1
                return_list.append((destination, low_item))
            elif current_bounds[1] < bound:
                return_list.append((destination, item))
                return return_list
        else:
            if current_bounds[0] < bound and bound < current_bounds[1]:
                high_item = deepcopy(item)
                item[category][1] = bound
                high_item[category][0] = bound+1
                return_list.append((destination, high_item))
            elif bound < current_bounds[1]:
                return_list.append((destination, item))
                return return_list

result = 0
for item in process_all(range_process, [{'x':[1,4000],'m':[1,4000],'a':[1,4000],'s':[1,4000]}])['A']:
    x = item['x'][1] - item['x'][0] + 1
    m = item['m'][1] - item['m'][0] + 1
    a = item['a'][1] - item['a'][0] + 1
    s = item['s'][1] - item['s'][0] + 1
    result += x*m*a*s
print(result)