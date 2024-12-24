
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug, Clone)]
struct Circuits {
    pub values: HashMap<String, bool>,
    pub gate_type: HashMap<String, String>,
    pub inputs: HashMap<String, HashSet<String>>,
    pub outputs: HashMap<String, HashSet<String>>,
}
impl Circuits {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            gate_type: HashMap::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, wire: &str, value: bool) {
        self.values.insert(String::from(wire), value);
    }

    pub fn update_gate(&mut self, gate_name: &str) -> bool {
        if let Some(value) = self.values.get(gate_name) {return *value;}

        let mut inputs = self.inputs.get(gate_name).unwrap().iter();
        let input_one_s = inputs.next().unwrap().clone();
        let input_two_s = inputs.next().unwrap().clone();
        let input_one = self.update_gate(&input_one_s);
        let input_two = self.update_gate(&input_two_s);

        let operation_s = self.gate_type.get(gate_name).unwrap();
        let operation = 
            if operation_s == "OR" {|a, b| a | b}
            else if operation_s == "AND" {|a, b| a & b}
            else if operation_s == "XOR" {|a, b| a ^ b}
            else {println!("uh oh"); return false};
        let result = operation(input_one, input_two);
        //println!("updating {gate_name} ({input_one_s} {} {operation_s} {input_two_s} {}) to {}", if input_one {"1"} else {"0"}, if input_two {"1"} else {"0"}, if result {"1"} else {"0"});
        self.values.insert(String::from(gate_name), result);
        result
    }

    pub fn add_gate(&mut self, gate_name: &str, input_one: &str, input_two: &str, gate_type: &str) {
        self.gate_type.insert(String::from(gate_name), String::from(gate_type));

        self.inputs.entry(String::from(gate_name))
            .and_modify(|v| {v.insert(String::from(input_one)); v.insert(String::from(input_two));})
            .or_insert({
                let mut inputs = HashSet::new();
                inputs.insert(String::from(input_one));
                inputs.insert(String::from(input_two));
                inputs
            });
        
        self.outputs.entry(String::from(input_one))
            .and_modify(|v| {v.insert(String::from(gate_name));})
            .or_insert({
                let mut inputs = HashSet::new();
                inputs.insert(String::from(gate_name));
                inputs
            });
        self.outputs.entry(String::from(input_two))
            .and_modify(|v| {v.insert(String::from(gate_name));})
            .or_insert({
                let mut inputs = HashSet::new();
                inputs.insert(String::from(gate_name));
                inputs
            });
    }

    pub fn update_all_gates(&mut self) {
        let all_gates = self.gate_type.keys().map(|s| s.clone()).collect::<Vec<_>>();
        for gate in all_gates {
            if self.values.contains_key(&gate) {continue}
            self.update_gate(&gate);
        }
    }
}

fn part1(input: &Circuits) {
    let mut z_gates = input.values.iter().filter(|(name, _)| name.chars().next().unwrap() == 'z').collect::<Vec<_>>();
    z_gates.sort();
    z_gates.reverse(); //30 fucking minutes to realize
    //println!("{z_gates:?}");
    let result = z_gates.iter().map(|(_, v)| if **v {1} else {0}).fold(0isize, |acc, bit| (acc << 1) + bit);
    println!("{result}");
}

fn part2(input: &Circuits) {
    let mut flagged: Vec<&str> = vec![];
    // 3 types of gate sets for ripple adder
    
    // start:
    // x00 ^ y00 -> z00
    // x00 & y00 -> c(arry out)

    // middle:
    // x ^ y -> A
    // cin ^ A -> z
    // cin & A -> B
    // x & y -> C
    // B | C -> cout

    // end:
    // x44 ^ y44 -> A
    // cin ^ A -> z44
    // cin & A -> B
    // x44 & y44 -> C
    // B | C -> z45
    for (gate_name, gate_type) in input.gate_type.iter() {
        let mut inputs = input.inputs.get(gate_name).unwrap().iter().collect::<Vec<_>>();
        inputs.sort();

        let from_xy = 
            inputs.get(0).unwrap().chars().nth(0) == Some('x') ||
            inputs.get(1).unwrap().chars().nth(0) == Some('y');

        // x00 ^ y00 -> z00 gets a pass
        if *inputs.get(0).unwrap() == "x00" && gate_type == "XOR" && gate_name == "z00" {
            continue;
        }
        // x00 & y00 -> ??? gets a pass
        if *inputs.get(0).unwrap() == "x00" && gate_type == "AND" {
            continue;
        }
        // ??? | ??? -> z45 gets a pass
        if gate_name == "z45" && gate_type == "OR" {
            continue;
        }

        // middle:
        // x ^ y -> A
        // cin ^ A -> z
        // cin & A -> B
        // x & y -> C
        // B | C -> cout

        // end:
        // x44 ^ y44 -> A
        // cin ^ A -> z44
        // cin & A -> B
        // x44 & y44 -> C
        

        // cin ^ A -> z
        // cin ^ A -> z44
        // remaining z gates MUST:
        if gate_name.chars().nth(0) == Some('z') {
            // be XOR gates
            if gate_type != "XOR" {
                //println!("z but !^: {gate_name}");
                flagged.push(gate_name);
            }
            // not have xy as input
            else if from_xy {
                //println!("from xy and to z: {gate_name}");
                flagged.push(gate_name);
            }
            continue;
        }

        // middle:
        // x ^ y -> A
        // cin & A -> B
        // x & y -> C
        // B | C -> cout

        // end:
        // x44 ^ y44 -> A
        // cin & A -> B
        // x44 & y44 -> C

        
        // x ^ y -> A
        // x44 ^ y44 -> A
        // remaining XOR gates MUST:
        if gate_type == "XOR" {
            // have xy as input
            if !from_xy {
                //println!("XOR not from xy: {gate_name}");
                flagged.push(gate_name);
            }
            // have another XOR gate as an output (that gate should be to a z bit but that gate might be mixed up)
            else if !input.outputs.get(gate_name).unwrap().iter().any(|gate_name| input.gate_type.get(gate_name).unwrap() == "XOR") {
                //println!("^->!^: {gate_name}");
                flagged.push(gate_name);
            }
            continue;
        }

        // middle:
        // cin & A -> B
        // x & y -> C
        // B | C -> cout

        // end:
        // cin & A -> B
        // x44 & y44 -> C

        // AND gates MUST:
        if gate_type == "AND" {
            // be used in an OR gate
            if !input.outputs.get(gate_name).unwrap().iter().any(|gate_name| input.gate_type.get(gate_name).unwrap() == "OR") {
                //println!("&->!|: {gate_name}");
                flagged.push(gate_name);
            }
            continue;
        }
        // at this point, it worked for me
    }
    flagged.sort();
    println!("{}", flagged.join(","));
}

pub fn main() {
    let input = crate::grab_input("day24");

    let mut circuits = Circuits::new();

    let mut input_split = input.split("\n\n");
    let value_grabber = Regex::new(r#"(.{3}): (\d)"#).unwrap();
    for value_capture in value_grabber.captures_iter(input_split.next().unwrap()) {
        let (_, [gate_name, gate_value]) = value_capture.extract();
        circuits.set_value(gate_name, gate_value == "1");
    }
    //println!("{:?}", circuits.values);

    let gate_grabber = Regex::new(r#"(.+) (XOR|OR|AND) (.+) -> (.+)"#).unwrap();
    for gate_capture in gate_grabber.captures_iter(input_split.next().unwrap()) {
        let (_, [input_one, gate_type, input_two, gate_name]) = gate_capture.extract();
        circuits.add_gate(gate_name, input_one, input_two, gate_type);
    }

    /*for gate in circuits.gate_type.keys() {
        let mut inputs = circuits.inputs.get(gate).unwrap().iter();
        let input_one = inputs.next().unwrap();
        let input_two = inputs.next().unwrap();
        let gate_type = circuits.gate_type.get(gate).unwrap();
        println!("{input_one} {gate_type} {input_two} -> {gate}");
    }*/

    circuits.update_all_gates();
    //let mut values = circuits.values.iter().collect::<Vec<_>>();
    //values.sort();
    //println!("{:?}", values);
    
    part1(&circuits);
    part2(&circuits);
}