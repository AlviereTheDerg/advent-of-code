#include <iostream>
#include <fstream>
#include <algorithm>
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

int see_left(int** tree_map, int size, int x, int y) {
    int trawl = x - 1;
    while (trawl >= 0) {
        if (tree_map[y][trawl--] >= tree_map[y][x])
            break;
    }
    return x - trawl - 1;
}

int see_right(int** tree_map, int size, int x, int y) {
    int trawl = x + 1;
    while (trawl < size) {
        if (tree_map[y][trawl++] >= tree_map[y][x])
            break;
    }
    return trawl - x - 1;
}

int see_up(int** tree_map, int size, int x, int y) {
    int trawl = y - 1;
    while (trawl >= 0) {
        if (tree_map[trawl--][x] >= tree_map[y][x])
            break;
    }
    return y - trawl - 1;
}

int see_down(int** tree_map, int size, int x, int y) {
    int trawl = y + 1;
    while (trawl < size) {
        if (tree_map[trawl++][x] >= tree_map[y][x])
            break;
    }
    return trawl - y - 1;
}

int is_visible(int** tree_map, int size, int x, int y) {
    //check each direction
    if (see_left(tree_map, size, x, y) == x && (tree_map[y][0] < tree_map[y][x] || x == 0))
        return 1;
    
    if (see_right(tree_map, size, x, y) == size - x - 1 && (tree_map[y][size - 1] < tree_map[y][x] || x == size - 1))
        return 1;
    
    if (see_up(tree_map, size, x, y) == y && (tree_map[0][x] < tree_map[y][x] || y == 0))
        return 1;
    
    if (see_down(tree_map, size, x, y) == size - y - 1 && (tree_map[size - 1][x] < tree_map[y][x] || y == size - 1))
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

int scenic_score(int** tree_map, int size, int x, int y) {
    int prod = 1;
    prod *= see_left(tree_map, size, x, y);
    prod *= see_right(tree_map, size, x, y);
    prod *= see_up(tree_map, size, x, y);
    prod *= see_down(tree_map, size, x, y);
    return prod;
}

int most_scenic(int** tree_map, int size) {
    int scenic = 0;
    for (int y = 0; y < size; y++) {
        for (int x = 0; x < size; x++) {
            scenic = max(scenic, scenic_score(tree_map, size, x, y));
        }
    }
    return scenic;
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
    int result_part2 = most_scenic(tree_map, size);

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}