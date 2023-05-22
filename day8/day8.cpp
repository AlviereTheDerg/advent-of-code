#include <iostream>
#include <fstream>
using namespace std;

int* parse_line(std::string line) {
    int* intline = new int[line.length()];
    for (int index = 0; index < line.length(); index++)
        intline[index] = line[index] - '0';
    return intline;
}

int** read_map(ifstream &input, int &size) {
    std::string line;
    input >> line;
    size = line.length();
    int** map = new int*[size];

    for (int index = 0; index <= line.length(); index++) {
        map[index] = parse_line(line);
        input >> line;
    }

    return map;
}

int is_visible(int** tree_map, int size, int x, int y) {
    //check each direction
    int trawl = x - 1;
    while (trawl >= 0) {
        if (tree_map[y][trawl] >= tree_map[y][x])
            break;
        trawl--;
    }
    if (trawl == -1)
        return 1;
    
    trawl = x + 1;
    while (trawl < size) {
        if (tree_map[y][trawl] >= tree_map[y][x])
            break;
        trawl++;
    }
    if (trawl == size)
        return 1;
    
    
    trawl = y - 1;
    while (trawl >= 0) {
        if (tree_map[trawl][x] >= tree_map[y][x])
            break;
        trawl--;
    }
    if (trawl == -1)
        return 1;
    
    trawl = y + 1;
    while (trawl < size) {
        if (tree_map[trawl][x] >= tree_map[y][x])
            break;
        trawl++;
    }
    if (trawl == size)
        return 1;

    return 0;
}

int find_visibles(int** tree_map, int size) {
    int visibles = 0;
    for (int y = 0; y < size; y++) {
        for (int x = 0; x < size; x++) {
            visibles += is_visible(tree_map, size, x, y);
        }
    }
    return visibles;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    int size;
    int** tree_map = read_map(input, size);
    input.close();

    int result_part1 = find_visibles(tree_map, size);
    int result_part2 = 0;

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}