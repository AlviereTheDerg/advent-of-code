#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <set>
using namespace std;

//kludge
const int max_dim = 100;

std::array<int,3> read_coord(std::string line) {
    std::array<int,3> result;
    int one, two;
    
    one = 0;
    two = line.find(',', one);
    result[0] = stoi(line.substr(one, two-one));
    
    one = two + 1;
    two = line.find(',', one);
    result[1] = stoi(line.substr(one, two-one));
    
    one = two + 1;
    result[2] = stoi(line.substr(one));

    return result;
}

std::vector<std::array<int,3>> read_input(ifstream &input) {
    std::vector<std::array<int,3>> results;
    std::string line;
    
    while (getline(input, line)) {
        results.push_back(read_coord(line));
    }
    
    return results;
}

int coord_to_int(std::array<int,3> coord) {
    int result = coord[0];
    result = result * max_dim + coord[1];
    result = result * max_dim + coord[2];
    return result;
}

std::array<int,3> int_to_coord(int value) {
    std::array<int,3> result = {0,0,0};
    result[2] = value % max_dim;
    value /= max_dim;
    result[1] = value % max_dim;
    value /= max_dim;
    result[0] = value;
    return result;
}

int conversion_checker(std::vector<std::array<int,3>> coords) {
    int holder;
    std::array<int,3> holderray;
    for (std::array<int,3> coord : coords) {
        holder = coord_to_int(coord);
        holderray = int_to_coord(holder);
        for (int foo = 0; foo < 3; foo++) {
            if (coord[foo] == holderray[foo])
                continue;
            
            std::cout << '[' << coord[0] << ", " << coord[1] << ", " << coord[2] << ']' << std::endl;
            std::cout << '[' << holderray[0] << ", " << holderray[1] << ", " << holderray[2] << ']' << std::endl;
            return -1;
        }
    }
    return 0;
}

std::set<int> convert_coords(std::vector<std::array<int,3>> coords) {
    std::set<int> results;
    for (std::array<int,3> coord : coords) {
        results.insert(coord_to_int(coord));
    }
    return results;
}

int count_surfaces(std::set<int> condensed_coords) {
    std::vector<std::array<int,3>> offset_coords = { {-1,0,0}, {1,0,0}, {0,-1,0}, {0,1,0}, {0,0,-1}, {0,0,1} };
    std::vector<int> offset_ints;
    for (std::array<int,3> coord : offset_coords) {
        offset_ints.push_back(coord_to_int(coord));
    }

    int result = 0;
    int current_coord;
    for (int current_coord : condensed_coords) {
        for (int offset : offset_ints) {
            if (condensed_coords.count(current_coord + offset) == 0)
                result++;
        }
    }
    return result;
}

int count_outer_surfaces(std::vector<std::array<int,3>> coords, std::set<int> condensed_coords) {
    int min_wall = -2, max_wall = 21;
    std::set<int> walls;
    for (int x = min_wall; x <= max_wall; x++) {
        for (int y = min_wall; y <= max_wall; y++) {
            for (int z = min_wall; z <= max_wall; z++) {
                if ((x==min_wall || x==max_wall) || (y==min_wall || y==max_wall) || (z==min_wall || z==max_wall))
                    walls.insert(coord_to_int({x,y,z}));
            }
        }
    }
    
    std::vector<std::array<int,3>> offset_coords = { {-1,0,0}, {1,0,0}, {0,-1,0}, {0,1,0}, {0,0,-1}, {0,0,1} };
    std::vector<int> offset_ints;
    for (std::array<int,3> coord : offset_coords) {
        offset_ints.push_back(coord_to_int(coord));
    }

    std::set<int> air;
    std::set<int> air_checking = { coord_to_int({-1,-1,-1}) };
    int current_coord;
    while (air_checking.size() > 0) {
        current_coord = *air_checking.begin();
        air_checking.erase(current_coord);
        air.insert(current_coord);

        for (int offset : offset_ints) {
            if (walls.count(current_coord + offset) != 0 || condensed_coords.count(current_coord + offset) != 0)
                continue;
            
            if (air.count(current_coord + offset) != 0)
                continue;
            
            if (air_checking.count(current_coord + offset) != 0)
                continue;

            air_checking.insert(current_coord + offset);
        }
    }

    int result = 0;
    for (int current_coord : condensed_coords) {
        for (int offset : offset_ints) {
            if (air.count(current_coord + offset) > 0)
                result++;
        }
    }
    return result;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::vector<std::array<int,3>> coords = read_input(input);
    input.close();
    if (conversion_checker(coords) != 0)
        return -1;
    
    std::set<int> condensed_coords = convert_coords(coords);
    
    std::cout << "Part 1: " << count_surfaces(condensed_coords) << std::endl;
    std::cout << "Part 2: " << count_outer_surfaces(coords, condensed_coords) << std::endl;
    return 0;
}