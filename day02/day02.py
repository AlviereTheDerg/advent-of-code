
from functools import reduce

r, g, b = "red", "green", "blue"
# parse the input
# dict => game id : info
# each game compressed down to highest count of each colour in the hand
with open("day02/day02.txt", "r") as input:
    def process_game(hands):
        raws = (tuple(stuff.split(' ')[::-1])for hand in hands for stuff in hand.split(', '))
        game = {r:0, g:0, b:0}
        for colour, count in raws:
            game[colour] = max(game[colour], int(count))
        return game
    
    result = {i:process_game(line[line.find(":")+2:].strip().split("; ")) for i,line in enumerate(input, start=1)}

# part 1
possible = {r:12, g:13, b:14}
print(reduce(lambda a,b: a+b, 
             (index for index, play in result.items() 
              if (play[r] <= possible[r] and play[g] <= possible[g] and play[b] <= possible[b]))))

# part 2
print(reduce(lambda a,b: a+b, 
             (reduce(lambda c,d: c*d, play.values()) 
              for play in result.values())))