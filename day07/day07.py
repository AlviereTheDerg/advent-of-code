
from functools import cmp_to_key, reduce
import operator

def classify(card):
    counts = dict()
    for char in card:
        counts[char] = 1 + counts.get(char, 0)
    counts = list(counts.values())
    counts.sort()
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
        
    return -1
input = {card:{"bid":bid, "class":classify(card)} for card,bid in (item.split() for item in open("day07/day07.txt"))}
#print(input)

char_ranks = ['A','K','Q','J','T','9','8','7','6','5','4','3','2']
def compare(card1, card2):
    relative = input[card1]["class"] - input[card2]["class"]
    if relative != 0:
        return relative
    #first nonmatching character -> grade
    for char1,char2 in zip(card1, card2):
        if char1 != char2:
            break
    else:
        return 0
    
    return char_ranks.index(char2) - char_ranks.index(char1)
    
    
card_list = list(input.keys())
card_list.sort(key=cmp_to_key(compare))
print(reduce(operator.add, (index*int(input[card]["bid"]) for index,card in enumerate(card_list, start=1))))