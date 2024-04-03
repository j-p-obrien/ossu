type Var = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clause {
    allow: [bool; 2],
    vars: [Var; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clauses {
    clauses: Vec<Clause>,
    n_var: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Satisfied<'a>(Vec<Vec<&'a Clause>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TwoSATSolver<'a> {
    clauses: &'a Clauses,
    satisfied: Satisfied<'a>,
    unsatisfied: Vec<&'a Clause>,
    assignment: Vec<bool>,
    rng: RNG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RNG(usize);

impl RNG {
    fn new(seed: usize) -> Self {
        Self(seed)
    }

    #[inline]
    fn generate(&mut self) -> usize {
        // Xorshift* algorithm
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    #[inline]
    fn random_indicator(&mut self) -> usize {
        self.generate() % 2
    }

    #[inline]
    fn random_bool(&mut self) -> bool {
        self.random_indicator() == 1
    }
}

impl Clause {
    pub fn from_str(data: &str) -> Self {
        let split = data.split_once(" ").unwrap();
        let vars = (
            split.0.parse::<i32>().unwrap(),
            split.1.parse::<i32>().unwrap(),
        );
        Self {
            allow: [vars.0.is_positive(), vars.1.is_positive()],
            vars: [vars.0.abs() as usize, vars.1.abs() as usize],
        }
    }

    fn other_var(&self, var: &Var) -> Var {
        let idx = (self.vars[0] == *var) as usize;
        self.vars[idx]
    }
}

impl Clauses {
    pub fn from_str(data: &str) -> Self {
        let mut lines = data.lines();
        let n_var = lines.next().unwrap().parse::<usize>().unwrap();
        let clauses = lines.map(Clause::from_str).collect();
        Self { clauses, n_var }
    }

    /// Use Papadimitriou's algorithm to determine whether or not this is satisfiable.
    pub fn is_satisfiable(&self, n_iter: usize) -> bool {
        let mut solver = TwoSATSolver::new(self, 42);
        // log_2(1000) \approx 10
        for _ in 0..n_iter {
            if solver.try_solve() {
                return true;
            }
        }
        false
    }
}

impl<'a> Satisfied<'a> {
    fn new(clauses: &Clauses) -> Self {
        Satisfied(vec![vec![]; clauses.n_var + 1])
    }

    fn push_clause(&mut self, clause: &'a Clause) {
        self.0[clause.vars[0]].push(clause);
        self.0[clause.vars[1]].push(clause);
    }

    fn remove_clause_at(&mut self, clause: &'a Clause, var: &Var) {
        let clauses = &mut self.0[*var];
        let remove_var = clauses
            .iter()
            .position(|&other_clause| *clause == *other_clause)
            .unwrap();
        clauses.swap_remove(remove_var);
    }
}

impl<'a> TwoSATSolver<'a> {
    fn new(clauses: &'a Clauses, seed: usize) -> Self {
        Self {
            clauses,
            satisfied: Satisfied::new(clauses),
            unsatisfied: vec![],
            assignment: vec![true; clauses.n_var + 1],
            rng: RNG::new(seed),
        }
    }

    fn randomize_assignment(&mut self) {
        self.assignment
            .iter_mut()
            .for_each(|var| *var = self.rng.random_bool())
    }

    fn random_unsatisfied_idx(&mut self) -> usize {
        self.rng.generate() % self.unsatisfied.len()
    }

    fn flip_random_var(&mut self) -> Var {
        let random_idx = self.random_unsatisfied_idx();
        let random_clause = self.unsatisfied[random_idx];
        let var = random_clause.vars[self.rng.random_indicator()];
        self.assignment[var] = !self.assignment[var];
        var
    }

    fn is_satisfied(&self, clause: &Clause) -> bool {
        (self.assignment[clause.vars[0]] == clause.allow[0])
            | (self.assignment[clause.vars[1]] == clause.allow[1])
    }

    fn partition(&mut self) {
        self.clauses.clauses.iter().for_each(|clause| {
            if self.is_satisfied(clause) {
                self.satisfied.push_clause(clause)
            } else {
                self.unsatisfied.push(clause)
            }
        })
    }

    fn random_repartition(&mut self) {
        let flip_var = self.flip_random_var();
        let n = self.satisfied.0[flip_var].len();
        let m = self.unsatisfied.len();
        let mut i = 0;
        for _ in 0..n {
            let clause = self.satisfied.0[flip_var][i];
            if self.is_satisfied(clause) {
                i += 1;
            } else {
                self.satisfied.0[flip_var].swap_remove(i);
                let other_var = clause.other_var(&flip_var);
                self.satisfied.remove_clause_at(clause, &other_var);
                self.unsatisfied.push(clause);
            }
        }
        for i in (0..m).rev() {
            if i >= self.unsatisfied.len() {
                break;
            }
            let clause = self.unsatisfied[i];
            if self.is_satisfied(clause) {
                self.unsatisfied.swap_remove(i);
                self.satisfied.push_clause(clause);
            }
        }
    }

    fn try_solve(&mut self) -> bool {
        self.randomize_assignment();
        self.partition();
        if self.unsatisfied.len() == 0 {
            return true;
        }
        for i in 0..(2 * self.clauses.n_var) {
            // * self.clauses.n_var / 100) {
            if i % 10_000 == 0 {
                dbg!(self.unsatisfied.len());
            }
            self.random_repartition();
            if self.unsatisfied.len() == 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::{Clauses, RNG};

    const SAT: &str = "2\n1 2\n1 -2\n-1 2\n";

    const UNSAT: &str = "2\n1 2\n1 -2\n-1 2\n-1 -2\n";

    #[test]
    fn test1() {
        let sat = Clauses::from_str(SAT);
        assert!(sat.is_satisfiable(100))
    }

    #[test]
    fn test2() {
        let unsat = Clauses::from_str(UNSAT);
        assert!(!unsat.is_satisfiable(10))
    }

    #[test]
    fn test_rng() {
        let mut rng = RNG::new(42);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        println!("{}", rng.generate() % 2);
        assert!(true)
    }
}
