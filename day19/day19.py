
file = open("day19/day19.txt")
def process(workflow, item):
    for destination, determinator in workflow:
        if determinator == True:
            return destination
        category, comparator, bound = determinator
        if comparator == '<':
            if item[category] < bound:
                return destination
        else:
            if item[category] > bound:
                return destination

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

item_map = {"in":item_list}
while len(item_map.keys() - {'A','R'}) > 0:
    this_label = next(iter(item_map.keys() - {'A','R'}))
    for item in item_map.pop(this_label):
        that_label = process(workflows[this_label], item)
        if this_label == that_label:
            print("OH GOD OH FUCK")
            exit()
        if that_label not in item_map:
            item_map[that_label] = []
        item_map[that_label].append(item)

#print("Accepted:", item_map.get('A', None))
#print("Rejected:", item_map.get('R', None))
print(sum(item['x']+item['m']+item['a']+item['s'] for item in item_map.get('A',[])))