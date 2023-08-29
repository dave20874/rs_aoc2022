
use std::{hash::Hash, marker::PhantomData, collections::HashMap};
use priority_queue::PriorityQueue;
use std::fmt::Debug;

trait Problem<S> {
    fn get_initial(&self) -> Box<S>;
    fn evaluate(&self, state:&S) -> (isize, isize);  // returns (score, priority)
    fn is_final(&self, state:&S) -> bool;
}

struct Solver<'a, P, S> 
where P: Problem<S>,
      S: Hash+Eq {
    problem: &'a P,                        // Problem produces init state and subsequent states, computes priority and score for states
    _marker: PhantomData<S>,
}

impl<'a, P, S>  Solver<'a, P, S> 
where P: Problem<S>, S: Hash+Eq {
    fn new(problem: &P) -> Solver<'a, P, S> {
        Solver { problem, _marker: PhantomData}
    }

    fn solve(&self) -> Option<S> {
        // Queue of states to check, highest priority first
        let to_search: PriorityQueue<S, isize> = PriorityQueue::new();     

        // Collection of states checked and their scores
        let best: HashMap<S, isize> = HashMap::new();

        let initial_state = self.problem.get_initial();
        let (score, priority) = self.problem.evaluate(&initial_state);
        to_search.push(*initial_state, priority);
        best.insert(*initial_state, score);

        while to_search.len() != 0 {
            let (state, priority) = to_search.pop().unwrap();
            if self.problem.is_final(&state) {
                return Some(state);
            }
            else {
                for s in self.problem.next_states(state) {
                    let (score, priority) = &self.problem.evaluate(s);
                    
                    if best.contains_key(s) {
                        if score <= best[s] {
                            // this is no better than another path we've seen.  Skip it.
                            continue;
                        }
                    }

                    // stash this state to advance further
                    to_search.push(s, priority);
                    best.insert(s, score);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, PartialEq, Eq)]
    struct TestState {
        // TODO
        position: (isize, isize)
    }

    struct TestProblem {
        start: (isize, isize),
        goal: (isize, isize),
    }

    impl TestProblem {
        fn new(x: isize, y: isize) -> TestProblem {
            TestProblem {start: (0, 0), goal: (x, y)}
        }
    }

    impl Problem<TestState> for TestProblem {

        fn get_initial(&self) -> Box<TestState> {
            Box::new(TestState {position: self.start })
        }

        fn evaluate(&self, state:TestState) -> (isize, isize) {
            let score = 0;
            let priority = 0;

            (score, priority)
        }
    }

    #[test]
    fn test1() {
        let problem = TestProblem::new(10, 10);
        let solver: Solver<TestProblem, TestState> = Solver::new(&problem);

        let soln: TestState = solver.solve().unwrap();

        assert_eq!(soln.path.len(), 20);
        assert_eq!(soln.path[19], (10, 10));
    }
}
