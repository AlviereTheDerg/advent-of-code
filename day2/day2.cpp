#include <iostream>
#include <fstream>
using namespace std;

enum Play { rock=-1, paper=0, scissors=1 };

int calculate_score_part1(Play opponent, Play self) {
    int score = (self + 2);
    switch((3 + opponent - self) % 3) { //kludge fix, C++ doesn't always have -1%3 = 2
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

int calculate_score_part2(Play opponent, Play win) {
    int score = (win + 1) * 3; //rock=-1 -> 0; paper=0 -> 1 -> 3; scissors=1 -> 2 -> 6;
    switch (win) {
        case -1: //lose, -1->3, 0->1, 1->2
            score += (opponent + 3) % 3 + 1;
            break;
        case 0: //tie
            score += (opponent + 2);
            break;
        case 1: //win, -1->3, 0->1, 1->2
            score += (opponent + 2) % 3 + 1;
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

    int score_part1 = 0, score_part2 = 0;
    Play* match = NULL;
    while (!input.eof()) {
        match = extract_plays(input);
        score_part1 += calculate_score_part1(match[0], match[1]);
        score_part2 += calculate_score_part2(match[0], match[1]);
        delete match;
    }
    input.close();

    std::cout << "Part 1: " << score_part1 << std::endl;
    std::cout << "Part 2: " << score_part2 << std::endl;
    return 0;
}