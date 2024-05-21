#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <chrono>
using namespace std;

std::unordered_map<std::string, std::pair<int, std::vector<std::string>>> exploration_space;
std::vector<std::string> node_names;
std::vector<std::vector<int>> path_weights;
std::unordered_map<std::string, int> valve_masks;

std::vector<std::string> extract_exploration(std::string line, std::string &name, int &relief) {
    std::vector<std::string> connections(0);
    int buff1, buff2;

    buff1 = line.find(' ') + 1;
    buff2 = line.find(' ', buff1);
    name = line.substr(buff1, buff2 - buff1);

    buff1 = line.find("rate=") + 5;
    buff2 = line.find(';', buff1);
    relief = std::stoi(line.substr(buff1, buff2 - buff1));

    buff1 = line.find("valve", buff1);
    buff1 = line.find(" ", buff1) + 1;
    while ((buff2 = line.find(",", buff1)) != std::string::npos) {
        connections.push_back(line.substr(buff1, buff2 - buff1));
        buff1 = line.find(" ", buff2) + 1;
    }
    connections.push_back(line.substr(buff1));
    return connections;
}

void load_space(ifstream &input) {
    std::string name, line;
    int relief;
    std::vector<std::string> connections;
    while (getline(input, line)) {
        connections = extract_exploration(line, name, relief);
        exploration_space[name] = make_pair(relief, connections);
    }

    int i = 0;
    for (std::pair<std::string, std::pair<int, std::vector<std::string>>> point : exploration_space) {
        if (point.second.first == 0)
            continue;
        valve_masks[point.first] = 1<<i++;
    }
}

std::vector<std::vector<int>> floyd_warshall() {
    std::vector<std::vector<int>> paths(node_names.size(), std::vector<int>(node_names.size(), INT_MAX / 4));

    for (int index = 0; index < node_names.size(); index++) {
        paths[index][index] = 0;
        for (std::string connection : exploration_space[node_names[index]].second) {
            int connect_index = std::find(node_names.begin(), node_names.end(), connection) - node_names.begin();
            paths[index][connect_index] = 1;
        }
    }

    for (int k = 0; k < node_names.size(); k++) {
        for (int i = 0; i < node_names.size(); i++) {
            for (int j = 0; j < node_names.size(); j++) {
                paths[i][j] = std::min(paths[i][j], paths[i][k] + paths[k][j]);
            }
        }
    }
    path_weights = paths;
    return path_weights;
}

int recursive_explore(std::string location, int time_remaining, std::vector<string> possible_visits) {
    int score = 0, index_here, index_there;
    std::string next_visit;

    index_here = std::find(node_names.begin(), node_names.end(), location) - node_names.begin();

    for (int i = 0; i < possible_visits.size(); i++) {
        next_visit = possible_visits[0];
        possible_visits.erase(possible_visits.begin());
        index_there = std::find(node_names.begin(), node_names.end(), next_visit) - node_names.begin();
        if (path_weights[index_here][index_there] < time_remaining) {
            score = std::max(score, recursive_explore(
                    next_visit, 
                    time_remaining - 1 - path_weights[index_here][index_there], 
                    possible_visits));
        }
        
        possible_visits.push_back(next_visit);
    }
    score += (exploration_space[location].first * time_remaining);
                
    return score;
}

std::map<int,int> explore(std::string location, int time_remaining, int visited_state, int current_flow, std::map<int,int> &results) {
    if (results.find(visited_state) == results.end())
        results[visited_state] = current_flow;
    else
        results[visited_state] = std::max(current_flow, results[visited_state]);
    
    int next_time, here_index = std::find(node_names.begin(), node_names.end(), location) - node_names.begin(), there_index;
    for (std::pair<std::string,int> mask : valve_masks) {
        there_index = std::find(node_names.begin(), node_names.end(), mask.first) - node_names.begin();
        next_time = time_remaining - path_weights[here_index][there_index] - 1;

        if (next_time <= 0 || (mask.second & visited_state) != 0)
            continue;
        
        explore(mask.first, next_time, visited_state | mask.second, current_flow + next_time * exploration_space[mask.first].first, results);
    }
    
    return results;
}

int explore_pt2() {
    std::map<int,int> results;
    int result = 0;
    results = explore("AA", 26, 0, 0, results);
    for (std::map<int,int>::iterator first = results.begin(); first != results.end(); first++) {
        for (std::map<int,int>::iterator second = results.begin(); second != results.end(); second++) {
            if ((first->first & second->first) != 0)
                continue;
            
            result = std::max(result, first->second + second->second);
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
    load_space(input);
    input.close();

    for (std::pair<std::string, std::pair<int, std::vector<std::string>>> spot : exploration_space) {
        node_names.push_back(spot.first);
    }
    floyd_warshall();

    std::vector<std::string> non_zeroes;
    for (std::pair<std::string, std::pair<int, std::vector<std::string>>> spot : exploration_space) {
        if (spot.second.first != 0)
            non_zeroes.push_back(spot.first);
    }
    
    auto t1 = std::chrono::high_resolution_clock::now();
    //std::cout << "Part 1: " << recursive_explore("AA", 30, non_zeroes) << std::endl;
    auto t2 = std::chrono::high_resolution_clock::now();
    std::cout << "Execution time: " << std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count() << std::endl;
    std::cout << "Part 2: " << explore_pt2() << std::endl;
    auto t3 = std::chrono::high_resolution_clock::now();
    std::cout << "Execution time: " << std::chrono::duration_cast<std::chrono::milliseconds>(t3 - t2).count() << std::endl;
    return 0;
}