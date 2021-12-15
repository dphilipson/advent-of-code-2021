use super::{Distance, SearchResult, SeenState};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

pub fn search<S, FNext, I, FGoal>(
    initial_state: S,
    get_next_states: FNext,
    is_goal: FGoal,
) -> SearchResult<S>
where
    S: Eq + Hash,
    FNext: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
    FGoal: Fn(&S) -> bool,
{
    let (seen_states, reached_goal) = {
        let mut seen_states = Vec::<TempState<S>>::new();
        let mut seen_set = HashSet::<Rc<S>>::new();
        let mut pending = VecDeque::<TempState<S>>::new();
        pending.push_back(TempState {
            state: Rc::new(initial_state),
            distance: 0,
            prev_index: None,
        });
        let mut reached_goal = false;
        while let Some(state) = pending.pop_front() {
            if seen_set.contains(&state.state) {
                continue;
            }
            seen_states.push(state.clone());
            if is_goal(&state.state) {
                reached_goal = true;
                break;
            }
            seen_set.insert(state.state.clone());
            let TempState {
                state, distance, ..
            } = state;
            let index = seen_states.len() - 1;
            for next_state in get_next_states(&state) {
                pending.push_back(TempState {
                    state: Rc::new(next_state),
                    distance: distance + 1,
                    prev_index: Some(index),
                })
            }
        }
        (seen_states, reached_goal)
    };
    let seen_states = seen_states
        .into_iter()
        .map(
            |TempState {
                 state,
                 distance,
                 prev_index,
             }| SeenState {
                state: Rc::try_unwrap(state).map_err(|_| ()).unwrap(),
                distance,
                prev_index,
            },
        )
        .collect();
    SearchResult {
        seen_states,
        reached_goal,
    }
}

#[derive(Debug)]
struct TempState<S> {
    state: Rc<S>,
    distance: Distance,
    prev_index: Option<usize>,
}

impl<S> Clone for TempState<S> {
    fn clone(&self) -> Self {
        TempState {
            state: self.state.clone(),
            distance: self.distance,
            prev_index: self.prev_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Edge {
        node: usize,
        cost: usize,
    }

    fn check_path(
        graph: &[Vec<usize>],
        start: usize,
        goal: usize,
        expected: Option<(Vec<usize>, Distance)>,
    ) {
        let results = search(start, |&node| graph[node].clone(), |&node| node == goal);
        let path = results
            .path_to_goal()
            .map(|path| path.into_iter().map(|&node| node).collect::<Vec<_>>());
        let distance = results.goal_state().map(|s| s.distance);
        match expected {
            Some((expected_path, expected_distance)) => {
                assert_eq!(path.unwrap(), expected_path);
                assert_eq!(distance.unwrap(), expected_distance);
            }
            None => {
                assert!(path.is_none());
                assert!(distance.is_none());
            }
        }
    }

    #[test]
    fn test_bfs() {
        //          +-----------------+
        //          |                 |
        //          v                 |
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        |        |      |
        //          |        |        |      |
        //          +------> 2 -------+      |
        //                   |               |
        //                   +---------------+
        let graph = vec![
            // Node 0
            vec![1, 2],
            // Node 1
            vec![3],
            // Node 2
            vec![1, 3, 4],
            // Node 3
            vec![0, 4],
            // Node 4
            vec![],
        ];
        check_path(&graph, 1, 3, Some((vec![1, 3], 1)));
        check_path(&graph, 0, 1, Some((vec![0, 1], 1)));
        check_path(&graph, 3, 0, Some((vec![3, 0], 1)));
        check_path(&graph, 0, 4, Some((vec![0, 2, 4], 2)));
        check_path(&graph, 4, 0, None);
        check_path(&graph, 0, 100, None);
    }
}
