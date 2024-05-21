#include <iostream>
#include <fstream>
#include <list>
using namespace std;

int gather_calories(ifstream& input) {
    int calories = 0;
    char buffer[32];
    while (!input.getline(buffer, 32).fail() && std::atoi(buffer) != 0) {
        calories += std::atoi(buffer);
    }
    return calories;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }

    std::list<int> elves;
    while (!input.eof()) {
        elves.push_back(gather_calories(input));
    }
    input.close();

    elves.sort();
    int highest = elves.back();

    int highest_three = 0;
    std::list<int>::iterator iter = elves.end();
    for (int i = 0; i < 3; i++) { highest_three += *(--iter); }

    std::cout << "Part 1: " << highest << std::endl;
    std::cout << "Part 2: " << highest_three << std::endl;

    return 0;
}