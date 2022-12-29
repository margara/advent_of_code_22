use std::cmp::{min, Ordering};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use crate::Data::{List, Num};

#[derive(Clone)]
enum Data {
    Num(u8),
    List(Vec<Data>),
}

impl Data {
    fn parse(s: &str) -> Self {

        fn add_to_stack(stack: &mut Vec<Data>, val: Data) {
            if let Some(List(l)) = stack.last_mut() {
                l.push(val);
            } else {
                panic!("Parse error");
            }
        }

        let mut stack = Vec::from([List(Vec::new())]);

        s.chars().into_iter().for_each(|next| {
            if next.is_numeric() {
                let val = next.to_string().parse::<u8>().expect("Parse error");
                let val = Num(val);
                add_to_stack(&mut stack, val)
            } else if next == 'A' {
                add_to_stack(&mut stack, Num(10));
            } else if next == '[' {
                stack.push(List(Vec::new()));
            } else if next == ']' {
                let closed = stack.pop().expect("Parse error");
                add_to_stack(&mut stack, closed);
            } else {
                panic!("Parse error: {}", next)
            }
        });

        if stack.len() != 1 {
            panic!("Parse error")
        }

        stack.pop().unwrap()
    }


}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(num) => {
                write!(f, "{}", num).unwrap();
            },
            List(l) => {
                write!(f, "[").unwrap();
                l.iter().for_each(|d| {
                    write!(f, "{}", d).unwrap();
                });
                write!(f, "]").unwrap();
            }
        }
        Ok(())
    }
}

impl PartialEq<Self> for Data {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Num(val) => {
                if let Num(other_val) = other {
                    val == other_val
                } else {
                    List(Vec::from([Num(val.clone())])).eq(other)
                }
            },
            List(l) => {
                match other {
                    List(other_l) => {
                        if other_l.len() != l.len() {
                            false
                        } else {
                            l.iter().zip(other_l.iter()).all(|(a, b)| a.eq(b))
                        }
                    },
                    Num(other_val) => {
                        self.eq(&List(Vec::from([Num(other_val.clone())])))
                    }
                }
            }
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Num(val) => {
                if let Num(other_val) = other {
                    val.partial_cmp(other_val)
                } else {
                    List(Vec::from([Num(val.clone())])).partial_cmp(other)
                }
            },
            List(l) => {
                match other {
                    List(other_l) => {
                        let min = min(l.len(), other_l.len());
                        for i in 0..min {
                            if l.len() < i {
                                return Some(Ordering::Less)
                            } else if other_l.len() < i {
                                return Some(Ordering::Greater)
                            } else {
                                if !l.get(i).unwrap().eq(other_l.get(i).unwrap()) {
                                    return l.get(i).unwrap().partial_cmp(other_l.get(i).unwrap())
                                }
                            }
                        }
                        l.len().partial_cmp(&other_l.len())
                    },
                    Num(other_val) => {
                        self.partial_cmp(&List(Vec::from([Num(other_val.clone())])))
                    }
                }
            }
        }
    }
}

impl Eq for Data { }

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input() -> Vec<Data> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| line.unwrap())
        .map(|line| line.replace("10", "A"))
        .map(|line| line.replace(",", ""))
        .filter(|line| !line.is_empty())
        .map(|line| Data::parse(&line))
        .collect()
}

fn main() {
    let mut input = parse_input();
    input.iter().for_each(|d| {
        println!("{d}");
    });

    let len = input.len();
    let res = (1..len/2+1).filter_map(|i| {
        let first_idx = (i-1) * 2;
        let second_idx = first_idx + 1;
        if input[first_idx] <= input[second_idx] {
            Some(i)
        } else {
            None
        }
    }).sum::<usize>();
    println!("Sum of indices of correct pairs: {}", res);

    // Part 2
    let d1 = Data::parse("[[2]]");
    let d2 = Data::parse("[[6]]");
    input.push(d1.clone());
    input.push(d2.clone());

    input.sort();
    let i1 = input.iter().position(|d| d == &d1).unwrap() + 1;
    let i2 = input.iter().position(|d| d == &d2).unwrap() + 1;
    let res = i1 * i2;
    println!("Decoder key: {}", res);
}
