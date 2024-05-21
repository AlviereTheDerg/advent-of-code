#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <sstream>
using namespace std;

std::array<std::string,10> read_start(ifstream& input) {
    std::array<std::string,10> boxes;
    std::string line;
    while (getline(input, line) && line != "") {
        for (int i = 0; i <= line.length() / 4; i++) {
            if (line[4*i + 1] == ' ')
                continue;
            boxes[i+1].insert(boxes[i+1].begin(), line[4*i + 1]);
        }
    }
    for (int i = 1; i < boxes.size(); i++) {
        boxes[i].erase(boxes[i].begin()); //strip stack numbers
    }
    return boxes;
}

std::array<int, 3> extract_inputs(std::string line) {
    std::array<int, 3> commands;
    std::string foo;
    std::stringstream stream(line);
    stream >> foo >> commands[0] >> foo >> commands[1] >> foo >> commands[2];
    return commands;
}

void move_box(std::array<std::string, 10> &boxes, int source, int dest) {
    if (boxes[source].empty())
        return;
    boxes[dest].push_back(boxes[source].back());
    boxes[source].pop_back();
}

void move_boxes(std::array<std::string, 10> &boxes, std::array<int, 3> commands) {
    for (int i = 0; i < commands[0]; i++) {
        move_box(boxes, commands[1], commands[2]);
    }
}

void shift_boxes(std::array<std::string, 10> &boxes, std::array<int, 3> commands) {
    boxes[commands[2]] += boxes[commands[1]].substr(boxes[commands[1]].size()-commands[0]);
    boxes[commands[1]] = boxes[commands[1]].substr(0, boxes[commands[1]].size()-commands[0]);
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }

    std::array<std::string,10> boxes_part1 = read_start(input);
    std::array<std::string,10> boxes_part2 = boxes_part1;
    
    std::string line;
    std::array<int, 3> commands;
    while (getline(input, line)) {
        commands = extract_inputs(line);
        move_boxes(boxes_part1, commands);
        shift_boxes(boxes_part2, commands);
    }
    input.close();

    std::string result_part1 = "";
    for (int i = 1; i < boxes_part1.size(); i++) {
        result_part1 += boxes_part1[i].back();
    }

    std::string result_part2 = "";
    for (int i = 1; i < boxes_part2.size(); i++) {
        result_part2 += boxes_part2[i].back();
    }

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}