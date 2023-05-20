#include <iostream>
#include <fstream>
using namespace std;

void dupe_free_append(std::string &buffer, char signal) {
    buffer = buffer.substr(buffer.find(signal) + 1);
    buffer.push_back(signal);
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

    int result_part1 = 0;//, result_part2 = 0;
    std::string signal;
    for (int i = 0; i < line.length(); i++) {
        if (signal.length() >= 4) {
            result_part1 = i;
            break;
        }
        dupe_free_append(signal, line[i]);
    }

    std::cout << "Part 1: " << result_part1 << std::endl;
    //std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}