use hw4::*;
use std::fs::read_to_string;

fn main() {
    for i in 1..7 {
        let filename = format!("2sat{i}.txt");
        let clauses = Clauses::from_str(&read_to_string(&filename).unwrap());
        if clauses.is_satisfiable(2) {
            println!("Problem {i} is satisfiable.")
        } else {
            println!("Problem {i} is not satisfiable.")
        }
    }
}
