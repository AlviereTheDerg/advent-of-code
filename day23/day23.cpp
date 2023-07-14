#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <set>
#include <chrono>
using namespace std;

typedef std::pair<int,int> location;
std::set<location> elves;
int start_rule = 0;
std::vector<location> checkables = { //all locations that can be checked
    location(-1,-1), location(-1, 0), location(-1, 1), location( 0,-1), location( 0, 1), location( 1,-1), location( 1, 0), location( 1, 1)
};
std::vector<std::pair<location, std::vector<int>>> rules = { //move offset, { check_1, check_2, ... check_n }, checks as indexes in checkables
    { location( 0,-1), {0,3,5} }, //-1,-1 -> 0; 0,-1 -> 3; 1,-1 -> 5
    { location( 0, 1), {2,4,7} }, //-1, 1 -> 2; 0, 1 -> 4; 1, 1 -> 7
    { location(-1, 0), {0,1,2} }, //-1,-1 -> 0;-1, 0 -> 1;-1, 1 -> 2
    { location( 1, 0), {5,6,7} }  // 1,-1 -> 5; 1, 0 -> 6; 1, 1 -> 7
};

void load_data(ifstream &input) {
    std::vector<std::string> map;
    std::string line;
    int y = 0;
    while (getline(input, line)) {
        for (int x = 0; x < line.length(); x++)
            if (line[x] == '#')
                elves.insert(location(x, y));
        y++;
    }
    
    return;
}

location plan_move(location elf) {
    std::vector<bool> checkeds;
    bool neighbours = false;
    for (int i = 0; i < checkables.size(); i++) {
        checkeds.push_back(elves.count(location(elf.first + checkables[i].first, elf.second + checkables[i].second)) == 1);
        neighbours |= checkeds[i];
    }
    if (!neighbours)
        return elf;
    
    location next_position = elf;
    for (int i = 0; i < rules.size(); i++) {
        neighbours = false;
        for (int rule : rules[(i + start_rule) % rules.size()].second)
            neighbours |= checkeds[rule];
        if (neighbours)
            continue;
        
        next_position.first += rules[(i + start_rule) % rules.size()].first.first;
        next_position.second += rules[(i + start_rule) % rules.size()].first.second;
        break;
    }

    return next_position;
}

std::map<location, location> plan_moves() {
    std::map<location, location> elf_moves;
    location plan;
    for (location elf : elves) {
        plan = plan_move(elf);
        if (plan != elf)
            elf_moves[elf] = plan;
    }
    return elf_moves;
}

std::set<location> identify_overlaps(std::map<location, location> elf_moves) {
    std::set<location> all_moves, overlaps;
    for (std::pair<location, location> move : elf_moves) {
        if (all_moves.count(move.second) == 0)
            all_moves.insert(move.second);
        else
            overlaps.insert(move.second);
    }
    return overlaps;
}

bool enact_moves(std::map<location, location> moves, std::set<location> overlaps) {
    bool result = false;
    for (std::pair<location, location> move : moves) {
        if (overlaps.count(move.second) != 0)
            continue;
        
        if (move.first == move.second)
            continue;

        result = true;
        elves.erase(move.first);
        elves.insert(move.second);
    }
    return result;
}

bool round() {
    std::map<location, location> prospective_moves = plan_moves();
    std::set<location> overlaps = identify_overlaps(prospective_moves);
    start_rule = (start_rule + 1) % rules.size();
    return enact_moves(prospective_moves, overlaps);
}

long long calculate_part1() {
    int up, left, down, right;
    up = down = elves.begin()->second;
    left = right = elves.begin()->first;
    for (location elf : elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }
    long long result = (long long) (right - left + 1) * (down - up + 1);
    result -= elves.size();
    return result;
}

void print() {
    int up, left, down, right;
    up = down = elves.begin()->second;
    left = right = elves.begin()->first;
    for (location elf : elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }

    for (int y = up; y <= down; y++) {
        for (int x = left; x <= right; x++) {
            std::cout << ((elves.count(location(x,y)) == 0) ? '.' : '#');
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    load_data(input);
    input.close();
    
    
    auto t1 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < 10; i++)
        round();
    auto t2 = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::milli> exec_double = t2 - t1;
    std::cout << "Part 1: " << calculate_part1() << std::endl;
    std::cout << "Part 1 execution time: " << exec_double.count() << "ms" << std::endl;
    
    
    t1 = std::chrono::high_resolution_clock::now();
    int i = 11;
    while (round()) {
        if (i % 10 == 0)
            std::cout << "Round: " << i << std::endl;
        i++;
    }
    t2 = std::chrono::high_resolution_clock::now();
    exec_double = t2 - t1;
    std::cout << "Part 2: " << i << std::endl;
    std::cout << "Part 2 execution time: " << exec_double.count() << "ms" << std::endl;
    
    return 0;
}