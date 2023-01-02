use std::collections::VecDeque;
use pest::{Parser};
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day11/parser.pest"]
struct InputParser;


// **********************************************

type WorryLevel = u64;


#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(WorryLevel),
    // new = old + 42
    Mul(WorryLevel),
    // new = old * 42
    Square,     // new = old * old
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divisible_by: WorryLevel,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Copy, Clone)]
struct OperationTest(Operation, Test);

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<WorryLevel>,
    operation: Operation,
    test: Test,
    inspected: u64,
}


#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    /// Least Common Multiple of all the monkeys `test.divisible_by` values
    modulo: WorryLevel,
}


// **********************************************

impl Monkey {
    fn from_parse(monkey: Pair<Rule>) -> Option<Self> {
        if monkey.as_rule() != Rule::monkey {
            return None;
        }

        let mut pairs = monkey.into_inner();

        let id: usize =
            pairs.next().unwrap() // monkey_name
                .into_inner()
                .next().unwrap() // num
                .as_str()
                .parse()
                .unwrap();

        let items: VecDeque<_> =
            pairs.next().unwrap()
                .into_inner()
                .map(|p| {
                    p.as_str().parse().unwrap()
                })
                .collect();

        let operation = Operation::from_parse(
            pairs.next().unwrap()
        ).unwrap();

        let test = Test::from_parse(
            pairs.next().unwrap()
        ).unwrap();

        Some(Self {
            id,
            items,
            operation,
            test,
            inspected: 0,
        })
    }
}

impl Operation {
    fn from_parse(operation: Pair<Rule>) -> Option<Self> {
        if operation.as_rule() != Rule::operation {
            return None;
        }

        let mut inner = operation.into_inner();

        // first rule is always `old`
        inner.next();

        let op = match inner.next() {
            // `new = old`
            None => {
                Operation::Add(0)
            }

            Some(binop) => {
                // if `binop` is given, then there is also another rule,
                // the right-hand-side
                let rhs = inner.next().unwrap();

                match rhs.as_rule() {
                    Rule::old => {
                        match binop.as_str() {
                            // `old + old` == `old * 2`
                            "+" => Operation::Mul(2),
                            // `old * old` == `old ^ 2`
                            "*" => Operation::Square,

                            _ => panic!("Unexpected binop: {:?}", binop)
                        }
                    }
                    Rule::num => {
                        let rhs: WorryLevel = rhs.as_str().parse().unwrap();
                        match binop.as_str() {
                            "+" => Operation::Add(rhs),
                            "*" => Operation::Mul(rhs),
                            _ => panic!("Unexpected binop: {:?}", binop)
                        }
                    }

                    _ => {
                        panic!("Unexpected rhs rule: {:?}", rhs);
                    }
                }
            }
        };
        Some(op)
    }
}

impl Test {
    fn from_parse(test: Pair<Rule>) -> Option<Self> {
        if test.as_rule() != Rule::test {
            return None;
        }

        let mut inner = test.into_inner();

        let divisible_by =
            inner.next().unwrap()
                .as_str()
                .parse().unwrap();

        let if_true =
            inner.next().unwrap()
                .into_inner()
                .next().unwrap()
                .as_str()
                .parse().unwrap();

        let if_false =
            inner.next().unwrap()
                .into_inner()
                .next().unwrap()
                .as_str()
                .parse().unwrap();

        Some(Self {
            divisible_by,
            if_true,
            if_false,
        })
    }
}

impl Monkeys {
    fn parse(input: &str) -> Self {
        let monkeys: Vec<_> =
            InputParser::parse(Rule::file, input)
                .expect("Failed parse")
                .next().unwrap() // get and unwrap the top-level `file` rule, should never fail
                .into_inner()
                .flat_map(|pair| {
                    match pair.as_rule() {
                        Rule::monkey => {
                            Some(
                                Monkey::from_parse(pair).unwrap()
                            )
                        }
                        _ => {
                            // whitespace, ignore
                            None
                        }
                    }
                })
                .collect();

        for (i, monkey) in monkeys.iter().enumerate() {
            assert_eq!(i, monkey.id);
        }

        let m = monkeys.iter()
            .map(|m| m.test.divisible_by)
            .fold(1, |x, y| x * y);

        Self {
            monkeys,
            modulo: m,
        }
    }
}


// *************************************************************************************************

impl Operation {
    fn modify(self, x: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(y) => { x + y }
            Operation::Mul(y) => { x * y }
            Operation::Square => { x * x }
        }
    }
}

impl Test {
    fn throw_target(self, item: WorryLevel) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl Monkey {
    fn operation_test(&self) -> OperationTest {
        OperationTest(self.operation, self.test)
    }
}

impl OperationTest {
    /// Given an items worry level, return the new worry level and the monkey to throw it to
    fn exec(&self, item: WorryLevel, modulo: Option<WorryLevel>) -> (WorryLevel, usize) {
        let mut item = self.0.modify(item);
        if let Some(m) = modulo {
            item %= m;
        } else {
            item /= 3;
        }

        (item, self.1.throw_target(item))
    }
}

impl Monkeys {
    fn do_round(&mut self, modulo: Option<WorryLevel>) {
        for index in 0..self.monkeys.len() {
            let m = &self.monkeys[index];
            let op_test = m.operation_test();

            let mut interactions = 0;
            while let Some(item) = self.monkeys[index].items.pop_front() {
                let (item, t) = op_test.exec(item, modulo);
                self.monkeys[t].items.push_back(item);
                interactions += 1;
            }
            self.monkeys[index].inspected += interactions;
        }
    }

    fn get_monkey_business(mut self, divide: bool, rounds: usize) -> u64 {
        let modulo = if divide {
            None
        } else {
            Some(self.modulo)
        };

        for i in 1..=rounds {
            println!("\n\nRound {i}:");
            self.do_round(modulo);
            for monkey in &self.monkeys {
                println!("{:?}", monkey)
            }
        }

        self.monkeys.sort_by_key(|m| m.inspected);

        let x = self.monkeys.pop().unwrap().inspected;
        let y = self.monkeys.pop().unwrap().inspected;
        x * y
    }
}


fn part1() {
    let m = Monkeys::parse(include_str!("test.txt"));

    println!("monkey business: {}",
             m.get_monkey_business(true, 20)
    );
}


fn part2() {
    let m = Monkeys::parse(include_str!("input.txt"));

    println!("{:#?}", m);

    println!("monkey business: {}",
             m.get_monkey_business(false, 10_000)
    );
}


pub fn main() {
    part2()
}