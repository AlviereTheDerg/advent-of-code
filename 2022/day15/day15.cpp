#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <algorithm>
using namespace std;

typedef std::array<int,4> sensor_data;

sensor_data read_sensor(std::string report_line) {
    int buff1, buff2, buff3, coord_x, coord_y;
    sensor_data sensor;
    std::string str_buff;

    buff1 = report_line.find("x=") + 2;
    buff2 = report_line.find(", y=") - buff1;
    str_buff = report_line.substr(buff1, buff2);
    sensor[0] = stoi(str_buff);

    buff2 += buff1 + 4;
    buff3 = report_line.find(": ") - buff2;
    str_buff = report_line.substr(buff2, buff3);
    sensor[1] = stoi(str_buff);

    buff3 = report_line.find("beacon");
    buff1 = report_line.find("x=", buff3) + 2;
    buff2 = report_line.find(", y=", buff3) - buff1;
    str_buff = report_line.substr(buff1, buff2);
    sensor[2] = stoi(str_buff);

    buff1 += buff2 + 4;
    str_buff = report_line.substr(buff1);
    sensor[3] = stoi(str_buff);

    return sensor;
}

std::vector<sensor_data> read_sensors(ifstream &input) {
    std::vector<sensor_data> sensors(0);
    std::string line;
    while (getline(input, line))
        sensors.push_back(read_sensor(line));
    return sensors;
}

int dist(sensor_data sn) {
    return std::abs(sn[0] - sn[2]) + std::abs(sn[1] - sn[3]);
}

std::vector<std::array<int,3>> read_data(std::vector<sensor_data> sensors) {
    std::vector<std::array<int,3>> data(sensors.size());
    for (int index = 0; index < sensors.size(); index++) {
        data[index] = { sensors[index][0], sensors[index][1], dist(sensors[index]) };
    }
    return data;
}

int part1_calculate(std::vector<std::array<int,3>> data, int y_value) {
    int smallest = INT_MAX, largest = INT_MIN, offset;
    for (std::array<int,3> datum : data) {
        offset = std::abs(y_value - datum[1]) - datum[2];
        largest = std::max(largest, datum[0] - offset);
        smallest = std::min(smallest, datum[0] + offset);
    }
    return largest - smallest;
}

std::pair<int,int> part2_helper(std::array<int,3> datum1, std::array<int,3> datum2) {
    std::pair<int,int> result;
    result.first = (datum2[0] + datum2[1] + datum2[2] + datum1[0] - datum1[1] - datum1[2]) / 2;
    result.second = 1 + (datum2[0] + datum2[1] + datum2[2] - datum1[0] + datum1[1] + datum1[2]) / 2;
    return result;
}

long long part2_calculate(std::vector<std::array<int,3>> data, int range) { //modified rotation method
    std::vector<std::array<int,2>> intersections(0);
    std::pair<int,int> result;
    bool check;
    long long datum_result = -1;
    for (int index1 = 0; index1 < data.size(); index1++) {
        for (int index2 = 0; index2 < data.size(); index2++) {
            result = part2_helper(data[index1], data[index2]);
            if (result.first < 0 || result.first > range)
                continue;
            
            if (result.second < 0 || result.second > range)
                continue;
            
            check = true;
            for (std::array<int,3> datum : data) {
                if (dist( {result.first, result.second, datum[0], datum[1]} ) > datum[2])
                    continue;
                check = false;
                break;
            }
            if (check) {
                datum_result = ((long long) range * result.first) + result.second;
                return datum_result;
            }
        }
    }
    return datum_result;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return -1;
    }
    std::vector<sensor_data> sensors = read_sensors(input);
    input.close();

    std::vector<std::array<int,3>> data = read_data(sensors);

    std::cout << "Part 1: " << part1_calculate(data, 2000000) << std::endl;
    std::cout << "Part 2: " << part2_calculate(data, 4000000) << std::endl;
    return 0;
}