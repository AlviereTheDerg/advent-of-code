
x,m,a,s = "xmas"
file = open("day19/day19.txt")
workflows = dict()
while (line := file.readline()) != '\n':
    label = line[:line.find('{')]
    def generate_rules(line):
        rules = []
        for rule in line[line.find('{')+1:-2].split(','):
            if ':' not in rule:
                rules.append((lambda item: True, rule))
            else:
                rules.append((lambda item, rule=rule: 
                                item[rule[0]] < int(rule[2:rule.find(':')]) 
                                if rule[1]=='<' else 
                                item[rule[0]] > int(rule[2:rule.find(':')]), 
                              rule[rule.find(':')+1:]))
        def process(item):
            for rule,destination in rules:
                if rule(item):
                    return destination
        return process
    workflows[label] = generate_rules(line)

item_list = []
while (line := file.readline()):
    item_list.append(eval(line.strip().replace('=', ':')))

item_map = {"in":item_list}
while len(item_map.keys() - {'A','R'}) > 0:
    this_label = next(iter(item_map.keys() - {'A','R'}))
    for item in item_map.pop(this_label):
        that_label = workflows[this_label](item)
        if this_label == that_label:
            print("OH GOD OH FUCK")
            exit()
        if that_label not in item_map:
            item_map[that_label] = []
        item_map[that_label].append(item)

#print("Accepted:", item_map.get('A', None))
#print("Rejected:", item_map.get('R', None))
print(sum(item[x]+item[m]+item[a]+item[s] for item in item_map.get('A',[])))