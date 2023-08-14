use super::{ConstraintKind, Problem, ProblemKind};
use rational::Rational;
use termion::color;

const INF: Rational = Rational::new(1000_000, 1);

const PROBLEM: ProblemKind = ProblemKind::Maximization;

pub fn solve(problem: Problem<i64>) {
    // For objective fn `Z = ax1 + bx2 + cx3`
    let objective: Vec<_> = problem
        .objective_function_coefficients()
        .iter()
        .map(|int| Rational::from_integer(*int))
        .collect();
    // Constraints are created according to the format `ax1 + bx2 + cx3 <= k`
    let constraints: Vec<_> = problem
        .constraints()
        .iter()
        .filter(|constraint| match constraint.kind() {
            ConstraintKind::LessThan => true,
            _ => panic!("Simplex doesn't support non 'LessThan' type constraints"),
        })
        .map(|constraint| {
            (
                constraint
                    .coefficients
                    .iter()
                    .map(|int| Rational::from_integer(*int))
                    .collect::<Vec<_>>(),
                Rational::from_integer(constraint.constant),
            )
        })
        .collect();
    let mut solution = [constraints[0].1, constraints[1].1, constraints[2].1];
    let ci = [
        objective[0],
        objective[1],
        objective[2],
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
    ];
    let mut cb = [
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
    ];
    let mut matrix = [
        [
            constraints[0].0[0],
            constraints[0].0[1],
            constraints[0].0[2],
            Rational::from_integer(1),
            Rational::from_integer(0),
            Rational::from_integer(0),
        ],
        [
            constraints[1].0[0],
            constraints[1].0[1],
            constraints[1].0[2],
            Rational::from_integer(0),
            Rational::from_integer(1),
            Rational::from_integer(0),
        ],
        [
            constraints[2].0[0],
            constraints[2].0[1],
            constraints[2].0[2],
            Rational::from_integer(0),
            Rational::from_integer(0),
            Rational::from_integer(1),
        ],
    ];
    let mut zj = [
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
    ];
    let mut ratio = [
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(0),
    ];
    let mut cj_zj = ci;
    let mut basis: [usize; 3] = [3, 4, 5];
    let index_to_var = |index| match index {
        0 => "x1",
        1 => "x2",
        2 => "x3",
        3 => "s1",
        4 => "s2",
        5 => "s3",
        _ => "INVALID_VAR",
    };
    let mut runs = 1;
    loop {
        println!(
            "\nIternation no: {}{runs}{}",
            color::Red.fg_str(),
            color::Reset.fg_str()
        );
        runs += 1;
        println!(
            "{}Matrix:\t\t\t{matrix:?}{}",
            color::Magenta.fg_str(),
            color::Reset.fg_str()
        );
        println!(
            "{}Cb, Basis, Solution:\t{:#?}{}",
            color::Blue.fg_str(),
            basis
                .iter()
                .zip(solution.iter().zip(cb.iter()))
                .map(|(variable_index, (solution, cb))| format!(
                    "{cb:?} {}: {solution:?}",
                    index_to_var(*variable_index),
                ))
                .collect::<Vec<_>>(),
            color::Reset.fg_str(),
        );
        println!("Zj:\t\t\t{zj:?}");
        println!("CJ-ZJ:\t\t\t{cj_zj:?}");
        let mut solved = true;
        for value in cj_zj.iter() {
            if *value > Rational::from_integer(0) && PROBLEM == ProblemKind::Maximization {
                solved = false;
                break;
            }
            if *value < Rational::from_integer(0) && PROBLEM == ProblemKind::Minimization {
                solved = false;
                break;
            }
        }
        if solved {
            break;
        }
        let (entering_index, entering_value) = if PROBLEM == ProblemKind::Maximization {
            cj_zj
                .iter()
                .enumerate()
                .max_by_key(|(_, num)| **num)
                .unwrap()
        } else {
            cj_zj
                .iter()
                .enumerate()
                .min_by_key(|(_, num)| **num)
                .unwrap()
        };
        for (i, sol) in solution.iter().enumerate() {
            ratio[i] = (*sol / matrix[i][entering_index]).unwrap_or(INF);
        }
        println!("Ratio: {ratio:?}");
        let (leaving_index, _) = ratio
            .iter()
            .enumerate()
            .filter(|(_, num)| **num > Rational::from_integer(0))
            .min_by_key(|(_, num)| **num)
            .unwrap();
        let pivot_element = matrix[leaving_index][entering_index];
        println!(
            "{}Leaving: {}, Entering: {}, Pivot: {:?}{}",
            color::Yellow.fg_str(),
            index_to_var(basis[leaving_index]),
            index_to_var(entering_index),
            pivot_element,
            color::Reset.fg_str(),
        );
        basis[leaving_index] = entering_index;
        cb[leaving_index] = *entering_value;
        // Fill the new pivot row
        matrix[leaving_index]
            .iter_mut()
            .for_each(|i| *i = (*i / pivot_element).unwrap_or(INF));
        solution[leaving_index] = (solution[leaving_index] / pivot_element).unwrap_or(INF);
        let mut set_non_pivot_row = |row_index: usize| {
            let corresponding_pivot_element = matrix[row_index][entering_index];
            for i in 0..6 {
                matrix[row_index][i] =
                    matrix[row_index][i] - corresponding_pivot_element * matrix[leaving_index][i];
            }
            solution[row_index] =
                solution[row_index] - corresponding_pivot_element * solution[leaving_index];
        };
        (0..3).for_each(|i| {
            if i != leaving_index {
                set_non_pivot_row(i);
            }
        });
        // Setting Zj
        (0..6).for_each(|i| {
            zj[i] = (0..3)
                .map(|j| matrix[j][i] * cb[j])
                .reduce(|acc, e| acc + e)
                .unwrap();
        });
        for i in 0..6 {
            cj_zj[i] = ci[i] - zj[i];
        }
    }
    println!(
        "\n{}[Final solutions]{}",
        color::LightGreen.bg_str(),
        color::Reset.bg_str()
    );
    let mut z_value = Rational::from_integer(0);
    for (i, variable_index) in basis.iter().enumerate() {
        if (0..3).contains(variable_index) {
            println!("{} = {:?}", index_to_var(*variable_index), solution[i]);
            z_value = z_value + solution[i] * objective[*variable_index];
        }
    }
    println!(
        "Z_{}: {:?}",
        if PROBLEM == ProblemKind::Maximization {
            "Max"
        } else {
            "Min"
        },
        z_value
    );
}
