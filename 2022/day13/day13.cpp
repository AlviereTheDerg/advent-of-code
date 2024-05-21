#include <iostream>
#include <fstream>
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

size_t part2_insert(std::vector<std::string> &string_list, std::string new_string, size_t start_index) {
    size_t location = 0;
    for (location = start_index; location < string_list.size(); location++) {
        switch(compare(new_string, string_list[location])) {
            case -1: continue;
            case 0: case 1:
                string_list.insert(string_list.begin() + location, new_string);
                return location;
        }
    }
    string_list.push_back(new_string);
    return location;
}

int part2_calculate(std::vector<std::string> string_list) {
    int result = -1;
    for (int index = 0; index < string_list.size(); index++) {
        if (result == -1 && string_list[index].compare("[[2]]") == 0)
            result = index + 1;
        else if (result != -1 && string_list[index].compare("[[6]]") == 0) {
            result *= index + 1;
            return result;
        }
    }
    return -1;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    int result_part1 = 0, comp_result, pair = 0;
    size_t index_holder;

    std::vector<std::string> result_part2(0);
    index_holder = part2_insert(result_part2, "[[2]]", 0);
    part2_insert(result_part2, "[[6]]", index_holder);

    std::string left, right, foo;
    while (!input.eof()) {
        getline(input, left);
        getline(input, right);
        getline(input, foo);
        pair++;
        comp_result = compare(left, right);

        result_part1 += pair * ((comp_result > 0) ? 1 : 0);
        if (comp_result == 1) {
            index_holder = part2_insert(result_part2, left, 0);
            part2_insert(result_part2, right, index_holder);
        } else {
            index_holder = part2_insert(result_part2, right, 0);
            part2_insert(result_part2, left, index_holder);
        }
    }
    input.close();

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << part2_calculate(result_part2) << std::endl;
}