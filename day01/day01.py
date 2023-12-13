
import re

numeric_strings = ["zero","one","two","three","four","five","six","seven","eight","nine"]
decoding_ring = {v:i for i,v in enumerate(numeric_strings)} | {str(i):i for i in range(10)}

def day01(numeric):
    with open("day01/day01.txt", "r") as input:
        result = 0
        for line in input:
            founds = re.findall(numeric, line)
            result += 10 * decoding_ring.get(founds[0]) + decoding_ring.get(founds[-1])
        print(result)

day01(re.compile(r"\d"))
day01(re.compile(r"(?=({}))".format("|".join(numeric_strings + [r"\d"]))))