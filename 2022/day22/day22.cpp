#include <iostream>
#include <fstream>
#include <vector>
#include <set>
#include <algorithm>
#include <array>
using namespace std;

int const cube_faces = 6, directions = 4;
typedef std::array<int,4> location_data;
//face, relative row, relative column, facing

location_data live_player;
int face_size;
std::array<std::vector<std::string>, cube_faces> map;
std::array<std::array<int,directions>,cube_faces> face_wraps;
std::vector<std::string> movement;

std::vector<std::vector<int>> mini_map;
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

void make_mini_map(std::vector<std::string> temp_map) {
    mini_map = std::vector<std::vector<int>>(height / face_size, std::vector<int>(width / face_size, -1));
    int face_count = 0;
    for (int y = 0; y < height; y += face_size) {
        for (int x = 0; x < width; x += face_size) {
            if (x < temp_map[y].length() && temp_map[y][x] == ' ')
                continue;
            
            if (x >= temp_map[y].length())
                continue;
            
            mini_map[y / face_size][x / face_size] = face_count++;
        }
    }
}

void store_map(std::vector<std::string> temp_map) {
    int current_face;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            current_face = mini_map[i][j];
            if (current_face == -1)
                continue;
            
            for (int k = 0; k < face_size; k++) {
                map[current_face].push_back(temp_map[i * face_size + k].substr(face_size * j, face_size));
            }
        }
    }
    return;
}

void load_data(ifstream &input) {
    std::vector<std::string> temp_map;
    std::set<int> face_finder;
    std::string line;
    while (getline(input, line) && line.compare("") != 0) {
        temp_map.push_back(line);
        face_finder.insert(std::min(line.find('.'), line.find('#')));
        face_finder.insert(line.length() - std::min(line.find('.'), line.find('#')));
        face_finder.insert(line.length());
        width = std::max(width, (int) line.length());
    }
    face_size = *face_finder.lower_bound(1);
    height = temp_map.size();
    make_mini_map(temp_map);
    height /= face_size;
    width /= face_size;
    store_map(temp_map);

    getline(input, line);
    movement = split_string(line);
    
    return;
}

void reset_wrap() {
    for (int i = 0; i < cube_faces; i++)
        for (int j = 0; j < directions; j++)
            face_wraps[i][j] = -1;
    
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            if (mini_map[y][x] == -1)
                continue;
            
            if (x < width - 1 && mini_map[y][x + 1] != -1)
                face_wraps[mini_map[y][x]][0] = mini_map[y][x+1];
            if (x > 0 && mini_map[y][x - 1] != -1)
                face_wraps[mini_map[y][x]][2] = mini_map[y][x-1];
            
            if (y < height - 1 && mini_map[y + 1][x] != -1)
                face_wraps[mini_map[y][x]][1] = mini_map[y+1][x];
            if (y > 0 && mini_map[y - 1][x] != -1)
                face_wraps[mini_map[y][x]][3] = mini_map[y-1][x];
        }
    }
    return;
}

void load_flat_wrap() {
    reset_wrap();
    int upleft, downright;
    for (int i = 0; i < height; i++) {
        upleft = height; downright = -1;
        for (int j = 0; j < width; j++) {
            if (mini_map[i][j] == -1)
                continue;
            upleft = std::min(upleft, j);
            downright = std::max(downright, j);
        }
        upleft = mini_map[i][upleft];
        downright = mini_map[i][downright];
        face_wraps[upleft][2] = downright;
        face_wraps[downright][0] = upleft;
    }
    
    for (int i = 0; i < width; i++) {
        upleft = width; downright = -1;
        for (int j = 0; j < height; j++) {
            if (mini_map[j][i] == -1)
                continue;
            upleft = std::min(upleft, j);
            downright = std::max(downright, j);
        }
        upleft = mini_map[upleft][i];
        downright = mini_map[downright][i];
        face_wraps[upleft][3] = downright;
        face_wraps[downright][1] = upleft;
    }
    return;
}

bool scan_and_connect() {
    bool result = false;
    int index, thumb, counter_index, counter_thumb;
    for (int i = 0; i < face_wraps.size(); i++) {
        for (int j = 0; j < face_wraps[i].size(); j++) {
            index = face_wraps[i][j];
            thumb = face_wraps[i][(j+1)%face_wraps[i].size()];
            
            if (index == -1 || thumb == -1)
                continue;
            
            counter_index = std::find(face_wraps[index].begin(), face_wraps[index].end(), i) - face_wraps[index].begin();
            counter_thumb = std::find(face_wraps[thumb].begin(), face_wraps[thumb].end(), i) - face_wraps[thumb].begin();

            if (face_wraps[index][counter_index] != i || face_wraps[thumb][counter_thumb] != i) {
                std::cout << "Something went wrong: scan_and_connect()" << std::endl;
                return false;
            }

            counter_index = (counter_index + face_wraps[counter_index].size() - 1) % face_wraps[counter_index].size();
            counter_thumb = (counter_thumb + face_wraps[counter_thumb].size() + 1) % face_wraps[counter_thumb].size();

            if (face_wraps[index][counter_index] == thumb && face_wraps[thumb][counter_thumb] == index)
                continue;
            
            result = true;
            face_wraps[index][counter_index] = thumb;
            face_wraps[thumb][counter_thumb] = index;
        }
    }
    return result;
}

void load_cube_wrap() {
    reset_wrap();
    while (scan_and_connect());
    return;
}

location_data wrap_around(location_data OOB_coord) {
    int position; //how far along given wall the junction is
    switch (OOB_coord[3]) {
        case 0: position = OOB_coord[1]; break;
        case 1: position = face_size - OOB_coord[2] - 1; break;
        case 2: position = face_size - OOB_coord[1] - 1; break;
        case 3: position = OOB_coord[2]; break;
    }

    int target_face = face_wraps[OOB_coord[0]][OOB_coord[3]];
    int counter_direction = OOB_coord[3];
    do {
        if (face_wraps[target_face][(counter_direction + 2) % directions] == OOB_coord[0])
            break;
        
        counter_direction = (counter_direction + 1) % directions;
    } while (counter_direction != OOB_coord[3]);
    
    int next_row, next_column;
    switch (counter_direction) {
        case 0: 
            next_row = position;
            next_column = 0;
            break;
        case 1: 
            next_row = 0;
            next_column = face_size - position - 1;
            break;
        case 2: 
            next_row = face_size - position - 1;
            next_column = face_size - 1;
            break;
        case 3: 
            next_row = face_size - 1;
            next_column = position;
            break;
    }
    
    return {target_face, next_row, next_column, counter_direction};
}

location_data forward() {
    int next_row = live_player[1], next_column = live_player[2];
    switch (live_player[3]) {
        case 0: next_column++; break;
        case 1: next_row++; break;
        case 2: next_column--; break;
        case 3: next_row--; break;
    }
    if (next_row >= 0 && next_row < face_size && next_column >= 0 && next_column < face_size)
        return {live_player[0], next_row, next_column, live_player[3]};
    return wrap_around({live_player[0], next_row, next_column, live_player[3]});
}

char check_position(location_data coords) {
    return map[coords[0]][coords[1]][coords[2]];
}

bool move() {
    location_data next = forward();
    if (check_position(next) == '#')
        return false;
    
    live_player = next;
    return true;
}

bool turn(char dir) {
    switch (dir) {
        case 'R': live_player[3] = (live_player[3] + 1) % directions; return true;
        case 'L': live_player[3] = (live_player[3] - 1 + directions) % directions; return true;
    }
    return false;
}

void traverse() {
    live_player = {0,0,0,0};
    int movin;
    for (std::string instruction : movement) {
        if (turn(instruction[0]))
            continue;
        
        movin = stoi(instruction);
        while (movin-- > 0 && move());
    }
}

int calculate_score() {
    int results, global_row = -1, global_column = -1;
    for (int y = 0; y < height; y++)
        for (int x = 0; x < width; x++)
            if (mini_map[y][x] == live_player[0]) {
                global_row = 1 + y * face_size + live_player[1];
                global_column = 1 + x * face_size + live_player[2];
            }
    results = 1000*global_row + 4*global_column + live_player[3];
    return results;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    load_data(input);
    input.close();
    
    load_flat_wrap();
    traverse();
    std::cout << "Part 1: " << calculate_score() << std::endl;
    load_cube_wrap();
    traverse();
    std::cout << "Part 2: " << calculate_score() << std::endl;

    return 0;
}