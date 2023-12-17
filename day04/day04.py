
def parse_card(line: str):
    line = line[line.find(": ")+2:].strip()
    winnings_set, having_set = line.split("|")
    winnings_set = {int(item) for item in winnings_set.split()}
    having_set = {int(item) for item in having_set.split()}
    return (winnings_set, having_set)
card_data = {index:parse_card(line) for index,line in enumerate(open("day04/day04.txt"), start=1)}

results = 0
for card in card_data.values():
    this_count = len(card[0] & card[1]) - 1
    if this_count >= 0:
        results += 2**this_count
print(results)