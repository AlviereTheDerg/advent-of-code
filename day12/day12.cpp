#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <algorithm>
using namespace std;

std::vector<std::string> height_map;
std::vector<std::vector<int>> search_map;
std::vector<std::array<int,3>> search_threads;
int length_found;

int read_map() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }

    height_map = std::vector<std::string>(0);
    std::string line;
    while (getline(input, line))
        height_map.push_back(line);
    
    search_map = std::vector<std::vector<int>>(height_map.size(), std::vector<int>(height_map[0].length(), -1));
    return 0;
}

void read_start() {
    int x, y;
    for (int y = 0; y < height_map.size(); y++) {
        if ((x = height_map[y].find('S')) == (std::string::npos))
            continue;
        
        search_threads.clear();
        search_threads.push_back({x,y});
    }
}

bool valid_move(std::array<int,3> current_coords, std::array<int,3> next_coords) {
    if (next_coords[0] < 0 || next_coords[0] >= height_map[0].length())
        return false; //x out of bounds
    
    if (next_coords[1] < 0 || next_coords[1] >= height_map.size())
        return false; //y out of bounds
    
    char current_pos = height_map[current_coords[1]][current_coords[0]];
    char next_pos = height_map[next_coords[1]][next_coords[0]];
    current_pos = (current_pos == 'S') ? 'a' : current_pos;
    next_pos = (next_pos == 'E') ? 'z' : next_pos;

    if (next_pos > current_pos + 1)
        return false;

    return true;
}

void propagate_first_search() {
    std::array<int,3> coords = search_threads.front();
    search_threads.erase(search_threads.begin());
    
    if (search_map[coords[1]][coords[0]] != -1 && search_map[coords[1]][coords[0]] <= coords[2] + 1)
        return;
    else
        search_map[coords[1]][coords[0]] = coords[2];

    std::array<int,2> possible_moves[] = { {0,1}, {0,-1}, {1,0}, {-1,0} };
    for (std::array<int,2> offset : possible_moves) {

        std::array<int,3> offset_coords = { coords[0] + offset[0] , coords[1] + offset[1] , coords[2] + 1};
        if (!valid_move(coords, offset_coords))
            continue;
        
        if (height_map[offset_coords[1]][offset_coords[0]] == 'E') {
            length_found = offset_coords[2];
            return;
        }

        search_threads.push_back(offset_coords);
    }
    return;
}

int main() {
    if (read_map() == -1) return 0;
    read_start();

    //for (std::string x : height_map)
    //    std::cout << x << std::endl;
    //std::cout << search_threads[0][0] << "," << search_threads[0][1] << std::endl;
    
    length_found = 0;
    while (length_found == 0 && search_threads.size() > 0) propagate_first_search();

    std::cout << "Part 1: " << length_found << std::endl;
    return 0;
}