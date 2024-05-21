
from functools import reduce
import operator

def parse_card(line: str):
    line = line[line.find(": ")+2:].strip()
    winnings_set, having_set = line.split("|")
    winnings_set = {int(item) for item in winnings_set.split()}
    having_set = {int(item) for item in having_set.split()}
    return len(winnings_set & having_set)
card_data = {index:parse_card(line) for index,line in enumerate(open("day04/day04.txt"), start=1)}

print(reduce(operator.add, (2**(matches-1) for matches in card_data.values() if matches > 0)))

results = {card:1 for card in card_data}
for card,winnings in card_data.items():
    for index in range(card, card+winnings):
        results[index+1] += results[card]
print(reduce(operator.add, results.values()))