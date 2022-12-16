use std::collections::BinaryHeap;

struct OrdByFirst<Prio, State>(Prio, State);

impl<Prio: PartialEq, State> PartialEq for OrdByFirst<Prio, State> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Prio: Eq, State> Eq for OrdByFirst<Prio, State> {}

impl<Prio: PartialOrd, State> PartialOrd for OrdByFirst<Prio, State> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Prio: Ord, State> Ord for OrdByFirst<Prio, State> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub trait Optimize {
    // A simulated (partial) solution to the problem
    type State: Clone;
    // The value to optimize
    type StateValue: Ord;
    // A cache of solutions
    type Solutions: Default;
    // The minimum, guaranteed StateValue of this partial solution
    fn guaranteed(&self, state: &Self::State) -> Self::StateValue;
    // The maximum, potential StateValue of this partial solution. Can be used to eliminate partial solutions, so getting this more precise, will speed up the algorithm.
    fn potential(&self, state: &Self::State) -> Self::StateValue;
    // The next states from an existing solved state.
    fn next_states(&self, state: &Self::State, next: &mut Vec<Self::State>);
    // Is similar states can be strictly better then we can eliminate partial solutions as well.
    // Return yes if this improves an old solution (or is new), no if it's worse.
    fn add_if_improvement(&self, _solutions: &mut Self::Solutions, _state: &Self::State) -> bool {
        true
    }
}

pub fn optimize<P: Optimize>(problem: P, initial: P::State) -> P::State {
    let mut queue = BinaryHeap::new();
    queue.push(OrdByFirst(problem.potential(&initial), initial.clone()));
    let mut best_solution = (problem.guaranteed(&initial), initial);
    let mut next = Vec::new();
    let mut solutions = P::Solutions::default();
    while let Some(OrdByFirst(_, state)) = queue.pop() {
        if problem.add_if_improvement(&mut solutions, &state) {
            let guaranteed = problem.guaranteed(&state);
            if guaranteed > best_solution.0 {
                best_solution = (guaranteed, state.clone());
            }
            problem.next_states(&state, &mut next);
            for next in next.drain(..) {
                let potential = problem.potential(&next);
                if potential > best_solution.0 {
                    queue.push(OrdByFirst(potential, next));
                }
            }
        }
    }
    best_solution.1
}
