#include <iostream>
#include <fstream>
using namespace std;

void line_to_elfdata(std::string line, int elfdata[4]) {
    int dash1 = line.find('-');
    int comma = line.find(',');
    int dash2 = line.substr(comma, line.length()).find('-') + comma;

    elfdata[0] = stoi(line.substr(0, dash1));
    elfdata[1] = stoi(line.substr(dash1 + 1, comma));

    elfdata[2] = stoi(line.substr(comma + 1, dash2));
    elfdata[3] = stoi(line.substr(dash2 + 1, line.length()));

    return;
}

int detect_overlap(int elfdata[4]) {
    if (elfdata[2] <= elfdata[0] && elfdata[1] <= elfdata[3])
        return 1; //if elf from 0-1 is inside elf from 2-3
    
    if (elfdata[0] <= elfdata[2] && elfdata[3] <= elfdata[1])
        return 1; //if elf from 2-3 is inside elf from 0-1
    
    return 0;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }

    std::string line;
    int score_part1 = 0, score_part2 = 0;
    int elfdata[4];
    while (getline(input, line)) {
        line_to_elfdata(line, elfdata);
        score_part1 += detect_overlap(elfdata);
    }
    input.close();

    std::cout << "Part 1: " << score_part1 << std::endl;
    std::cout << "Part 2: " << score_part2 << std::endl;
    return 0;
}