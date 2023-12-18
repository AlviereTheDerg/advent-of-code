
from functools import cmp_to_key, reduce
import operator

def classify_pt1(card):
    counts = dict()
    for char in card:
        counts[char] = 1 + counts.get(char, 0)
    counts = sorted(counts[char] for char in counts)
    match counts:
        case [5]:
            return 6
        case [1,4]:
            return 5
        case [2,3]:
            return 4
        case [1,1,3]:
            return 3
        case [1,2,2]:
            return 2
        case [1,1,1,2]:
            return 1
        case [1,1,1,1,1]:
            return 0
        case _:
            print("UH OH")

def classify_pt2(card):
    counts = dict()
    for char in card:
        counts[char] = 1 + counts.get(char, 0)
    counts_sorted = sorted(counts[char] for char in counts if char != 'J')
    if len(counts_sorted) > 0:
        counts_sorted[-1] += counts.get('J',0)
    else:
        counts_sorted.append(counts.get('J',0))
    match counts_sorted:
        case [5]:
            return 6
        case [1,4]:
            return 5
        case [2,3]:
            return 4
        case [1,1,3]:
            return 3
        case [1,2,2]:
            return 2
        case [1,1,1,2]:
            return 1
        case [1,1,1,1,1]:
            return 0
        case _:
            print("UH OH")

def process(hand_classifier, card_rankings):
    def compare(card1, card2):
        relative = hand_classifier(card1) - hand_classifier(card2)
        if relative != 0:
            return relative
        #first nonmatching character -> grade
        for char1,char2 in zip(card1, card2):
            if char1 != char2:
                break
        else:
            return 0
        
        return card_rankings.index(char2) - card_rankings.index(char1)
    comparator = cmp_to_key(compare)
    print(reduce(operator.add, (index*int(bids[card]) for index,card in enumerate(sorted(bids.keys(), key=comparator), start=1))))

bids = {card:bid for card,bid in (item.split() for item in open("day07/day07.txt"))}
process(classify_pt1, ['A','K','Q','J','T','9','8','7','6','5','4','3','2'])
process(classify_pt2, ['A','K','Q','T','9','8','7','6','5','4','3','2','J'])