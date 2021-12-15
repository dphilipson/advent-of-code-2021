use super::{Distance, SearchResult, SeenState};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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
    I: IntoIterator<Item = (S, Distance)>,
    FGoal: Fn(&S) -> bool,
{
    let (seen_states, reached_goal) = {
        let mut tracked_states = Vec::<TrackedState<S>>::new();
        let mut tracked_state_indices = HashMap::<Rc<S>, usize>::new();
        let mut pending_states = BinaryHeap::<PendingState>::new();
        let mut seen_states = Vec::<TempSeenState<S>>::new();
        let initial_state = Rc::new(initial_state);
        tracked_states.push(TrackedState {
            state: initial_state.clone(),
            distance: 0,
            prev_index: None,
            seen_index: None,
        });
        tracked_state_indices.insert(initial_state, 0);
        pending_states.push(PendingState {
            distance: 0,
            index: 0,
        });
        let mut reached_goal = false;
        while let Some(PendingState { distance, index }) = pending_states.pop() {
            let TrackedState {
                state,
                distance: tracked_distance,
                prev_index,
                ..
            } = tracked_states[index].clone();
            if distance > tracked_distance {
                continue;
            }
            seen_states.push(TempSeenState {
                state: state.clone(),
                distance,
                prev_index: prev_index.map(|i| tracked_states[i].seen_index.unwrap()),
            });
            tracked_states[index].seen_index = Some(seen_states.len() - 1);
            if is_goal(&state) {
                reached_goal = true;
                break;
            }
            for (next_state, added_distance) in get_next_states(&state) {
                let next_distance = distance + added_distance;
                let next_state = Rc::new(next_state);
                if let Some(&known_index) = tracked_state_indices.get(&next_state) {
                    let known_state = &mut tracked_states[known_index];
                    if next_distance < known_state.distance {
                        pending_states.push(PendingState {
                            distance: next_distance,
                            index: known_index,
                        });
                        known_state.distance = next_distance;
                        known_state.prev_index = Some(index);
                    }
                } else {
                    tracked_states.push(TrackedState {
                        state: next_state.clone(),
                        distance: next_distance,
                        prev_index: Some(index),
                        seen_index: None,
                    });
                    let next_index = tracked_states.len() - 1;
                    tracked_state_indices.insert(next_state, next_index);
                    pending_states.push(PendingState {
                        distance: next_distance,
                        index: next_index,
                    });
                }
            }
        }
        (seen_states, reached_goal)
    };
    let seen_states = seen_states
        .into_iter()
        .map(
            |TempSeenState {
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
struct TrackedState<S> {
    state: Rc<S>,
    distance: Distance,
    prev_index: Option<usize>,
    seen_index: Option<usize>,
}

impl<S> Clone for TrackedState<S> {
    fn clone(&self) -> Self {
        TrackedState {
            state: self.state.clone(),
            distance: self.distance,
            prev_index: self.prev_index,
            seen_index: self.seen_index,
        }
    }
}

#[derive(Debug)]
struct TempSeenState<S> {
    state: Rc<S>,
    distance: Distance,
    prev_index: Option<usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct PendingState {
    index: usize,
    distance: Distance,
}

impl Ord for PendingState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl PartialOrd for PendingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        graph: &[Vec<Edge>],
        start: usize,
        goal: usize,
        expected: Option<(Vec<usize>, Distance)>,
    ) {
        let results = search(
            start,
            |&node| graph[node].iter().map(|&Edge { node, cost }| (node, cost)),
            |&node| node == goal,
        );
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
    fn test_dijkstra() {
        //                  7
        //          +-----------------+
        //          |                 |
        //          v   10       2    |  2
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        | 5      |      |
        //          |        |        | 6    | 9
        //          +------> 2 -------+      |
        //           1       |               |
        //                   +---------------+
        //
        let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 1 }, Edge { node: 1, cost: 10 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![
                Edge { node: 1, cost: 5 },
                Edge { node: 3, cost: 6 },
                Edge { node: 4, cost: 9 },
            ],
            // Node 3
            vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
            // Node 4
            vec![],
        ];
        check_path(&graph, 1, 3, Some((vec![1, 3], 2)));
        check_path(&graph, 0, 1, Some((vec![0, 2, 1], 6)));
        check_path(&graph, 3, 0, Some((vec![3, 0], 7)));
        check_path(&graph, 0, 4, Some((vec![0, 2, 3, 4], 9)));
        check_path(&graph, 4, 0, None);
        check_path(&graph, 0, 100, None);
    }
}
