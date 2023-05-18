#include <iostream>
#include <fstream>
using namespace std;

enum Play { rock=-1, paper=0, scissors=1 };

int calculate_score(Play opponent, Play self) {
    int score = (self + 2);
    switch((3 + opponent - self) % 3) {
        case 0: //tie
            score += 3;
            break;
        case 1: //lose
            score += 0;
            break;
        case 2: //win
            score += 6;
            break;
    }
    return score;
}

Play* extract_plays(ifstream& input) {
    char buffer[4];
    input.getline(buffer, 4);
    Play* result = new Play[2];

    switch(buffer[0]) {
        case 'A': result[0] = rock; break;
        case 'B': result[0] = paper; break;
        case 'C': result[0] = scissors; break;
    }

    switch(buffer[2]) {
        case 'X': result[1] = rock; break;
        case 'Y': result[1] = paper; break;
        case 'Z': result[1] = scissors; break;
    }

    return result;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }

    int score_total = 0;
    Play* match = NULL;
    while (!input.eof()) {
        match = extract_plays(input);
        score_total += calculate_score(match[0], match[1]);
        delete match;
    }
    input.close();

    std::cout << "Part 1: " << score_total << std::endl;
    return 0;
}