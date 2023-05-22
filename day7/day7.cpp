#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <iterator>
#include <cassert>
using namespace std;

struct vfile {
    std::string name;
    int size;
};

struct vdir {
    std::string name;
    int size;

    struct vdir* parent;
    std::vector<struct vdir*> children;
    std::vector<struct vfile*> files;
};

void cd(struct vdir** current_dir, std::string dir_name) {
    if (dir_name == "..") {
        *current_dir = (*current_dir)->parent;
        return;
    }
    std::vector<struct vdir*>::iterator subdir = std::find_if(
            ((*current_dir)->children).begin(), 
            ((*current_dir)->children).end(), 
            [dir_name] (const struct vdir *dir) { return dir_name.compare((dir)->name) == 0; }
            );
    if (subdir == (*current_dir)->children.end())
        return;
    
    *current_dir = *subdir;
    return;
}

struct vfile* touch(std::string file_name, int file_size) {
    struct vfile* new_file = new vfile;
    new_file->name = file_name;
    new_file->size = file_size;
    return new_file;
}

struct vdir* mkdir(struct vdir* parent, std::string dir_name) {
    struct vdir* new_dir = new vdir;
    new_dir->parent = parent;
    new_dir->name = dir_name;
    new_dir->size = 0;
    new_dir->children.clear();
    new_dir->files.clear();
    return new_dir;
}

struct vdir* construct_vfs(ifstream& input) {
    int state = 0, aux;
    struct vdir *root = NULL, *cur_dir = NULL;
    cur_dir = root = mkdir(NULL, "/");
    std::string arg;

    input >> arg >> arg >> arg; //remove leading "$ cd /"
    while (input >> arg) {
        switch(state) {
            case 0: //new command
            assert(arg == "$");
            state = 1;
            break;

            case 1: //$ entered
            assert(arg == "ls" || arg == "cd");
            if (arg == "cd")
                state = 2;
            else
                state = 3;
            break;

            case 2: //change directory
            cd(&cur_dir, arg);
            state = 0; //reset back to expecting $
            break;

            case 3: //list files
            assert(arg == "$" || arg == "dir" || stoi(arg));
            if (arg == "$") //new command
                state = 1;
            else if (arg == "dir") //directory found
                state = 4;
            else { //listing a file
                aux = stoi(arg);
                state = 5;
            }
            break;

            case 4: //adding directory
            cur_dir->children.push_back(mkdir(cur_dir, arg));
            state = 3;
            break;

            case 5: //adding file
            cur_dir->files.push_back(touch(arg, aux));
            aux = 0;
            state = 3;
            break;
        }
    }

    return root;
}

int calculate_dir_size(struct vdir* directory) {
    int size = 0;
    for (std::vector<struct vfile*>::iterator iter = directory->files.begin(); iter != directory->files.end(); iter++)
        size += (*iter)->size;
    
    for (std::vector<struct vdir*>::iterator iter = directory->children.begin(); iter != directory->children.end(); iter++)
        size += calculate_dir_size(*iter);
    
    directory->size = size;
    return size;
}

void teardown(struct vdir* directory) {
    for (std::vector<struct vfile*>::iterator iter = directory->files.begin(); iter != directory->files.end(); iter++)
        delete (*iter);
    
    for (std::vector<struct vdir*>::iterator iter = directory->children.begin(); iter != directory->children.end(); iter++)
        teardown(*iter);
    
    delete directory;
}

int sum_of_directories_of_at_most_1e6(struct vdir* directory) {
    int sum = 0;
    if (directory->size <= 100000)
        sum += directory->size;
    
    for (std::vector<struct vdir*>::iterator iter = directory->children.begin(); iter != directory->children.end(); iter++)
        sum += sum_of_directories_of_at_most_1e6(*iter);

    return sum;
}

int main() {
    ifstream input("input.txt");
    if (!input.is_open()) {
        std::cout << "Unable to open file" << std::endl;
        return 0;
    }
    struct vdir* root = construct_vfs(input);
    input.close();

    int total = calculate_dir_size(root);
    int result_part1 = sum_of_directories_of_at_most_1e6(root);
    int result_part2 = 0;

    teardown(root);
    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;
    return 0;
}