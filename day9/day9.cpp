#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <unordered_set>

namespace std {
    template<typename T, size_t N>
    struct hash<array<T, N>> {
        typedef array<T, N> argument_type;
        typedef size_t result_type;

        result_type operator()(const argument_type& a) const {
            hash<T> hasher;
            result_type h = 0;
            for (result_type i = 0; i < N; i++) {
                h = h * 31 + hasher(a[i]);
            }
            return h;
        }
    };
}


std::array<int, 2> update_tail(std::array<int,2> head, std::array<int,2> &tail) {
    if (abs(head[0] - tail[0]) <= 1 && abs(head[1] - tail[1]) <= 1)
        return tail;
    
    if (head[0] < tail[0]) tail[0]--;
    if (head[0] > tail[0]) tail[0]++;
    if (head[1] < tail[1]) tail[1]--;
    if (head[1] > tail[1]) tail[1]++;
    return tail;
}

void merge_sets(std::unordered_set<std::array<int,2>> &destination, std::unordered_set<std::array<int,2>> source) {
    destination.insert(source.begin(), source.end());
}

std::array<int,2> tick_and_propagate(std::array<int,2> offset, std::vector<std::array<int,2>> &thread) {
    thread[0][0] += offset[0]; thread[0][1] += offset[1];
    std::array<int, 2> tail;
    for (int index = 0; index < thread.size() - 1; index++) {
        tail = update_tail(thread[index], thread[index + 1]);
    }
    return tail;
}

int track_tail(int length) {
    std::ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    std::vector<std::array<int,2>> thread(length, {0,0});
    std::array<int,2> offset;
    std::string dir, amount_str;
    int amount;
    std::unordered_set<std::array<int,2>> visited;

    while (input >> dir >> amount_str) {
        amount = stoi(amount_str);
        switch(dir[0]) {
            case 'U': offset = { 0, 1}; break;
            case 'L': offset = {-1, 0}; break;
            case 'R': offset = { 1, 0}; break;
            case 'D': offset = { 0,-1}; break;
        }
        for (int index = 0; index < amount; index++) {
            visited.insert(tick_and_propagate(offset, thread));
        }
    }
    input.close();

    return visited.size();
}

int main() {

    int result_part1 = track_tail(2);
    int result_part2 = track_tail(10);

    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}