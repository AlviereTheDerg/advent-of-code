#include <iostream>
#include <fstream>
using namespace std;

void dupe_free_append(std::string &buffer, char signal) {
    buffer = buffer.substr(buffer.find(signal) + 1);
    buffer.push_back(signal);
}

int find_dupe_free_segment(std::string input, int segment_length) {
    std::string signal;
    for (int i = 0; i < input.length(); i++) {
        if (signal.length() >= segment_length)
            return i;
        dupe_free_append(signal, input[i]);
    }
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    std::string line;
    input >> line;
    input.close();

    int result_part1 = 0, result_part2 = 0;
    result_part1 = find_dupe_free_segment(line, 4);
    result_part2 = find_dupe_free_segment(line, 14);

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}