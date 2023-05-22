use std::collections::BTreeMap;
use std::fs;
use clap::Parser;
use std::str::FromStr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// filepath
    #[arg(short, long, default_value="test")]
    path: String,

}

#[derive(Debug, Clone)]
enum Location{
    Mark(String),
    Line(usize)
}


#[derive(Debug)]
struct State{
    ip: Location,
    registers: Registers,
    ac: RegisterWidth,
    ram: BTreeMap::<String, RegisterWidth>,
    holded: bool,
    jmp: bool
}
impl State {
    fn execute(&mut self, lines: &Vec<Line>, marks: BTreeMap<&String, usize>) {
        let mut lin = match self.ip.clone(){
            Location::Mark(str) => marks[&str],
            Location::Line(int) => int,
        };
        lines[lin].instruction.execute(self);
        if self.jmp{
            println!("jumped{:?}", self.ip);
            self.jmp = false;
        } else{
            lin += 1;
            self.ip = Location::Line(lin)
        }
    }

    fn load(&mut self, str: &str) {
        self.loadi(self.ram[str]);
    }

    fn loadi(&mut self, int: RegisterWidth) {
        self.ac = int;

        self.update_registers();
    }

    fn store(&mut self, str: &str) {
        //self.ram[str] = self.ac;
        self.ram.insert(str.to_string(), self.ac);
    }

    fn add(&mut self, str: &str) {
        self.addi(self.ram[str]);
    }

    fn addi(&mut self, int: RegisterWidth) {
        self.ac += int;

        self.update_registers();
    }

    fn sub(&mut self, str: &str) {
        self.subi(self.ram[str]);
    }

    fn subi(&mut self, int: RegisterWidth) {
        self.ac -= int;

        self.update_registers();
    }

    fn mul(&mut self, str: &str) {
        self.muli(self.ram[str]);
    }

    fn muli(&mut self, int: RegisterWidth) {
        self.ac *= int;

        self.update_registers();
    }

    fn div(&mut self, str: &str) {
        self.divi(self.ram[str]);
    }

    fn divi(&mut self, int: RegisterWidth) {
        self.ac /= int;

        self.update_registers();
    }

    fn modulo(&mut self, str: &str) {
        self.moduloi(self.ram[str]);
    }

    fn moduloi(&mut self, int: RegisterWidth) {
        self.ac %= int;

        self.update_registers();
    }

    fn cmp(&mut self, str: &str) {
        self.cmpi(self.ram[str]);
    }

    fn cmpi(&mut self, int: RegisterWidth){
        self.registers.clear();
        if self.ac == int{
            self.registers.Z = true;
        } else if self.ac < int{
            self.registers.N = true;
        }
    }

    fn jmp(&mut self, addr: &str) {
        self.ip = Location::Mark(addr.to_string());
        self.jmp = true;
    }

    fn jmpz(&mut self, addr: &str) {
        if self.registers.Z{
            self.jmp(addr);
        }
    }

    fn jmpnz(&mut self, addr: &str) {
        if !self.registers.Z{
            self.jmp(addr);
        }
    }

    fn jmpn(&mut self, addr: &str) {
        if self.registers.N{
            self.jmp(addr);
        }
    }

    fn jmpnn(&mut self, addr: &str) {
        if !self.registers.N {
            self.jmp(addr);
        }
    }

    fn jmpp(&mut self, addr: &str) {
        if (!self.registers.N) && (!self.registers.Z){
            self.jmp(addr);
        }
    }

    fn jmpnp(&mut self, addr: &str) {
        if !((!self.registers.N) && (!self.registers.Z)){
            self.jmp(addr);
        }
    }
    
    fn hold(&mut self) {
        self.holded = true;
    }

    fn update_registers(&mut self) {
        self.registers.clear();
        if self.ac == 0{
            self.registers.Z = true;
        }
        else if self.ac < 0 {
            self.registers.N = true;
        }
        
    }

    

}

impl Default for State{
    fn default() -> Self {
        Self { ip: Location::Line(0), registers: Default::default(), ram: Default::default(), ac: 0, holded: false, jmp: false }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct Registers{
    Z: bool,
    N: bool,
    V: bool, //ignored

}
impl Registers {
    fn clear(&mut self) {
        self.Z = false;
        self.N = false;
        self.V = false;
    }
}

impl Default for Registers{
    fn default() -> Self {
        Self { Z: false, N: false, V: false }
    }
}

type RegisterWidth = i64;

#[derive(Debug)]
enum Instruction{
    LOAD(String),
    LOADI(RegisterWidth),
    STORE(String),

    ADD(String),
    ADDI(RegisterWidth),

    SUB(String),
    SUBI(RegisterWidth),

    MUL(String),
    MULI(RegisterWidth),

    DIV(String),
    DIVI(RegisterWidth),

    MOD(String),
    MODI(RegisterWidth),

    CMP(String),
    CMPI(RegisterWidth),

    JMP(String),
    JMPZ(String),
    JMPNZ(String),
    JMPN(String),
    JMPNN(String),
    JMPP(String),
    JMPNP(String),

    HOLD
}

#[derive(Debug)]
enum ParseErr{
    WrongInstruction(String)

}

impl FromStr for Instruction {
    type Err = ParseErr;
    fn from_str(
        s: &str,
    ) -> Result<Instruction, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let l = parts.next().unwrap();
        let r = parts.next();
        Ok(
            match l {
                "LOAD" => Instruction::LOAD(r.unwrap().into()),
                "LOADI" => Instruction::LOADI(r.unwrap().parse().unwrap()),
                "STORE" => Instruction::STORE(r.unwrap().parse().unwrap()),
                "ADD" => Instruction::ADD(r.unwrap().into()),
                "ADDI" => Instruction::ADDI(r.unwrap().parse().unwrap()),
                "SUB" => Instruction::SUB(r.unwrap().into()),
                "SUBI" => Instruction::SUBI(r.unwrap().parse().unwrap()),
                "MUL" => Instruction::MUL(r.unwrap().into()),
                "MULI" => Instruction::MULI(r.unwrap().parse().unwrap()),
                "DIV" => Instruction::DIV(r.unwrap().into()),
                "DIVI" => Instruction::DIVI(r.unwrap().parse().unwrap()),
                "MOD" => Instruction::MOD(r.unwrap().into()),
                "MODI" => Instruction::MODI(r.unwrap().parse().unwrap()),
                "CMP" => Instruction::CMP(r.unwrap().into()),
                "CMPI" => Instruction::CMPI(r.unwrap().parse().unwrap()),
                "JMP" => Instruction::JMP(r.unwrap().into()),
                "JMPZ" => Instruction::JMPZ(r.unwrap().into()),
                "JMPNZ" => Instruction::JMPNZ(r.unwrap().into()),
                "JMPN" => Instruction::JMPN(r.unwrap().into()),
                "JMPNN" => Instruction::JMPNN(r.unwrap().into()),
                "JMPP" => Instruction::JMPP(r.unwrap().into()),
                "JMPnp" => Instruction::JMPNP(r.unwrap().into()),
                "HOLD" => Instruction::HOLD,
                _ => {
                    return Err(ParseErr::WrongInstruction(s.to_string()))
                }
            },
        )
    }
}

impl Instruction{
    fn execute(&self, state: &mut State){
        match self{
            Instruction::LOAD(str) => state.load(str),
            Instruction::LOADI(int) => state.loadi(*int),
            Instruction::STORE(str) => state.store(str),
            Instruction::ADD(str) => state.add(str),
            Instruction::ADDI(int) => state.addi(*int),
            Instruction::SUB(str) => state.sub(str),
            Instruction::SUBI(int) => state.subi(*int),
            Instruction::MUL(str) => state.mul(str),
            Instruction::MULI(int) => state.muli(*int),
            Instruction::DIV(str) => state.div(str),
            Instruction::DIVI(int) => state.divi(*int),
            Instruction::MOD(str) => state.modulo(str),
            Instruction::MODI(int) => state.moduloi(*int),
            Instruction::CMP(str) => state.cmp(str),
            Instruction::CMPI(int) => state.cmpi(*int),
            Instruction::JMP(addr) => state.jmp(addr),
            Instruction::JMPZ(addr) => state.jmpz(addr),
            Instruction::JMPNZ(addr) => state.jmpnz(addr),
            Instruction::JMPN(addr) => state.jmpn(addr),
            Instruction::JMPNN(addr) => state.jmpnn(addr),
            Instruction::JMPP(addr) => state.jmpp(addr),
            Instruction::JMPNP(addr) => state.jmpnp(addr),
            Instruction::HOLD => {state.hold()},
        }
    }
}

#[derive(Debug)]
struct Line{
    mark: Option<String>,
    instruction: Instruction
}

fn main() {
    let args = Args::parse();
    
    let file_content = fs::read_to_string(args.path).expect("cannot read file");



    let filtered = file_content.lines().map(|line| {
        {
            match line.to_string().replace("\t", " ").trim().split_once(";"){
                None => line.to_string(),
                Some((a, _)) => a.to_string(),
            }
        }
    }).filter(|line|{!line.is_empty()});

    let lines: Vec<_> = filtered.map(|line| {
        let mut colon_seperated = line.rsplit(":");
        let instruction = colon_seperated.next().unwrap();
        let mark = match colon_seperated.next(){
            Some(x) => Some(x.to_string()),
            None => None
        };
        if let Some(_)  = colon_seperated.next(){
            panic!("to many : in line with countent {}", line)
        }
        //let instruction_v: &str = &instruction.to_owned();
        let instruction = Instruction::from_str(instruction)
            .expect(&format!("invalid operation: {}", instruction).to_owned());
        
        Line{ mark, instruction }
    
    }).collect();

    println!("parsed lines: {:?}", lines);

    let marks: BTreeMap<_,_> = lines.iter().enumerate().filter_map(|(i, line)|{
        if let Some(mark) = &line.mark{
            return Some((mark, i))
        }
        None
    }).collect();

    let mut state = State::default();

    while !state.holded{
        state.execute(&lines, marks.clone());
    }

    println!("marks:");
    println!("{:?}",marks);
    println!("---------------------------------------------");
    println!("state:");
    println!("{:?}", state);

}
