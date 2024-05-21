#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <map>
#include <set>
#include <chrono>
using namespace std;

typedef std::pair<int,int> location;

location operator+(const location l, const location r) {
    return {l.first+r.first, l.second+r.second};
}

std::vector<location> motion_directions{ {1,0}, {0,1}, {-1,0}, {0,-1}, {0,0} };

//blizzards[direction] is all blizzards moving in direction
//0 is right, 1 is down, 2 is left, 3 is up
std::array<std::vector<std::string>,4> blizzards{};

//all locations the elf group can reach by current estimate
std::set<location> reachables{};

location start{}, destination{};
int width{}, height{};

void load_data(ifstream &input) {
    std::string line;
    getline(input, line);
    start = {line.find('.') - 1, -1};
    width = line.size() - 2;
    std::vector<std::string> sample_thing{};
    while (getline(input, line)) {
        line = line.substr(1, width);
        if (line[1] == '#' || line[2] == '#') //if this is the last line
            break;
        sample_thing.push_back(line);
        std::string left{}, right{}, up{}, down{};
        for (char letter : line) {
            right.push_back((letter == '>') ? '>' : '.');
            down.push_back((letter == 'v') ? 'v' : '.');
            left.push_back((letter == '<') ? '<' : '.');
            up.push_back((letter == '^') ? '^' : '.');
        }
        blizzards[0].push_back(right);
        blizzards[1].push_back(down);
        blizzards[2].push_back(left);
        blizzards[3].push_back(up);
    }

    height = blizzards[0].size();
    destination = {line.find('.'), height};
    reachables.insert(start);
}

void tick_blizzards() {
    //tick right-facings
    for (int i = 0; i < blizzards[0].size(); i++) {
        blizzards[0][i] = blizzards[0][i].substr(width - 1) + blizzards[0][i].substr(0, width - 1);
    }

    //tick down-facings
    blizzards[1].insert(blizzards[1].begin(), blizzards[1].back());
    blizzards[1].erase(blizzards[1].end());
    
    //tick left-facings
    for (int i = 0; i < blizzards[2].size(); i++) {
        blizzards[2][i] = blizzards[2][i].substr(1) + blizzards[2][i].substr(0, 1);
    }

    //tick up-facings
    blizzards[3].push_back(blizzards[3].front());
    blizzards[3].erase(blizzards[3].begin());
}

bool can_go_to(location loc) {
    if (loc == start || loc == destination)
        return true; //if start or destination, good to go (no blizzards can get here)
    
    //if OOB, not good to go
    if (loc.first < 0 || loc.first >= width)
        return false;
    if (loc.second < 0 || loc.second >= height)
        return false;
    
    //check each blizzard, if blizzard present at location, no go
    for (int i = 0; i < 4; i++) {
        if (blizzards[i][loc.second][loc.first] != '.')
            return false;
    }
    return true;
}

bool update_locations(location dest) {
    std::set<location> next_locations;
    for (location possible : reachables)
        for (location offset : motion_directions)
            if (can_go_to(possible + offset))
                next_locations.insert(possible + offset);
    reachables = next_locations;
    return (reachables.count(dest) != 0);
}

char get_blizzard_char_at(location loc) {
    char result{'.'};
    int count{};
    for (int i = 0; i < 4; i++) {
        if (blizzards[i][loc.second][loc.first] == '.')
            continue;
        count++;
        result = blizzards[i][loc.second][loc.first];
    }
    if (count == 0 || count == 1)
        return result;
    else
        return '0' + count;
}

void print_map() {
    std::cout << '#';
    for (int x = 0; x < width; x++)
        std::cout << ((start.first != x) ? '#' : '.');
    std::cout << "#\n";

    for (int y = 0; y < height; y++) {
        std::cout << '#';
        for (int x = 0; x < width; x++) {
            std::cout << get_blizzard_char_at({x,y});
        }
        std::cout << "#\n";
    }

    std::cout << '#';
    for (int x = 0; x < width; x++)
        std::cout << ((destination.first != x) ? '#' : '.');
    std::cout << "#\n";
    
    std::cout << '\n';
    return;
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
    int i{};
    do {
        i++;
        tick_blizzards();
    } while (!update_locations(destination));
    auto t2 = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::milli> exec_double = t2 - t1;
    std::cout << "Part 1: " << i << '\n';
    std::cout << "Part 1 execution time: " << exec_double.count() << "ms" << std::endl;
    
    t1 = std::chrono::high_resolution_clock::now();
    reachables = {destination};
    do {
        i++;
        tick_blizzards();
    } while (!update_locations(start));
    reachables = {start};
    do {
        i++;
        tick_blizzards();
    } while (!update_locations(destination));
    t2 = std::chrono::high_resolution_clock::now();
    exec_double = t2 - t1;
    std::cout << "Part 2: " << i << '\n';
    std::cout << "Part 2 execution time: " << exec_double.count() << "ms" << std::endl;
    
    
    return 0;
}