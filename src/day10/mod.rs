use std::fmt::{Formatter};
use std::ops::{Index, IndexMut};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day10/parser.pest"]
struct InputParser;


#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

#[derive(Debug, Copy, Clone)]
struct ProgramState {
    cycle: usize,
    x: i64,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Execution {
    instructions: Vec<Instruction>,
    state: ProgramState,

    /// The index of the current instruction
    program_counter: usize,

    /// The cycle at which to start a new instruction
    end_cycle: usize,
}


// *************************************************************************************************

impl ProgramState {
    fn new() -> Self {
        Self {
            cycle: 1,
            x: 1,
        }
    }
}

impl Execution {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut ret = Self {
            instructions,
            state: ProgramState::new(),
            program_counter: 0,
            end_cycle: 0,
        };
        ret.set_end_cycle();
        ret
    }

    fn current_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.program_counter).copied()
    }

    /// Sets the `end_cycle` to the current `state.cycle` plus the current instructions cycle_count
    fn set_end_cycle(&mut self) {
        let cycle_count = self.current_instruction()
            .map(Instruction::cycle_count).unwrap_or(0);

        self.end_cycle = self.state.cycle + cycle_count;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 1;
        self.set_end_cycle();
    }
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
        }
    }
}

// *************************************************************************************************


impl Program {
    fn parse(input: &str) -> Self {
        let instructions: Vec<_> =
            InputParser::parse(Rule::program, input)
                .expect("Failed to parse input")
                .next().unwrap()
                .into_inner()
                .flat_map(|inst| {
                    if inst.as_rule() != Rule::inst {
                        None
                    } else {
                        let inst = inst.into_inner().next().unwrap();
                        let inst = match inst.as_rule() {
                            Rule::noop => Instruction::Noop,
                            Rule::addx => {
                                let operand = inst.into_inner().next().unwrap().as_str().parse().unwrap();
                                Instruction::Addx(operand)
                            }
                            _ => {
                                panic!("Unexpected instruction rule: {inst:?}")
                            }
                        };
                        Some(inst)
                    }
                })
                .collect();

        println!("{:?}", &instructions);

        Self::new(instructions)
    }
}


// *************************************************************************************************

impl Instruction {
    fn cycle_count(self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl ProgramState {
    /// Perform an instructions effect
    /// Note: does *not* modify the cycle count
    fn do_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Noop => {}
            Instruction::Addx(y) => {
                self.x += y;
            }
        }
    }
}

impl Program {
    fn execute(self) -> Execution {
        Execution::new(self.instructions)
    }
}

impl Iterator for Execution {
    type Item = (ProgramState, Option<Instruction>);

    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.current_instruction()?;

        if self.end_cycle == self.state.cycle {
            self.state.do_instruction(inst);
            self.increment_program_counter();
        }

        // copy the current state, to return
        let state = self.state;

        self.state.cycle += 1;

        Some((state, self.current_instruction()))
    }
}

impl Execution {
    fn trace(self) -> impl Iterator<Item=<Self as Iterator>::Item> {
        self.inspect(|(state, inst)| println!("{state:?}  \t{inst:?}"))
    }
}


// *************************************************************************************************


#[aoc(day10, part1)]
fn part1(input: &str) -> i64 {
    let p = Program::parse(input);
    p.execute()
        .trace()
        .filter(|(state, _)| {
            let c = state.cycle;
            c == 20 || (c > 20 && (c - 20) % 40 == 0)
        })
        .map(|(state, _)| {
            state.x * state.cycle as i64
        })
        .inspect(|signal_strength| println!("    -- signal strength = {signal_strength}"))
        .sum()
}


// *************************************************************************************************


impl Crt {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    const SPRITE_SIZE: isize = 3;
}

struct Crt {
    pixels: [[bool; Self::WIDTH]; Self::HEIGHT],
}

struct CrtCoord {
    row: usize,
    col: usize,
}

impl Index<CrtCoord> for Crt {
    type Output = bool;

    fn index(&self, index: CrtCoord) -> &Self::Output {
        &self.pixels[index.row][index.col]
    }
}

impl IndexMut<CrtCoord> for Crt {
    fn index_mut(&mut self, index: CrtCoord) -> &mut Self::Output {
        &mut self.pixels[index.row][index.col]
    }
}

impl ProgramState {
    fn current_pixel(&self) -> CrtCoord {
        let c = self.cycle - 1;
        let col = c % Crt::WIDTH;
        let row = c / Crt::WIDTH;
        CrtCoord { col, row }
    }
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: [[false; Self::WIDTH]; Self::HEIGHT]
        }
    }

    fn draw_pixel(&mut self, program_state: ProgramState) {
        let pixel = program_state.current_pixel();

        let sprite = program_state.x as isize - 1;
        let col = pixel.col as isize;
        if sprite <= col && col < sprite + Self::SPRITE_SIZE {
            self[pixel] = true;
        }
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for &pixel_is_set in row {
                write!(f, "{}", if pixel_is_set { '#' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Crt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for &pixel_is_set in row {
                write!(f, "{}", if pixel_is_set { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}


// *************************************************************************************************


#[aoc(day10, part2)]
fn part2(input: &str) -> Crt {
    let p = Program::parse(input);
    let mut screen = Crt::new();

    for (state, _) in p.execute().trace() {
        screen.draw_pixel(state)
    }

    screen
}


// *************************************************************************************************
