#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <set>
#include <chrono>
using namespace std;

typedef std::pair<int,int> location;
std::set<location> current_unmoving_elves, current_moving_elves;
std::set<location> next_unmoving_elves, next_moving_elves;

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
std::vector<int> opposing_directions{1,0,3,2};

location operator+(const location l, const location r) {
    return {l.first+r.first, l.second+r.second};
}

location operator-(const location l, const location r) {
    return {l.first-r.first, l.second-r.second};
}

location operator*(const int l, const location r) {
    return {l*r.first, l*r.second};
}

void load_data(ifstream &input) {
    current_unmoving_elves.clear();
    current_moving_elves.clear();

    std::set<location> elves{};
    std::string line;
    int y = 0;
    while (getline(input, line)) {
        for (int x = 0; x < line.size(); x++)
            if (line[x] == '#')
                elves.insert(location(x, y));
        y++;
    }
    
    bool neighbour;
    for (location elf : elves) {
        neighbour = false;
        for (location offset : checkables) {
            if (elves.count(elf + offset) == 0)
                continue;
            
            neighbour = true;
            break;
        }
        if (neighbour)
            current_moving_elves.insert(elf);
        else
            current_unmoving_elves.insert(elf);
    }

    std::cout << "Total elves: " << elves.size() << '\n';
    std::cout << "Elves with no need to move: " << current_unmoving_elves.size() << '\n';
    std::cout << "Elves who need to move: " << current_moving_elves.size() << '\n';
    if (elves.size() != current_unmoving_elves.size() + current_moving_elves.size())
        std::cout << "Uh oh\n";
    return;
}

int plan_move(location elf) {
    std::vector<bool> checkeds;
    bool neighbours = false;
    for (int i = 0; i < checkables.size(); i++) {
        checkeds.push_back(current_unmoving_elves.count(elf + checkables[i]) == 1
            || current_moving_elves.count(elf + checkables[i]) == 1);
        neighbours |= checkeds[i];
    }
    if (!neighbours)
        return -1; //no need to move
    
    for (int i = 0; i < rules.size(); i++) {
        neighbours = false;
        for (int rule : rules[(i + start_rule) % rules.size()].second)
            neighbours |= checkeds[rule];
        if (neighbours)
            continue;
        
        return (i + start_rule) % rules.size();
    }

    return -1;
}

bool round() {
    next_unmoving_elves.clear();
    next_moving_elves.clear();

    int sanity_check{(int) current_moving_elves.size() + (int) current_unmoving_elves.size()};
    //plan out moves
    //if an elf outright can't move then put them in unable_to_move
    //if an elf might run into a move contestation put them into possible_contestation
    std::map<location,int> move_plans{};
    std::map<location,int> possible_contestation{};
    std::set<location> unable_to_move{};
    location destination;
    for (location elf : current_moving_elves) {
        move_plans[elf] = plan_move(elf);
        if (move_plans[elf] == -1) {
            unable_to_move.insert(elf);
            move_plans.erase(elf);
            continue;
        }
        destination = elf + 2*rules[move_plans[elf]].first;
        if (current_moving_elves.count(destination) != 0) {
            possible_contestation[elf] = move_plans[elf];
            move_plans.erase(elf);
            continue;
        }
    }

    //settle contestations
    for (std::pair<location,int> elf_move : possible_contestation) {
        destination = elf_move.first + 2*rules[elf_move.second].first;
        if (possible_contestation.count(destination) == 0) {
            move_plans[elf_move.first] = elf_move.second;
            continue;
        }
        if (possible_contestation[destination] == opposing_directions[elf_move.second]) {
            //another elf is trying to move to the same location
            unable_to_move.insert(elf_move.first);
        } else {
            //other elf isn't moving to this location
            move_plans[elf_move.first] = elf_move.second;
        }
    }

    //merge the moved elves
    std::set<location> unsorted_elves{unable_to_move};
    for (std::pair<location,int> elf_move : move_plans) {
        unsorted_elves.insert(elf_move.first + rules[elf_move.second].first);
    }

    //settle the elves into
    next_unmoving_elves = current_unmoving_elves;
    for (location elf : unsorted_elves) {
        bool neighbours = true;
        for (int i = 0; i < checkables.size(); i++) {
            location offset = checkables[i];
            if (next_unmoving_elves.count(elf + offset) == 0 && next_moving_elves.count(elf + offset) == 0)
                continue;
            
            neighbours = false;
            next_moving_elves.insert(elf);
            next_unmoving_elves.erase(elf + offset);
            next_moving_elves.insert(elf + offset);
        }
        if (neighbours) {
            next_unmoving_elves.insert(elf);
        }
    }

    bool result{current_moving_elves.size() != 0};
    current_moving_elves = next_moving_elves;
    current_unmoving_elves = next_unmoving_elves;
    if (sanity_check != current_moving_elves.size() + current_unmoving_elves.size())
        std::cout << "Uh oh. (elves lost in round)\n";
    
    start_rule = (start_rule + 1) % rules.size();
    return result;
}

void print_elves() {
    int up, left, down, right;
    if (current_unmoving_elves.size() != 0) {
        left = right = current_unmoving_elves.begin()->first;
        up = down = current_unmoving_elves.begin()->second;
    } else {
        left = right = current_moving_elves.begin()->first;
        up = down = current_moving_elves.begin()->second;
    }
    
    for (location elf : current_unmoving_elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }
    for (location elf : current_moving_elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }

    for (int y = up; y <= down; y++) {
        for (int x = left; x <= right; x++) {
            std::cout << (
                (current_unmoving_elves.count(location(x,y)) == 0 
                && current_moving_elves.count(location(x,y)) == 0) 
                ? '.' : '#');
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}

long long calculate_part1() {
    int up, left, down, right;
    if (current_unmoving_elves.size() != 0) {
        left = right = current_unmoving_elves.begin()->first;
        up = down = current_unmoving_elves.begin()->second;
    } else {
        left = right = current_moving_elves.begin()->first;
        up = down = current_moving_elves.begin()->second;
    }
    
    for (location elf : current_unmoving_elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }
    for (location elf : current_moving_elves) {
        up = std::min(up, elf.second);
        down = std::max(down, elf.second);
        left = std::min(left, elf.first);
        right = std::max(right, elf.first);
    }

    long long result = (long long) (right - left + 1) * (down - up + 1);
    result -= current_moving_elves.size() + current_unmoving_elves.size();
    return result;
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