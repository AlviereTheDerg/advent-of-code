#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
using namespace std;

std::vector<int> parse_input(ifstream &input) {
    std::vector<int> signal(1, 0);
    int regX = 1, state = 0, cycle = 0;
    std::string arg;

    while (input >> arg) {
        signal.push_back(regX);
        switch(state) {
            case 0:
            if (arg.compare("noop") == 0)
                state = 0; //no change
            else
                state = 1; //go to taking int for addx
            break;

            case 1:
            regX += std::stoi(arg);
            state = 0;
            break;
        }
    }
    signal.push_back(regX);
    return signal;
}

int tally_signal(std::vector<int> signal) {
    int sum = 0;
    for (int index = 0; index < signal.size(); index++)
        sum += (index % 40 == 20) ? index * signal[index] : 0;
    return sum;
}

int main() {ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    std::vector<int> signal = parse_input(input);
    input.close();

    int result_part1 = tally_signal(signal);
    int result_part2 = 0;

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}