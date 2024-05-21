#include <iostream>
#include <fstream>
#include <algorithm>
#include <vector>
using namespace std;

std::string rucksack_intersection(std::string::iterator sack1_start, std::string::iterator sack1_end, std::string::iterator sack2_start, std::string::iterator sack2_end) {
    std::sort(sack1_start, sack1_end);
    std::sort(sack2_start, sack2_end);
    
    std::vector<char> result(sack1_end - sack1_start);
    std::vector<char>::iterator iter = std::set_intersection(sack1_start, sack1_end, sack2_start, sack2_end, result.begin());
    result.resize(iter - result.begin());
    return string(result.begin(), result.end());
}

char examine_rucksack(std::string rucksack) {
    int compartment_size = rucksack.length() / 2;

    return rucksack_intersection(rucksack.begin(), rucksack.begin() + compartment_size, rucksack.begin() + compartment_size, rucksack.end())[0];
}

int find_priority(char item) {
    return item + 1 - ((isupper(item)) ? 'A' - 26 : 'a');
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }

    std::string rucksack, elf_group;
    int score_part1 = 0, score_part2 = 0, elf_count = 0;
    while (getline(input, rucksack)) {
        score_part1 += find_priority(examine_rucksack(rucksack));

        switch ((elf_count++) % 3) {
            case 0: elf_group = rucksack; break;
            case 1: elf_group = rucksack_intersection(elf_group.begin(), elf_group.end(), rucksack.begin(), rucksack.end()); break;
            case 2: score_part2 += find_priority(rucksack_intersection(elf_group.begin(), elf_group.end(), rucksack.begin(), rucksack.end())[0]); break;
        }
    }
    input.close();

    std::cout << "Part 1: " << score_part1 << std::endl;
    std::cout << "Part 2: " << score_part2 << std::endl;
    return 0;
}