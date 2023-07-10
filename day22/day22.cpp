#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

int row, column, facing;
std::vector<std::string> map;
std::vector<int> map_lefts, map_uppers, map_lowers;
std::vector<std::string> movement;
int height, width;

std::vector<std::string> split_string(std::string input) {
    std::vector<std::string> result;
    int one = 0, two;
    int L = input.find('L', one);
    int R = input.find('R', one);
    
    while (L != -1 && R != -1) {
        two = std::min(L, R);
        result.push_back(input.substr(one, two-one));
        result.push_back(input.substr(two, 1));
        one = two + 1;
        L = input.find('L', one);
        R = input.find('R', one);
    }
    two = std::max(L, R);
    result.push_back(input.substr(one, two-one));
    result.push_back(input.substr(two, 1));
    result.push_back(input.substr(two + 1));

    return result;
}

void load_data(ifstream &input) {
    map.clear();
    std::string line;
    while (getline(input, line) && line.compare("") != 0) {
        map.push_back(line);
        width = std::max(width, (int) line.length());
    }
    height = map.size();

    map_lefts.clear();
    for (int i = 0; i < height; i++) {
        map_lefts.push_back(std::min(map[i].find('.'), map[i].find('#')));
        map[i] = map[i].substr(map_lefts[i]);
    }

    map_uppers.clear();
    map_lowers.clear();
    for (int i = 0; i < width; i++) {
        map_uppers.push_back(height);
        map_lowers.push_back(-1);
        for (int j = 0; j < height; j++) {
            if (i < map_lefts[j] || i >= map_lefts[j] + map[j].length())
                continue;
            map_uppers[i] = std::min(map_uppers[i], j);
            map_lowers[i] = std::max(map_lowers[i], j);
        }
    }

    getline(input, line);
    movement = split_string(line);
    
    row = 0;
    column = map[row].find('.') + map_lefts[0];
    facing = 0;
    
    return;
}

std::pair<int,int> forward() {
    int next_row = row, next_column = column;
    switch (facing) {
        case 0:
            next_column = (next_column + 1 - map_lefts[row]) % map[row].length() + map_lefts[row];
            break;
        case 1:
            next_row = (next_row + 1 - map_uppers[column]) % (map_lowers[column] - map_uppers[column] + 1) + map_uppers[column];
            break;
        case 2:
            next_column = (next_column - 1 - map_lefts[row] + map[row].length()) % map[row].length() + map_lefts[row];
            break;
        case 3:
            next_row = (next_row - 1 - map_uppers[column] + (map_lowers[column] - map_uppers[column] + 1)) % (map_lowers[column] - map_uppers[column] + 1) + map_uppers[column];
            break;
    }
    return std::make_pair(next_column, next_row);
}

char check_position(std::pair<int,int> coords) {
    return map[coords.second][coords.first - map_lefts[coords.second]];
}

void set_position(std::pair<int,int> coords) {
    auto select = [](int foo) { 
        switch(foo) {
            case 0: return '>';
            case 1: return 'v';
            case 2: return '<';
            case 3: return '^';
        }
    };
    map[coords.second][coords.first - map_lefts[coords.second]] = select(facing);
}

bool move() {
    set_position(std::make_pair(column, row));
    std::pair<int,int> next = forward();
    if (check_position(next) == '#')
        return false;
    
    column = next.first;
    row = next.second;
    return true;
}

bool turn(char dir) {
    switch (dir) {
        case 'R': facing = (facing + 1) % 4; return true;
        case 'L': facing = (facing - 1 + 4) % 4; return true;
    }
    return false;
}

void print_coord(std::pair<int,int> input) {
    std::cout << "(" << input.first << "," << input.second << "): " << check_position(input) << std::endl;
}

void traverse() {
    int movin;
    for (std::string instruction : movement) {
        if (turn(instruction[0]))
            continue;
        
        movin = stoi(instruction);
        while (movin-- > 0 && move());
    }
}

int part1_calculate() {
    return ((1 + row) * 1000) + ((1 + column) * 4) + facing;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    load_data(input);
    input.close();
    
    traverse();
    std::cout << "Part 1: " << part1_calculate() << std::endl;
    
    return 0;
}