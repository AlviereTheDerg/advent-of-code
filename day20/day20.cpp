#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
using namespace std;

std::vector<int> read_input(ifstream &input) {
    std::vector<int> results;
    std::string line;
    while (getline(input, line))
        results.push_back(std::stoi(line));
    return results;
}

std::vector<int> mix(std::vector<int> input) {
    std::vector<std::pair<int,int>> mixed;
    for (int i = 0; i < input.size(); i++)
        mixed.push_back(make_pair(i, input[i]));
    
    int old_index, new_index, modul = input.size() - 1;
    std::pair<int,int> value;
    for (int i = 0; i < input.size(); i++) {
        value = make_pair(i, input[i]);
        old_index = std::find(mixed.begin(), mixed.end(), value) - mixed.begin();
        mixed.erase(mixed.begin() + old_index);
        new_index = ((old_index + value.second) % modul + modul) % modul;
        mixed.insert(mixed.begin() + new_index, value);
    }

    for (int i = 0; i < input.size(); i++) {
        input[i] = mixed[i].second;
    }

    return input;
}

int part1_calculate(std::vector<int> mixed_input) {
    int results = 0, index, modul = mixed_input.size();
    index = std::find(mixed_input.begin(), mixed_input.end(), 0) - mixed_input.begin();
    for (int value : {1000,2000,3000}) {
        results += mixed_input[(index + value) % modul];
    }
    return results;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::vector<int> input_list = read_input(input);
    input.close();

    std::vector<int> one_mix = mix(input_list);
    std::cout << "Part 1: " << part1_calculate(one_mix) << std::endl;
    
    return 0;
}