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

std::vector<long long> mix(std::vector<int> input, int rounds, long long key) {
    std::vector<std::pair<int, int>> mixed;
    for (int i = 0; i < input.size(); i++)
        mixed.push_back(make_pair(i, input[i]));
    
    int old_index, new_index, modul = input.size() - 1;
    std::pair<int,int> value;
    for (int round = 0; round < rounds; round++) {
        for (int i = 0; i < input.size(); i++) {
            value = make_pair(i, input[i]);
            old_index = std::find(mixed.begin(), mixed.end(), value) - mixed.begin();
            mixed.erase(mixed.begin() + old_index);
            new_index = ((key * value.second + old_index) % modul + modul) % modul;
            mixed.insert(mixed.begin() + new_index, value);
        }
    }
    
    std::vector<long long> output;
    for (int i = 0; i < mixed.size(); i++)
        output.push_back(key * mixed[i].second);

    return output;
}

long long extract_coords(std::vector<int> input_list, int rounds, long long key) {
    std::vector<long long> mixed = mix(input_list, rounds, key);
    
    int index, modul = input_list.size();
    long long results = 0;
    
    index = std::find(mixed.begin(), mixed.end(), 0) - mixed.begin();
    for (int value : {1000,2000,3000}) {
        results += mixed[(index + value) % modul];
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

    std::cout << "Part 1: " << extract_coords(input_list, 1, 1) << std::endl;
    std::cout << "Part 2: " << extract_coords(input_list, 10, 811589153) << std::endl;
    
    return 0;
}