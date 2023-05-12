
use std::{hash::Hash, marker::PhantomData, collections::HashMap};
use priority_queue::PriorityQueue;
use std::fmt::Debug;

pub trait AStarState<K> {
    fn is_final(&self) -> bool;
    fn next_states(&self) -> Vec<Box<Self>>;
    fn cost(&self) -> isize;
    fn completion_estimate(&self) -> isize;
    fn get_key(&self) -> K;
    fn get_key_metric(&self) -> usize;
}

pub struct AStarSearch<T, K>     
where T: AStarState<K>+Hash+Eq+Debug {
    minimize: bool,
    in_progress: PriorityQueue<T, isize>,
    metrics: HashMap<K, usize>,
    verbose: bool,
    _marker: PhantomData<K>,
}

impl<T, K> AStarSearch<T, K>  
where T: AStarState<K>+Hash+Eq+Debug,
      K: Hash+Eq
{
    pub fn new(min: bool, verbose: bool) -> AStarSearch<T, K> {
        let minimize = min;  // TODO: support maximizing, too.

        let in_progress: PriorityQueue<T, isize> = PriorityQueue::new();
        let metrics: HashMap<K, usize> = HashMap::new();

        AStarSearch {minimize, in_progress: in_progress, metrics, verbose, _marker: PhantomData}
    }

    fn push(&mut self, s: T, priority: isize) {
        // Extract state-key, metric
        let k = s.get_key();
        let metric = s.get_key_metric();

        // If there's already a metric associated with this state-key
        if self.metrics.contains_key(&k) {
            // If this metric is better
            if metric > *self.metrics.get(k).unwrap() {
                // Update the metric for this state-key
                self.metrics.insert(k, metric);
            }
        }
        else {
            // First time seeing this key, store it's metric
            self.metrics.insert(k, metric);
        }

        if self.minimize {
            // Priority queue returns the largest (most postitive) priority
            // first.  When minimizing, negate the priority to reverse this.
            // println!("Pushing {:?}, {}", s, -priority);
            self.in_progress.push(s, -priority);
        }
        else {
            // When maximizing, 
            // println!("Pushing {:?}, {}", s, priority);
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

        // while in_progress is not empty
        while self.in_progress.len() > 0 {
            // remove highest priority solution in progress
            let (state, _priority) = self.in_progress.pop().unwrap();
            if self.verbose {
                println!("{} Popped {}. {:?}", self.in_progress.len(), _priority, state);
                // println!("Queue: {}", self.in_progress.len());
            }

            if state.is_final() {
                // We found the final state!
                retval = Some(state);
                break;
            }

            // Extract state-key and evaluate metric
            let k = state.get_key();
            let metric = state.get_key_metric();

            // Don't explore this state if one with the same key and better metric has been seen.
            if metric >= self.metrics[k] {
                // Explore
                // For each next state from this one, evaluate it and add it back
                // to the priority queue if it's worth exploring.
                for next in state.next_states() {
                    let cost = next.cost();
                    let heuristic = next.completion_estimate();
                    self.push(*next, cost + heuristic);
                }
            }
        }

        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash)]
    struct TestStateKey {
        position: isize,
    }

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

    impl AStarState<TestStateKey> for TestState {
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

        fn get_key(&self) -> TestStateKey {
            TestStateKey {position: self.position}
        }

        fn get_key_metric(&self) -> usize {
            self.position
        }
    }

    #[test]
    fn test_new_search() {
        let mut search: AStarSearch<TestState, TestStateKey> = AStarSearch::new(true, false);
        search.set_start(TestState { moves: 0, position: 3});
        let final_state  = search.search();
        assert_eq!(final_state, Some(TestState{moves: 7, position: 10}));
    }
}
