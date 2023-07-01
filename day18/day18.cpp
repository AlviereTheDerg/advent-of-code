#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <algorithm>
#include <set>
using namespace std;


std::array<int,3> read_coord(std::string line, int &max_dim) {
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
    
    max_dim = std::max(max_dim, *std::max_element(result.begin(), result.end()));

    return result;
}

std::vector<std::array<int,3>> read_input(ifstream &input, int &max_dim) {
    std::vector<std::array<int,3>> results;
    std::string line;
    
    while (getline(input, line)) {
        results.push_back(read_coord(line, max_dim));
    }
    
    return results;
}

int coord_to_int(std::array<int,3> coord, int max_dim) {
    int result = coord[0];
    result = result * max_dim + coord[1];
    result = result * max_dim + coord[2];
    return result;
}

std::array<int,3> int_to_coord(int value, int max_dim) {
    std::array<int,3> result = {0,0,0};
    result[2] = value % max_dim;
    value /= max_dim;
    result[1] = value % max_dim;
    value /= max_dim;
    result[0] = value;
    return result;
}

int conversion_checker(std::vector<std::array<int,3>> coords, int max_dim) {
    int holder;
    std::array<int,3> holderray;
    for (std::array<int,3> coord : coords) {
        holder = coord_to_int(coord, max_dim);
        holderray = int_to_coord(holder, max_dim);
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

std::set<int> convert_coords(std::vector<std::array<int,3>> coords, int max_dim) {
    std::set<int> results;
    for (std::array<int,3> coord : coords) {
        results.insert(coord_to_int(coord, max_dim));
    }
    return results;
}

int count_surfaces(int max_dim, std::set<int> condensed_coords) {
    std::vector<std::array<int,3>> offset_coords = { {-1,0,0}, {1,0,0}, {0,-1,0}, {0,1,0}, {0,0,-1}, {0,0,1} };
    std::vector<int> offset_ints;
    for (std::array<int,3> coord : offset_coords) {
        offset_ints.push_back(coord_to_int(coord, max_dim));
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

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    int max_dim = 0;
    std::vector<std::array<int,3>> coords = read_input(input, max_dim);
    input.close();
    max_dim++;
    if (conversion_checker(coords, max_dim) != 0)
        return -1;
    
    std::set<int> condensed_coords = convert_coords(coords, max_dim);
    
    std::cout << "Part 1: " << count_surfaces(max_dim, condensed_coords) << std::endl;
    return 0;
}