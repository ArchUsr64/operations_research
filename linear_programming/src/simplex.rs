//! Finds solution to a linear programming problem using Simplex Method
mod problem_builder;
use problem_builder::*;
mod generic_simplex;

fn main() {
    // For objective fn `Z = ax1 + bx2 + cx3`
    let mut problem = Problem::new(&[3, 2, 5], ProblemKind::Maximization);
    problem.add_constraint(Constraint::new(&[1, 2, 1], 430, ConstraintKind::LessThan));
    problem.add_constraint(Constraint::new(&[3, 0, 2], 460, ConstraintKind::LessThan));
    problem.add_constraint(Constraint::new(&[1, 4, 0], 420, ConstraintKind::LessThan));
    generic_simplex::solve(problem);
}
