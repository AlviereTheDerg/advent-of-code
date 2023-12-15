
r, g, b = "red", "green", "blue"
# parse the input
# dict => game id : game info
# game info => list of hands
# hands => maps of r,g,b to int (number of coloured cubes)
with open("day02/day02.txt", "r") as input:
    result = {i:
                [{r:0, g:0, b:0} | dict(                                    # if a colour doesn't show up in a hand, it defaults to 0
                    tuple(stuff.split(' ')[::-1])                           # separate number and colour, k=colour v=number
                    for stuff in hand.split(', '))                          # hand => list of colour counts ("number colour")
                 for hand in line[line.find(":")+2:].strip().split("; ")]   # "Game number: hand; hand;"" => list of hands
              for i,line in enumerate(input, start=1)}                      # file => game id : game info

possible = {r:12, g:13, b:14}
score = 0
for game in result.items():
    for play in game[1]:
        #compare
        if int(play[r]) > possible[r] or int(play[g]) > possible[g] or int(play[b]) > possible[b]:
            break
    else:
        score += game[0]
print(score)