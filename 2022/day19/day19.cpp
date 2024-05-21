#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <chrono>
using namespace std;

typedef std::array<int,6> bp_type;
typedef std::array<int,8> state_data;

bp_type current_bp;
int current_max;

std::vector<std::string> split_string(std::string input) {
    std::vector<std::string> result;
    int one = 0, two = -1;
    
    while ((two = input.find(' ', one)) != -1) {
        result.push_back(input.substr(one, two-one));
        one = two + 1;
    }
    result.push_back(input.substr(one));

    return result;
}

bp_type make_blueprint(std::vector<std::string> input) {
    bp_type result = {0,0,0, 0,0,0};
    result[0] = stoi(input[6]); //ore needed for orebot
    result[1] = stoi(input[12]); //ore needed for claybot
    result[2] = stoi(input[18]); //ore needed for obsidianbot
    result[3] = stoi(input[21]); //clay needed for obsidianbot
    result[4] = stoi(input[27]); //ore needed for geodebot
    result[5] = stoi(input[30]); //obsidian needed for geodebot
    return result;
}

std::vector<bp_type> load_blueprints(ifstream &input) {
    std::vector<bp_type> blueprints;
    std::string line;
    while (getline(input, line)) {
        blueprints.push_back(make_blueprint(split_string(line)));
    }
    return blueprints;
}

bool can_make_bot(state_data current_state, int bot_type) {
    switch (bot_type) {
        case 0:
            return (current_state[4] >= current_bp[0]);
        case 1:
            return (current_state[4] >= current_bp[1]);
        case 2:
            return (current_state[4] >= current_bp[2] && current_state[5] >= current_bp[3]);
        case 3:
            return (current_state[4] >= current_bp[4] && current_state[6] >= current_bp[5]);
    }
    
    return false;
}

bool should_make_bot(state_data current_state, int bot_type) {
    switch (bot_type) {
        case 0:
            return (current_state[0] < std::max(std::max(current_bp[0], current_bp[1]), std::max(current_bp[2], current_bp[4])));
        case 1:
            return (current_state[1] < current_bp[3]);
        case 2:
            return (current_state[2] < current_bp[5]);
        case 3:
            return true;
    }
    return false;
}

bool make_bot(state_data &current_state, int bot_type) {
    if (!can_make_bot(current_state, bot_type))
        return false;
    
    current_state[bot_type]++;
    switch (bot_type) {
        case 0:
            current_state[4] -= current_bp[0];
            break;
        case 1:
            current_state[4] -= current_bp[1];
            break;
        case 2:
            current_state[4] -= current_bp[2];
            current_state[5] -= current_bp[3];
            break;
        case 3:
            current_state[4] -= current_bp[4];
            current_state[6] -= current_bp[5];
            break;
    }
    
    return true;
}

void tick_state(state_data &current_state) {
    for (int i = 0; i < 4; i++) {
        current_state[4 + i] += current_state[i];
    }
}

int calculate_hypothetical_max(state_data current_state, int time_remaining) {
    int results = current_state[3] * time_remaining + current_state[7];
    results += (time_remaining - 1) * (time_remaining) / 2;
    return results;
}

int calculate_max_geodes(state_data current_state, int time_remaining, std::array<bool,4> possibilities) {
    if (time_remaining <= 0)
        return current_state[7];
    
    if (calculate_hypothetical_max(current_state, time_remaining) <= current_max)
        return 0;
    
    state_data branches;
    int results = 0;
    std::array<bool,4> next_possibilities = {true,true,true,true};
    possibilities[3] = true;
    
    for (int i = 3; i >= 0; i--) {
        if (!possibilities[i] || !can_make_bot(current_state,i) || !should_make_bot(current_state,i))
            continue;
        
        next_possibilities[i] = false;
        branches = current_state;
        tick_state(branches);
        make_bot(branches,i);
        results = std::max(results, calculate_max_geodes(branches, time_remaining - 1, {true,true,true,true}));
    }

    tick_state(current_state);
    results = max(results, calculate_max_geodes(current_state, time_remaining - 1, next_possibilities));
    
    current_max = max(current_max, results);
    return results;
}

void reset_things(bp_type blueprint) {
    current_bp = blueprint;
    current_max = 0;
}

int calculate_part1(std::vector<bp_type> blueprints) {
    int results = 0, bp_number = 0;
    while (bp_number < blueprints.size()) {
        reset_things(blueprints[bp_number]);
        
        bp_number++;
        std::cout << "Checking bp " << bp_number << "/" << blueprints.size() << " ... ";
        results += bp_number * calculate_max_geodes({1,0,0,0, 0,0,0,0}, 24, {true,true,true,true});
        std::cout << "done" << std::endl;
    }
    return results;
}

int calculate_part2(std::vector<bp_type> blueprints) {
    int results = 1, bp_number = 0;
    while (bp_number < 3) {
        reset_things(blueprints[bp_number]);
        
        bp_number++;
        std::cout << "Checking bp " << bp_number << "/3 ... ";
        results *= calculate_max_geodes({1,0,0,0, 0,0,0,0}, 32, {true,true,true,true});
        std::cout << "done" << std::endl;
    }
    return results;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::vector<bp_type> blueprints = load_blueprints(input);
    input.close();

    auto t1 = std::chrono::high_resolution_clock::now();
    int part1 = calculate_part1(blueprints);
    auto t2 = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::milli> exec_double = t2 - t1;
    std::cout << "Part 1: " << part1 << std::endl;
    std::cout << "Part 1 execution time: " << exec_double.count() << "ms" << std::endl;

    t1 = std::chrono::high_resolution_clock::now();
    int part2 = calculate_part2(blueprints);
    t2 = std::chrono::high_resolution_clock::now();
    exec_double = t2 - t1;
    std::cout << "Part 2: " << part2 << std::endl;
    std::cout << "Part 2 execution time: " << exec_double.count() << "ms" << std::endl;
    
    return 0;
}