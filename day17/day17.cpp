#include <iostream>
#include <fstream>
#include <vector>
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

bool can_rock_go(std::vector<int> rock_tower, int rock_type, int vert, int hori) {
    if (hori < 0) return false;
    if (hori + rock_widths[rock_type] > width) return false;
    if (vert < 0) return false;
    
    for (int height = 0; height < rocks[rock_type].size(); height++) {
        int intersect = rock_tower[vert+height] & (rocks[rock_type][height] >> hori);
        if (intersect != 0)
            return false;
    }
    
    return true;
}

bool shift_rock(std::vector<int> rock_tower, int rock_height, int &rock_position, int rock_type, bool to_right) {
    if (!to_right && can_rock_go(rock_tower, rock_type, rock_height, rock_position - 1)) {
        rock_position--;
        return true;
    }

    if (to_right && can_rock_go(rock_tower, rock_type, rock_height, rock_position + 1)) {
        rock_position++;
        return true;
    }

    return false;
}

bool move_rock_down(std::vector<int> rock_tower, int &rock_height, int rock_position, int rock_type) {
    if (can_rock_go(rock_tower, rock_type, rock_height - 1, rock_position)) {
        rock_height--;
        return true;
    }
    return false;
}

void place_rock(std::vector<int> &rock_tower, int &highest, int rock_height, int rock_position, int rock_type) {
    if (!can_rock_go(rock_tower, rock_type, rock_height, rock_position)) std::cout << "Oh no" << std::endl;
    
    for (int height = 0; height < rocks[rock_type].size(); height++) {
        rock_tower[rock_height + height] |= (rocks[rock_type][height] >> rock_position);
        highest = std::max(highest, rock_height + height + 1);
    }
    
    return;
}

void drop_new_rock(std::vector<int> &rock_tower, int &highest, int rock_type) {
    int rock_height = highest + 3;
    int rock_position = rock_starter_offset;

    do {
        shift_rock(rock_tower, rock_height, rock_position, rock_type, (jets[jet_position] == '>'));
        jet_position = (jet_position + 1) % jets.length();
    } while (move_rock_down(rock_tower, rock_height, rock_position, rock_type));

    place_rock(rock_tower, highest, rock_height, rock_position, rock_type);

    return;
}

int part1_calculate(int rocks_to_drop) {
    std::vector<int> rock_tower (rocks_to_drop * 4 + 3, 0);
    int highest = 0;

    for (int i = 0; i < rocks_to_drop; i++) {
        drop_new_rock(rock_tower, highest, i % rock_types) ;
    }

    return highest;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    getline(input, jets);
    input.close();

    std::cout << "Part 1: " << part1_calculate(2022) << std::endl;
    return 0;
}