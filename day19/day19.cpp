#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <set>
#include <map>
using namespace std;

typedef std::array<int,6> bp_type;
typedef std::array<int,8> state_data;

bp_type current_bp;
std::map<state_data,int> visiteds;
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
    results += (time_remaining - 2) * (time_remaining - 1) / 2;
    return results;
}

int calculate_max_geodes(state_data current_state, int time_remaining) {
    if (visiteds.count(current_state) != 0 && visiteds[current_state] >= time_remaining)
        return 0;
    visiteds[current_state] = time_remaining;

    int results = calculate_hypothetical_max(current_state, time_remaining);
    if (results < current_max)
        return 0;
    
    if (time_remaining <= 0)
        return current_state[7];
    
    state_data branches;
    results = 0;

    for (int i = 0; i < 4; i++) {
        if (!can_make_bot(current_state,i))
            continue;
        
        branches = current_state;
        tick_state(branches);
        make_bot(branches, i);
        results = max(results, calculate_max_geodes(branches, time_remaining - 1));
    }
    tick_state(current_state);
    results = max(results, calculate_max_geodes(current_state, time_remaining - 1));
    
    current_max = max(current_max, results);
    return results;
}

void reset_things(bp_type blueprint) {
    current_bp = blueprint;
    visiteds.clear();
    current_max = 0;
}

int calculate_part1(std::vector<bp_type> blueprints) {
    int results = 0, bp_number = 0;
    while (bp_number < blueprints.size()) {
        reset_things(blueprints[bp_number]);
        
        bp_number++;
        std::cout << "Checking bp " << bp_number << "/" << blueprints.size() << " ... ";
        results += bp_number * calculate_max_geodes({1,0,0,0, 0,0,0,0}, 24);
        std::cout << "done" << std::endl;
    }
    return results;
}

void testsample() {
    bp_type bp1 = {4,2,3,14,2,7};
    bp_type bp2 = {2,3,3,8,3,12};
    
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,2, 6,41,8,9}, 0) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,2, 5,37,6,7}, 1) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,2, 4,33,4,5}, 2) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,2, 3,29,2,3}, 3) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,1, 4,25,7,2}, 4) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,1, 3,21,5,1}, 5) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,4,2,1, 2,17,3,0}, 6) << std::endl;
    reset_things(bp1);
    std::cout << "M24: 9 " << calculate_max_geodes({1,0,0,0, 0,0,0,0}, 24) << std::endl;
    reset_things(bp2);
    std::cout << "M24: 12 " << calculate_max_geodes({1,0,0,0, 0,0,0,0}, 24) << std::endl;
    return;
}

int main() {
    //testsample();
    
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::vector<bp_type> blueprints = load_blueprints(input);
    input.close();

    int part1 = calculate_part1(blueprints);
    std::cout << "Part 1: " << part1 << std::endl;
    
    return 0;
}