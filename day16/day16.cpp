#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_map>
using namespace std;

std::unordered_map<std::string, std::pair<int, std::vector<std::string>>> exploration_space;

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

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    load_space(input);
    input.close();

    return 0;
}