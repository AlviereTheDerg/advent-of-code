#include <iostream>
#include <fstream>
#include <map>
#include <vector>
using namespace std;

int draw_line(std::map<std::pair<int,int>, char> &wall, std::string point1, std::string point2, int &leftmost, int &rightmost) {
    std::pair<int,int> pair1, pair2;
    pair1 = make_pair( stoi( point1.substr(0, point1.find(',') ) ) , stoi( point1.substr( point1.find(',')+1 ) ) );
    pair2 = make_pair( stoi( point2.substr(0, point2.find(',') ) ) , stoi( point2.substr( point2.find(',')+1 ) ) );
    void (*iteration)(std::pair<int,int> &foo);
    if (pair1.first == pair2.first && pair1.second < pair2.second)
        iteration = [](std::pair<int,int> &foo) { foo.second++; };
    else if (pair1.first == pair2.first)
        iteration = [](std::pair<int,int> &foo) { foo.second--; };
    else if (pair1.second == pair2.second && pair1.first < pair2.first)
        iteration = [](std::pair<int,int> &foo) { foo.first++; };
    else
        iteration = [](std::pair<int,int> &foo) { foo.first--; };
    
    for (std::pair<int,int> coord(pair1); coord != pair2; iteration(coord)) {
        wall.insert(make_pair(coord,'#'));
    }
    wall.insert(make_pair(pair2,'#'));

    leftmost = std::min(leftmost, std::min(pair1.first, pair2.first));
    rightmost = std::max(rightmost, std::max(pair1.first, pair2.first));
    return std::max(pair1.second, pair2.second);
}

std::vector<std::string> split_list(std::string input_list) {
    size_t start = 0, end;
    std::string section, delim = " -> ";
    std::vector<std::string> result(0);

    while ((end = input_list.find(delim, start)) != string::npos) {
        section = input_list.substr(start, end - start);
        start = end + delim.length();
        result.push_back(section);
    }
    result.push_back(input_list.substr(start));
    return result;
}

int sand_fall(std::map<std::pair<int,int>,char> &wall, int wall_height, std::pair<int,int> current_position) {
    if (current_position.second == wall_height + 2) {
        return 0;
    }
    
    std::map<std::pair<int,int>,char>::iterator found = wall.find(make_pair(current_position.first, current_position.second + 1)); //down
    if (found == wall.end())
        return sand_fall(wall, wall_height, make_pair(current_position.first, current_position.second + 1));
    
    found = wall.find(make_pair(current_position.first - 1, current_position.second + 1)); //down left
    if (found == wall.end())
        return sand_fall(wall, wall_height, make_pair(current_position.first - 1, current_position.second + 1));
    
    found = wall.find(make_pair(current_position.first + 1, current_position.second + 1)); //down right
    if (found == wall.end())
        return sand_fall(wall, wall_height, make_pair(current_position.first + 1, current_position.second + 1));
    
    //all 3 below it blocked
    found = wall.find(current_position);
    if (found != wall.end()) //current position filled by something
        return 0;

    wall.insert(make_pair(current_position,'o'));
    return 1;
}

int drop_sand(std::map<std::pair<int,int>,char> &wall, int wall_height) {
    int count = 0;
    while (sand_fall(wall, wall_height, make_pair(500,0))) count++;
    return count;
}

void print_wall(std::map<std::pair<int,int>,char> &wall, int wall_height, int leftmost, int rightmost) {
    std::map<std::pair<int,int>,char>::iterator found;
    for (int y = 0; y <= wall_height + 2; y++) {
        for (int x = leftmost; x <= rightmost; x++) {
            if (x == 500 && y == 0) {
                std::cout << '+';
                continue;
            }

            found = wall.find(make_pair(x,y));
            if (found == wall.end())
                std::cout << '.';
            else
                std::cout << found->second;
            
        }
        std::cout << std::endl;
    }
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }

    int deepest = 0, leftmost = 500, rightmost = 500;
    std::string line;
    std::vector<std::string> line_v(0);
    std::map<std::pair<int,int>, char> wall_map;
    while (!input.eof()) {
        getline(input, line);
        line_v = split_list(line);
        for (int index = 0; index < line_v.size() - 1; index++)
            deepest = std::max(deepest, draw_line(wall_map, line_v[index], line_v[index+1], leftmost, rightmost));
    }
    input.close();

    int result_part1 = drop_sand(wall_map, deepest);
    //print_wall(wall_map, deepest, leftmost, rightmost);
    std::cout << "Part 1: " << result_part1 << std::endl;

    std::string big_leftmost = std::to_string(500 - deepest - 2) + "," + std::to_string(deepest + 2);
    std::string big_rightmost = std::to_string(500 + deepest + 2) + "," + std::to_string(deepest + 2);
    draw_line(wall_map, big_leftmost, big_rightmost, leftmost, rightmost);
    int result_part2 = drop_sand(wall_map, deepest) + result_part1;
    //print_wall(wall_map, deepest, leftmost, rightmost);
    std::cout << "Part 2: " << result_part2 << std::endl;

    return 0;
}