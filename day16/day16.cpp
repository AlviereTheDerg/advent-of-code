#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <chrono>
using namespace std;

std::unordered_map<std::string, std::pair<int, std::vector<std::string>>> exploration_space;
std::vector<std::string> node_names;
std::vector<std::vector<int>> path_weights;

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
    std::cout << "Part 1: " << recursive_explore("AA", 30, non_zeroes) << std::endl;
    auto t2 = std::chrono::high_resolution_clock::now();
    std::cout << "Execution time: " << std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count() << std::endl;
    return 0;
}