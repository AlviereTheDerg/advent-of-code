'''
Created on Jun 1, 2023

@author: Alviere
'''
import re
input_data = [re.split(r'[\s=;,]+', line) for line in open("input.txt", "r").read().splitlines()]

connections = { line[1]:set(line[10:]) for line in input_data }
valve_flows = { line[1]:int(line[5]) for line in input_data if int(line[5]) != 0 }
valve_masks = { valve: 1<<index for index, valve in enumerate(valve_flows) }
path_distances = { x:
                    {y: 0 if x==y else 1 if y in connections[x] else float('inf') for y in connections }
                for x in connections}

for k in path_distances:
    for i in path_distances:
        for j in path_distances:
            path_distances[i][j] = min(path_distances[i][j], path_distances[i][k] + path_distances[k][j])

def explore(location, time_remaining, visited_state, current_flow, results):
    if (visited_state in results):
        results[visited_state] = max(results[visited_state], current_flow)
    else:
        results[visited_state] = current_flow
    
    for valve in valve_flows:
        next_time = time_remaining - path_distances[location][valve] - 1
        
        if (next_time <= 0 or (valve_masks[valve] & visited_state) != 0):
            continue
        
        explore(valve, 
                next_time, 
                valve_masks[valve] | visited_state, 
                current_flow + next_time * valve_flows[valve], 
                results)
    
    return results

all_visits = explore("AA", 26, 0, 0, {})
result = max( relief1+relief2 for valves1,relief1 in all_visits.items() for valves2,relief2 in all_visits.items() if (valves1&valves2 == 0) )
print("Part 2: {}".format(result))