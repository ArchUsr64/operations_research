//! Finds solution to a linear programming problem using Simplex Method
use rational::Rational;

const INF: Rational = Rational::new(1000_000, 1);

#[derive(PartialEq)]
enum ProblemKind {
    Minimization,
    Maximization,
}

const PROBLEM: ProblemKind = ProblemKind::Maximization;

fn main() {
    let objective = [
        Rational::from_integer(4),
        Rational::from_integer(3),
        Rational::from_integer(6),
    ];
    let constraint_1 = [
        Rational::from_integer(2),
        Rational::from_integer(3),
        Rational::from_integer(2),
        Rational::from_integer(440),
    ];
    let constraint_2 = [
        Rational::from_integer(4),
        Rational::from_integer(0),
        Rational::from_integer(3),
        Rational::from_integer(470),
    ];
    let constraint_3 = [
        Rational::from_integer(2),
        Rational::from_integer(5),
        Rational::from_integer(0),
        Rational::from_integer(430),
    ];
    let mut solution = [constraint_1[3], constraint_2[3], constraint_3[3]];
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
            constraint_1[0],
            constraint_1[1],
            constraint_1[2],
            Rational::from_integer(1),
            Rational::from_integer(0),
            Rational::from_integer(0),
        ],
        [
            constraint_2[0],
            constraint_2[1],
            constraint_2[2],
            Rational::from_integer(0),
            Rational::from_integer(1),
            Rational::from_integer(0),
        ],
        [
            constraint_3[0],
            constraint_3[1],
            constraint_3[2],
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
    loop {
        println!("\nMatrix: {matrix:?}");
        println!("Solution: {solution:?}");
        println!("Basis: {basis:?}");
        println!("Zj: {zj:?}");
        println!("CJZJ: {cj_zj:?}");
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
                .max_by_key(|(_, num)| num.clone())
                .unwrap()
        } else {
            cj_zj
                .iter()
                .enumerate()
                .min_by_key(|(_, num)| num.clone())
                .unwrap()
        };
        println!("Entering index: {entering_index} Entering value: {entering_value:?}");
        for (i, sol) in solution.iter().enumerate() {
            ratio[i] = (*sol / matrix[i][entering_index]).unwrap_or(INF);
        }
        let (leaving_index, _) = ratio
            .iter()
            .enumerate()
            .filter(|(_, num)| **num > Rational::from_integer(0))
            .min_by_key(|(_, num)| num.clone())
            .unwrap();
        let pivot_element = matrix[leaving_index][entering_index];
        println!("Pivot: {pivot_element:?}");
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
}
