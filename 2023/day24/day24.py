
import numpy as np
from math import isclose

hailstone_information = [(tuple(int(dimension) for dimension in position.split(', ')), tuple(int(dimension) for dimension in velocity.split(', '))) 
                         for position, velocity in (line.strip().split(" @ ") for line in open("day24/day24.txt"))]
#print(hailstone_information)

def reduce(array:np.ndarray): #honors topics in linalg don't fail me now
    for top_row in range(array.shape[0]):
        for row in range(top_row, array.shape[0]):
            if array[row][top_row] != 0:
                array[top_row] += array[row]
                break
        if array[top_row][top_row] == 0:
            break
        array[top_row] /= array[top_row][top_row]
        for row in range(0, array.shape[0]):
            if row == top_row:
                continue
            array[row] -= array[top_row] * array[row][top_row]
    else:
        return array[:,-1]
    return None

lower_bound = 200000000000000
upper_bound = 400000000000000
result = 0
for index, first_stone in enumerate(hailstone_information):
    for second_stone in hailstone_information[index+1:]:
        #print(first_stone, second_stone)
        px1, py1 = first_stone[0][0:2]
        vx1, vy1 = first_stone[1][0:2]
        px2, py2 = second_stone[0][0:2]
        vx2, vy2 = second_stone[1][0:2]
        stone_array = np.asarray([[-vx1,vx2,px1-px2],[-vy1,vy2,py1-py2]], dtype=float)
        reduced = reduce(stone_array)
        if type(reduced) != type(None):
            x1,y1 = px1+vx1*reduced[0], py1+vy1*reduced[0]
            x2,y2 = px2+vx2*reduced[1], py2+vy2*reduced[1]
            if isclose(x1,x2) and isclose(y1,y2):
                if lower_bound <= x1 and x1 <= upper_bound and lower_bound <= y1 and y1 <= upper_bound:
                    if reduced[0] >= 0 and reduced[1] >= 0:
                        result += 1
print(result)

trajectory_construct_xy = np.zeros((4,5))
trajectory_construct_xz = np.zeros((4,5))
for index in range(4):
    first_stone = hailstone_information[index]
    px1,py1,pz1 = first_stone[0][0:3]
    vx1,vy1,vz1 = first_stone[1][0:3]
    second_stone = hailstone_information[index+1]
    px2,py2,pz2 = second_stone[0][0:3]
    vx2,vy2,vz2 = second_stone[1][0:3]
    trajectory_construct_xy[index][0] = vy2 - vy1
    trajectory_construct_xy[index][1] = vx1 - vx2
    trajectory_construct_xy[index][2] = py1 - py2
    trajectory_construct_xy[index][3] = px2 - px1
    trajectory_construct_xy[index][4] = (py1*vx1 - py2*vx2) - (px1*vy1 - px2*vy2)

    trajectory_construct_xz[index][0] = vz2 - vz1
    trajectory_construct_xz[index][1] = vx1 - vx2
    trajectory_construct_xz[index][2] = pz1 - pz2
    trajectory_construct_xz[index][3] = px2 - px1
    trajectory_construct_xz[index][4] = (pz1*vx1 - pz2*vx2) - (px1*vz1 - px2*vz2)
trajectory_answer_xy = reduce(trajectory_construct_xy)
trajectory_answer_xz = reduce(trajectory_construct_xz)
print(trajectory_answer_xy)
print(trajectory_answer_xz)
print(int(trajectory_answer_xy[0]), int(trajectory_answer_xy[1]), int(trajectory_answer_xz[1]))
print(int(trajectory_answer_xy[0]) + int(trajectory_answer_xy[1]) + int(trajectory_answer_xz[1]))