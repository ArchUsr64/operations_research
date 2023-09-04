const N: usize = 4;
const C: [[usize; N]; N] = [[5, 9, 3, 6], [8, 7, 8, 2], [6, 10, 12, 7], [3, 10, 8, 6]];

/// Solves the problem and returns the minimum cost
/// TODO Memoization using a HashMap (with custom hash function would be cool)
/// TODO Return job allocations
fn solve() -> usize {
    let allocated = [None; N];
    /// Returns cost given an allocation
    fn recurse(allocated: [Option<usize>; N]) -> usize {
        let all_allocated = !allocated.iter().any(|allocation| allocation.is_none());
        if all_allocated {
            allocated
                .iter()
                .enumerate()
                .map(|(job, allocated_machine)| C[job][allocated_machine.unwrap()])
                .sum()
        } else {
            let (allocated_jobs, allocated_machines): (Vec<_>, Vec<_>) = allocated
                .iter()
                .enumerate()
                .filter_map(|(job, machine)| machine.map(|b| (job, b)))
                .unzip();
            // Can potentially use binary search here
            let unallocated_jobs = (0..N).filter(|job| !allocated_jobs.contains(job));
            let unallocated_machines: Vec<_> = (0..N)
                .filter(|machine| !allocated_machines.contains(machine))
                .collect();
            let mut min_cost = usize::MAX;
            for job in unallocated_jobs {
                for machine in unallocated_machines.iter() {
                    let mut new_allocation = allocated;
                    new_allocation[job] = Some(*machine);
                    min_cost = min_cost.min(recurse(new_allocation));
                }
            }
            return min_cost;
        }
    }
    recurse(allocated)
}

/// The assignment problem goes as follows:
/// Given N jobs and N machines where each machine can only perform a single task,
/// find the allocation of jobs that minimizes the cost given an NxN matrix with
/// Cost C_ij corresponding to the cost taken by Machine j to do Job i
fn main() {
    println!("Minimum cost: {}", solve());
}
