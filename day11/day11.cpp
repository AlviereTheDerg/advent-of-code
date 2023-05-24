#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
using namespace std;

struct Monkey {
    std::vector<long long> items;
    char operation;
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
        monkey->operation = '^';
        monkey->op_aux = 2;
        return;
    }

    monkey->operation = func_operation[0];
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
    long long result;
    switch (monkey->operation) {
        case '^':
        result = value * value;
        break;
        case '+':
        result = value + monkey->op_aux;
        break;
        case '*':
        result = value * monkey->op_aux;
        break;
    }
    return result;
}

int check(struct Monkey* monkey, long long value) {
    return (value % monkey->check == 0) ? monkey->if_true : monkey->if_false;
}

int throw_all(std::vector<struct Monkey*> monkeys, int index, long long (*oper)(long long item, long long aux), long long auxiliary) {
    for (long long item : monkeys[index]->items) {
        item = inspect(monkeys[index], item);
        item = oper(item, auxiliary);
        int foo = check(monkeys[index], item);
        monkeys[foo]->items.push_back(item);
    }
    int amount = monkeys[index]->items.size();
    monkeys[index]->items.clear();
    return amount;
}

std::vector<int> monkey_round(std::vector<struct Monkey*> monkeys, long long (*oper)(long long item, long long aux), long long auxiliary) {
    std::vector<int> checks(monkeys.size(), 0);
    
    for (int index = 0; index < monkeys.size(); index++) {
        checks[index] += throw_all(monkeys, index, oper, auxiliary);
    }

    return checks;
}

long long monkey_business(int rounds, long long (*oper)(long long item, long long aux)) {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    std::vector<struct Monkey*> monkeys(0);
    while (!input.eof()) monkeys.push_back(make_a_monkey(input));
    input.close();

    std::vector<long long> inspections(monkeys.size(), 0);
    std::vector<int> current_round;
    long long auxiliary = 1;
    for (struct Monkey* monkey : monkeys)
        auxiliary *= monkey->check;
    
    for (int round = 0; round < rounds; round++) {
        current_round = monkey_round(monkeys, oper, auxiliary);
        for (int index = 0; index < inspections.size(); index++) {
            inspections[index] += current_round[index];
        }
    }
    std::sort(inspections.rbegin(), inspections.rend());
    return inspections[0] * inspections[1];
}

int main() {
    long long result_part1 = monkey_business(20, [](long long item, long long aux) {return (item) / 3;});
    long long result_part2 = monkey_business(10000, [](long long item, long long aux) {return item % aux;});
    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}