
use std::hash::Hash;
use priority_queue::PriorityQueue;
use std::fmt::Debug;

pub trait AStarState {
    fn is_final(&self) -> bool;
    fn next_states(&self) -> Vec<Box<Self>>;
    fn cost(&self) -> isize;
    fn completion_estimate(&self) -> isize;
}

pub struct AStarSearch<T>     
where T: AStarState+Hash+Eq+Debug {
    minimize: bool,
    in_progress: PriorityQueue<T, isize>,
}

impl<T> AStarSearch<T>  
where T: AStarState+Hash+Eq+Debug {
    pub fn new() -> AStarSearch<T> {
        let minimize = true;  // TODO: support maximizing, too.
        let in_progress: PriorityQueue<T, isize> = PriorityQueue::new();

        AStarSearch {minimize, in_progress: in_progress}
    }

    fn push(&mut self, s: T, priority: isize) {
        if self.minimize {
            // Priority queue returns the largest (most postitive) priority
            // first.  When minimizing, negate the priority to reverse this.
            println!("Pushing {:?}, {}", s, -priority);
            self.in_progress.push(s, -priority);
        }
        else {
            // When maximizing, 
            println!("Pushing {:?}, {}", s, priority);
            self.in_progress.push(s, priority);
        }
    }

    pub fn set_start(&mut self, s: T) {
        let cost = s.cost();
        let heuristic = s.completion_estimate();
        let priority = cost + heuristic;
        self.push(s, priority);
    }

    pub fn search(&mut self) -> Option<T> {
        let mut retval = None;
        // We haven't found a final state yet.
        let mut found_end = false;

        // while in_progress is not empty
        while self.in_progress.len() > 0 && !found_end {
            // remove highest priority solution in progress
            let (state, _priority) = self.in_progress.pop().unwrap();
            println!("Popped {:?}, {}", state, _priority);

            if state.is_final() {
                // We found the final state!
                retval = Some(state);
                break;
            }

            // For each next state from this one, evaluate it and add it back
            // to the priority queue if it's worth exploring.
            for next in state.next_states() {
                let cost = next.cost();
                let heuristic = next.completion_estimate();
                self.push(*next, cost + heuristic);
            }
        }

        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct TestState {
        moves: usize,
        position: isize,
    }

    impl Drop for TestState {
        fn drop(&mut self) {
            println!("Dropping {:?}", &self);
        }
    }

    impl AStarState for TestState {
        fn is_final(&self) -> bool {
            self.position >= 10
        }

        fn next_states(&self) -> Vec<Box<TestState>>
        {
            println!("next_states from n:{}", self.position);

            let mut v = Vec::new();

            v.push(Box::new(TestState{moves: self.moves+1, position: self.position+1}));
            v.push(Box::new(TestState{moves: self.moves+1, position: self.position-1}));

            v
        }

        fn cost(&self) -> isize {
            self.moves as isize
        }

        fn completion_estimate(&self) -> isize {
            9 - self.position
        }
    }

    #[test]
    fn test_new_search() {
        let mut search: AStarSearch<TestState> = AStarSearch::new();
        search.set_start(TestState { moves: 0, position: 3});
        let final_state  = search.search();
        assert_eq!(final_state, Some(TestState{moves: 7, position: 10}));
    }
}
