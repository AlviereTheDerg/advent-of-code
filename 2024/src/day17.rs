
use regex::Regex;

#[derive(Debug, Clone)]
struct Computer {
    pub reg_a: usize,
    pub reg_b: usize,
    pub reg_c: usize,
    pc: usize,
    out: Vec<usize>,
}
impl Computer {
    pub fn initialize(reg_a: usize, reg_b: usize, reg_c: usize) -> Self {
        Self{
            reg_a,
            reg_b,
            reg_c,
            pc: 0,
            out: vec![],
        }
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => {println!("UH FUCKING OH"); usize::MAX},
        }
    }

    fn adv(&mut self, operand: usize) -> bool {
        self.reg_a /= (2usize).pow(self.get_combo_operand(operand) as u32);
        true
    }

    fn bxl(&mut self, operand: usize) -> bool {
        self.reg_b ^= operand;
        true
    }

    fn bst(&mut self, operand: usize) -> bool {
        self.reg_b = self.get_combo_operand(operand) % 8;
        true
    }

    fn jnz(&mut self, operand: usize) -> bool {
        if self.reg_a == 0 {true}
        else {
            self.pc = operand;
            false
        }
    }

    fn bxc(&mut self, _: usize) -> bool {
        self.reg_b ^= self.reg_c;
        true
    }

    fn out(&mut self, operand: usize) -> bool {
        self.out.push(self.get_combo_operand(operand) % 8);
        true
    }

    fn bdv(&mut self, operand: usize) -> bool {
        self.reg_b = self.reg_a / (2usize).pow(self.get_combo_operand(operand) as u32);
        true
    }

    fn cdv(&mut self, operand: usize) -> bool {
        self.reg_c = self.reg_a / (2usize).pow(self.get_combo_operand(operand) as u32);
        true
    }

    pub fn run(&mut self, instructions: &Vec<usize>) {
        while let Some(opcode) = instructions.get(self.pc) {
            let operand = *instructions.get(self.pc + 1).unwrap();
            if match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => {break}
            } {self.pc += 2};
        }
    }

    pub fn get_out(&self) -> &Vec<usize> {
        &self.out
    }
}

fn part1(mut computer: Computer, instructions: &Vec<usize>) {
    computer.run(instructions);
    let result = computer.get_out().iter().map(usize::to_string).collect::<Vec<String>>().join(",");
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day17");
    let capturer = Regex::new(r#"^Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)\n$"#).unwrap();
    let (_, [reg_a, reg_b, reg_c, instructions]) = capturer.captures(&input).unwrap().extract();
    let computer = {
        let reg_a = reg_a.parse().unwrap();
        let reg_b = reg_b.parse().unwrap();
        let reg_c = reg_c.parse().unwrap();

        Computer::initialize(reg_a, reg_b, reg_c)
    };
    let instructions = instructions.split(",")
        .map(|ch| ch.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    part1(computer.clone(), &instructions);
}