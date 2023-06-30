#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <map>
using namespace std;

const static int rock_types = 5;
const static int width = 7;
std::string jets;
int jet_position = 0;
std::vector<int> rocks[rock_types] = { 
    { 0x78 }, //111 1000
    { 0x20,0x70,0x20 }, //010 0000, 111 0000, 010 0000
    { 0x70,0x10,0x10 }, //001 0000, 001 0000, 111 0000 but reversed
    { 0x40,0x40,0x40,0x40 }, //100 0000, 100 0000, 100 0000, 100 0000
    { 0x60,0x60 } //110 0000, 110 0000
};
const static int rock_starter_offset = 2; //leftmost position
int rock_widths[rock_types] = { 4,3,3,1,2 }; //how wide each rock is

std::vector<int> rock_tower;
long long current_floor, highest;
std::map<std::pair<int,int>, std::array<long long,3>> rock_cycle_map;

bool can_rock_go(int rock_type, long long vert, int hori) {
    if (hori < 0) return false;
    if (hori + rock_widths[rock_type] > width) return false;
    if (vert < current_floor) return false;
    
    for (int height = 0; height < rocks[rock_type].size(); height++) {
        int intersect = rock_tower[vert+height-current_floor] & (rocks[rock_type][height] >> hori);
        if (intersect != 0)
            return false;
    }
    
    return true;
}

bool shift_rock(long long rock_height, int &rock_position, int rock_type, bool to_right) {
    if (!to_right && can_rock_go(rock_type, rock_height, rock_position - 1)) {
        rock_position--;
        return true;
    }

    if (to_right && can_rock_go(rock_type, rock_height, rock_position + 1)) {
        rock_position++;
        return true;
    }

    return false;
}

bool move_rock_down(long long &rock_height, int rock_position, int rock_type) {
    if (can_rock_go(rock_type, rock_height - 1, rock_position)) {
        rock_height--;
        return true;
    }
    return false;
}

void place_rock(long long rock_height, int rock_position, int rock_type) {
    if (!can_rock_go(rock_type, rock_height, rock_position)) std::cout << "Oh no" << std::endl;
    
    for (int height = 0; height < rocks[rock_type].size(); height++) {
        rock_tower[rock_height + height - current_floor] |= (rocks[rock_type][height] >> rock_position);
        highest = std::max(highest, rock_height + height + 1);
    }
    
    return;
}

void pad(int amount) {
    if (rock_tower.size() >= (highest - current_floor) + amount)
        return;
    
    rock_tower.resize(highest - current_floor + amount, 0);
}

void trim(long long rock_height, int rock_position, int rock_type) {
    int high_trim = current_floor;
    for (int trim_check = rock_height - 1; trim_check < rock_height + rocks[rock_type].size(); trim_check++) {
        if ((rock_tower[trim_check - current_floor] | rock_tower[trim_check - current_floor + 1]) == 127)
            high_trim = trim_check;
    }
    if (high_trim == current_floor)
        return;
    rock_tower.erase(rock_tower.begin(), rock_tower.begin() + (high_trim - current_floor));
    current_floor = high_trim;
}

void drop_new_rock(int rock_type) {
    long long rock_height = highest + 3;
    int rock_position = rock_starter_offset;
    pad(10);

    do {
        shift_rock(rock_height, rock_position, rock_type, (jets[jet_position] == '>'));
        jet_position = (jet_position + 1) % jets.length();
    } while (move_rock_down(rock_height, rock_position, rock_type));

    place_rock(rock_height, rock_position, rock_type);
    trim(rock_height, rock_position, rock_type);
    return;
}

int calculate_rock_score() {
    int result = 0, level;
    for (int bitmask = 1; bitmask < 128; bitmask = bitmask << 1) {
        result *= 7;
        level = highest - current_floor;
        while (level > 0 && (bitmask & rock_tower[level]) == 0) level--;
        result += level;
    }
    return result;
}

std::pair<int, long long> drop_rocks() {
    long long rocks_to_drop = 1000000000000;
    rock_tower.clear();
    std::pair<int, long long> results = make_pair(0,0);
    highest = 0;
    current_floor = 0;
    int rock_type = 0;
    std::pair<int,int> rock_index;
    int temp_score;
    long long loop_gap, loop_length, loops_skipped = 0;

    for (long long rocks_dropped = 0; rocks_dropped < rocks_to_drop; rock_type = ++rocks_dropped % rock_types) {
        if (rocks_dropped == 2022) {
            results.first = highest;
        }
        
        rock_index = make_pair(rock_type, jet_position);
        temp_score = calculate_rock_score();
        if (rock_cycle_map.count(rock_index) == 0 || rock_cycle_map[rock_index][0] != temp_score) {
            rock_cycle_map[rock_index] = {temp_score, highest, rocks_dropped};
            
        } else if (rocks_dropped > 2022 && loops_skipped == 0) {
            loops_skipped = (rocks_to_drop - rocks_dropped) / (rocks_dropped - rock_cycle_map[rock_index][2]);
            loop_gap = highest - rock_cycle_map[rock_index][1];
            rocks_dropped += (rocks_dropped - rock_cycle_map[rock_index][2]) * loops_skipped;
        }

        drop_new_rock(rock_type);
    }
    
    results.second = highest + loop_gap * loops_skipped;
    return results;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    getline(input, jets);
    input.close();
    std::pair<int, long long> results = drop_rocks();

    std::cout << "Part 1: " << results.first << std::endl;
    std::cout << "Part 2: " << results.second << std::endl;
    return 0;
}