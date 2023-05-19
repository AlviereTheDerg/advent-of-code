#include <iostream>
#include <fstream>
#include <algorithm>
#include <vector>
using namespace std;

char examine_rucksack(std::string rucksack) {
    char result;
    int compartment_size = rucksack.length() / 2;

    std::sort(rucksack.begin(), rucksack.begin() + compartment_size);
    std::sort(rucksack.begin() + compartment_size, rucksack.end());
    
    std::set_intersection(rucksack.begin(), rucksack.begin() + compartment_size, rucksack.begin() + compartment_size, rucksack.end(), &result);
    return result;
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

    std::string rucksack;
    char current_find;
    int score = 0;
    while (getline(input, rucksack)) {
        current_find = examine_rucksack(rucksack);
        score += find_priority(current_find);
    }
    input.close();

    std::cout << "Part 1: " << score << std::endl;
    return 0;
}