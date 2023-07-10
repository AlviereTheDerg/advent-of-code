#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <tuple>
#include <chrono>
using namespace std;

typedef std::tuple<std::string, std::string, char> func_monkey_data;

std::map<std::string, long long> int_monkeys;
std::map<std::string, func_monkey_data> func_monkeys;

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

void make_monkey(std::vector<std::string> input) {
    input[0] = input[0].substr(0, 4);
    if (input.size() == 2) {
        int_monkeys[input[0]] = stoi(input[1]);
    } else if (input.size() == 4) {
        func_monkeys[input[0]] = make_tuple(input[1], input[3], input[2][0]);
    }
    return;
}

void load_monkeys(ifstream &input) {
    std::string line;
    while (getline(input, line))
        make_monkey(split_string(line));
}

long long read_monkey(std::string monkey_name);
long long monkey_function(func_monkey_data this_monkey) {
    long long operand1 = read_monkey(std::get<0>(this_monkey));
    long long operand2 = read_monkey(std::get<1>(this_monkey));

    switch (std::get<2>(this_monkey)) {
        case '+': return operand1 + operand2; break;
        case '-': return operand1 - operand2; break;
        case '*': return operand1 * operand2; break;
        case '/': return operand1 / operand2; break;
    }

    std::cout << "Something went wrong" << std::endl;
    return -1;
}

long long read_monkey(std::string monkey_name) {
    if (int_monkeys.count(monkey_name) != 0)
        return int_monkeys[monkey_name];
    
    if (func_monkeys.count(monkey_name) == 0) {
        std::cout << "Error happened" << std::endl;
        return -1;
    }
    
    long long holder = monkey_function(func_monkeys[monkey_name]);
    return holder;
}

long long part2_find() {
    long long holder1, holder2;
    long long change = 1000000000000, decrease = 10;
    std::get<2>(func_monkeys["root"]) = '-';
    
    int_monkeys["humn"] = 0;
    holder1 = read_monkey("root");
    int old_direction, curr_direction = 1;
    
    while (holder1 != 0) {
        holder2 = holder1;
        int_monkeys["humn"] += change * curr_direction;
        holder1 = read_monkey("root");
        old_direction = curr_direction;
        curr_direction = (holder1 >= 0) ? 1 : -1;
        if (curr_direction == old_direction)
            continue;
        change /= decrease;
    }
    return int_monkeys["humn"];
}

bool collapse_except(std::string current, std::string ignore) {
    if (current.compare(ignore) == 0)
        return false; //if current is the ignored one, say it couldn't collapse
    
    if (func_monkeys.count(current) == 0)
        return true; //if current isn't a function monkey, say it's collapsed
    
    if (!collapse_except(std::get<0>(func_monkeys[current]), ignore) || !collapse_except(std::get<1>(func_monkeys[current]), ignore))
        return false; //if a child couldn't collapse, say this one can't collapse
    
    int_monkeys[current] = read_monkey(current);
    func_monkeys.erase(current);
    return true;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    load_monkeys(input);
    input.close();
    
    auto t1 = std::chrono::high_resolution_clock::now();    
    collapse_except("root", "humn");
    auto t2 = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double, std::milli> exec_double = t2 - t1;
    std::cout << "Collapse execution time: " << exec_double.count() << "ms" << std::endl;
    
    t1 = std::chrono::high_resolution_clock::now();
    std::cout << "Part 1: " << read_monkey("root") << std::endl;
    t2 = std::chrono::high_resolution_clock::now();
    exec_double = t2 - t1;
    std::cout << "Part 1 execution time: " << exec_double.count() << "ms" << std::endl;
    
    
    t1 = std::chrono::high_resolution_clock::now();
    std::cout << "Part 2: " << part2_find() << std::endl;
    t2 = std::chrono::high_resolution_clock::now();
    exec_double = t2 - t1;
    std::cout << "Part 2 execution time: " << exec_double.count() << "ms" << std::endl;
    
    return 0;
}