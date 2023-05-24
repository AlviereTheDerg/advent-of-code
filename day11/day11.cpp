#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
using namespace std;

struct Monkey {
    std::vector<long long> items;
    long long (*operation)(long long,int);
    int op_aux;
    int check, if_true, if_false;
};

void trim(std::string &input, std::string trim_off) {
    input = input.substr(input.find(trim_off) + trim_off.length());
}

void load_items(struct Monkey* monkey, std::string items) {
    trim(items, "Starting items:");
    std::stringstream item_stream(items);
    std::string item;
    while (getline(item_stream, item, ',')) monkey->items.push_back(stoi(item));
    return;
}

void load_operation(struct Monkey* monkey, std::string func_operation) {
    trim(func_operation, "Operation: new = old ");
    if (func_operation.compare("* old") == 0) {
        monkey->operation = [] (long long old, int unused) {return old * old;};
        monkey->op_aux = 0;
        return;
    }

    if (func_operation[0] == '+')
        monkey->operation = [] (long long old, int operand) {return old + operand;};
    else
        monkey->operation = [] (long long old, int operand) {return old * operand;};
    
    monkey->op_aux = std::stoi(func_operation.substr(func_operation.find(" ")));
    return;
}

void load_check(struct Monkey* monkey, std::string test_line, std::string if_true_line, std::string if_false_line) {
    trim(test_line, "Test: divisible by ");
    trim(if_true_line, "If true: throw to monkey ");
    trim(if_false_line, "If false: throw to monkey ");

    monkey->check = stoi(test_line);
    monkey->if_true = stoi(if_true_line);
    monkey->if_false = stoi(if_false_line);

    return;
}

struct Monkey* make_a_monkey(ifstream &input) {
    std::string line;
    std::vector<std::string> args(0);
    while (getline(input, line) && line != "") args.push_back(line);
    struct Monkey* monkey = new Monkey();
    load_items(monkey, args[1]);
    load_operation(monkey, args[2]);
    load_check(monkey, args[3], args[4], args[5]);

    return monkey;
}

long long inspect(struct Monkey* monkey, long long value) {
    return monkey->operation(value, monkey->op_aux);
}

long long check(struct Monkey* monkey, long long value) {
    return (value % monkey->check == 0) ? monkey->if_true : monkey->if_false;
}

int throw_all(std::vector<struct Monkey*> monkeys, int index) {
    for (long long item : monkeys[index]->items) {
        std::cout << "Monkey " << index << ": ";
        std::cout << item << "->";
        item = inspect(monkeys[index], item);
        std::cout << item << "->";
        item /= 3;
        std::cout << item << ": choice ";
        int foo = check(monkeys[index], item);
        std::cout << foo << std::endl;
        monkeys[foo]->items.push_back(item);
    }
    int amount = monkeys[index]->items.size();
    monkeys[index]->items.clear();
    return amount;
}

std::vector<int> monkey_round(std::vector<struct Monkey*> monkeys) {
    std::vector<int> checks(monkeys.size(), 0);
    
    for (int index = 0; index < monkeys.size(); index++) {
        checks[index] += throw_all(monkeys, index);
    }

    return checks;
}

int monkey_business(std::vector<struct Monkey*> monkeys, int rounds) {
    std::vector<int> inspections(monkeys.size(), 0), current_round;
    int highest;
    
    for (int round = 0; round < rounds; round++) {
        highest = 0;
        std::cout << "Round " << round << std::endl;
        current_round = monkey_round(monkeys);
        for (int index = 0; index < inspections.size(); index++) {
            inspections[index] += current_round[index];
            for (int x : monkeys[index]->items)
                highest = std::max(highest, x);
        }
        std::cout << "Highest: " << highest << std::endl;
    }
    std::sort(inspections.begin(), inspections.end());
    return *(inspections.end() - 1) * *(inspections.end() - 2);
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    std::vector<struct Monkey*> monkeys(0);
    while (!input.eof()) monkeys.push_back(make_a_monkey(input));
    input.close();

    int result_part1 = monkey_business(monkeys, 20);
    std::cout << "Part 1: " << result_part1 << std::endl;
    return 0;
}