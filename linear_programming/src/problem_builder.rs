#![allow(unused)]
#[derive(PartialEq)]
pub enum ProblemKind {
    Minimization,
    Maximization,
}

#[derive(Clone, Copy)]
pub enum ConstraintKind {
    Equal,
    GreaterThan,
    LessThan,
}

pub struct Constraint<T: Clone> {
    pub coefficients: Vec<T>,
    pub constant: T,
    kind: ConstraintKind,
}
impl<T: Clone> Constraint<T> {
    pub fn new(coefficients: &[T], constant: T, kind: ConstraintKind) -> Self {
        Self {
            coefficients: coefficients.to_vec(),
            constant,
            kind,
        }
    }
    pub fn kind(&self) -> ConstraintKind {
        self.kind
    }
}

pub struct Problem<T: Clone> {
    objective_function: Vec<T>,
    constraints: Vec<Constraint<T>>,
    goal: ProblemKind,
}
impl<T: Clone> Problem<T> {
    pub fn new(objective_function_coefficients: &[T], goal: ProblemKind) -> Self {
        Self {
            objective_function: objective_function_coefficients.to_vec(),
            constraints: Vec::new(),
            goal,
        }
    }
    pub fn add_constraint(&mut self, constraint: Constraint<T>) {
        self.constraints.push(constraint);
    }
    pub fn objective_function_coefficients(&self) -> &[T] {
        &self.objective_function
    }
    pub fn constraints(&self) -> &[Constraint<T>] {
        &self.constraints
    }
}
