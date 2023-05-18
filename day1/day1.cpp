#include <iostream>
#include <fstream>
using namespace std;

int gather_calories(ifstream& input) {
    int calories = 0;
    char buffer[32];
    while (!input.getline(buffer, 32).fail() && atoi(buffer) != 0) {
        calories += atoi(buffer);
    }
    cout << "Elf calories: " << calories << endl;
    return calories;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        cout << "Unable to open file" << endl;
        return 0;
    }

    int highest = -1; //kludge starter value
    while (int next_elf = gather_calories(input)) {
        highest = std::max(highest, next_elf);
    }

    input.close();
    cout << "Most calories on single elf: " << highest << endl;
    return 0;
}