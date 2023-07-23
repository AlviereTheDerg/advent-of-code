#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

typedef std::string snafu;
typedef long long decimal;

decimal snafu_to_decimal(snafu input) {
    decimal result{};
    for (char digit : input) {
        result *= 5;
        switch(digit) {
            case '0': break;
            case '1': result += 1; break;
            case '2': result += 2; break;
            case '-': result -= 1; break;
            case '=': result -= 2; break;
        }
    }
    return result;
}

snafu decimal_to_snafu(decimal input) {
    snafu result{};
    while (input > 0) {
        char next{};
        switch (input % 5) {
            case 0: next = '0'; break;
            case 1: next = '1'; break;
            case 2: next = '2'; break;
            case 3: next = '='; input += 5; break;
            case 4: next = '-'; input += 5; break;
        }
        result.insert(result.begin(), next);
        input /= 5;
    }
    return result;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::string line{};
    decimal result{};
    while (getline(input, line)) {
        result += snafu_to_decimal(line);
    }
    input.close();

    std::cout << "Part 1: " << decimal_to_snafu(result) << '\n';
    
    return 0;
}