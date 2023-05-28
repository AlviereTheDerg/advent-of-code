#include <iostream>
#include <fstream>
//#include <sstream>
#include <vector>
using namespace std;

size_t find_item_bound(std::string input_list, size_t start) {
    int layer = 0;
    for (size_t end = start; end < input_list.length(); end++) {
        if (input_list[end] == '[') {
            layer++;
        } else if (input_list[end] == ']') {
            layer--;
        } else if (input_list[end] == ',' && layer == 0)
            return end;
        
        if (layer < 0)
            return end;
    }

    return string::npos;
}

std::vector<std::string> split_list(std::string input_list) {
    size_t start = 0, end;
    std::string section;
    std::vector<std::string> result(0);
    input_list = input_list.substr(1, input_list.length() - 2);

    while ((end = find_item_bound(input_list, start)) != string::npos) {
        section = input_list.substr(start, end - start);
        start = end + 1;
        result.push_back(section);
    }
    result.push_back(input_list.substr(start));
    if (result.size() == 1 && result[0] == "") result.clear();
    return result;
}

int compare(std::string left, std::string right) {
    if (left[0] != '[' && right[0] != '[') //for int:int
        return (stoi(left) < stoi(right)) - (stoi(left) > stoi(right));
        //if left < right, return 1, if left = right, return 0, if left > right, return -1
    
    if (left[0] != '[' && right[0] == '[') //for int:list
        return compare("[" + left + "]", right); //replace left with a list, compare
    if (left[0] == '[' && right[0] != '[') //for list:int
        return compare(left, "[" + right + "]"); //replace right with a list, compare
    
    //for list:list
    std::vector<std::string> left_v = split_list(left), right_v = split_list(right);
    for (int index = 0; index < std::min(left_v.size(), right_v.size()); index++) {
        switch(compare(left_v[index], right_v[index])) {
            case -1: return -1;
            case 0: break;
            case 1: return 1;
        }
    }

    return (left_v.size() < right_v.size()) - (left_v.size() > right_v.size()); 
    //if left shorter, 1, if equal length, 0, if right shorter, -1
}

int part1() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    int result = 0, pair = 0;

    std::string left, right, foo;
    while (!input.eof()) {
        getline(input, left);
        getline(input, right);
        getline(input, foo);
        pair++;

        result += pair * ((compare(left, right) > 0) ? 1 : 0);
    }

    input.close();
    return result;
}

int main() {
    std::cout << "Part 1: " << part1() << std::endl;
}