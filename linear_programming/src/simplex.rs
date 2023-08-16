//! Finds solution to a linear programming problem using Simplex Method
mod problem_builder;
use problem_builder::*;
mod generic_simplex;
use rational::Rational;

fn main() {
    // For objective fn `Z = ax1 + bx2 + cx3`
    let mut problem = Problem::new(
        &[
            Rational::from_integer(3),
            Rational::from_integer(2),
            Rational::from_integer(5),
        ],
        ProblemKind::Maximization,
    );
    problem.add_constraint(Constraint::new(
        &[
            Rational::from_integer(1),
            Rational::from_integer(2),
            Rational::from_integer(1),
        ],
        Rational::from_integer(430),
        ConstraintKind::LessThan,
    ));
    problem.add_constraint(Constraint::new(
        &[
            Rational::from_integer(3),
            Rational::from_integer(0),
            Rational::from_integer(2),
        ],
        Rational::from_integer(460),
        ConstraintKind::LessThan,
    ));
    problem.add_constraint(Constraint::new(
        &[
            Rational::from_integer(1),
            Rational::from_integer(4),
            Rational::from_integer(0),
        ],
        Rational::from_integer(420),
        ConstraintKind::LessThan,
    ));
    generic_simplex::solve(
        problem,
        Rational::from_integer(0),
        Rational::from_integer(1),
        Rational::from_integer(10000),
    );
}
