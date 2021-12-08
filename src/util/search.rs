pub mod bfs;
pub mod dijkstra;

pub type Distance = usize;

#[derive(Debug)]
pub struct SearchResult<S> {
    pub seen_states: Vec<SeenState<S>>,
    reached_goal: bool,
}

#[derive(Debug)]
pub struct SeenState<S> {
    pub state: S,
    pub distance: Distance,
    prev_index: Option<usize>,
}

impl<S> SearchResult<S> {
    pub fn goal_state(&self) -> Option<&SeenState<S>> {
        if self.reached_goal {
            Some(self.seen_states.last().unwrap())
        } else {
            None
        }
    }

    pub fn path_to_goal(&self) -> Option<Vec<&S>> {
        self.goal_state().map(|s| self.path_to(s))
    }

    pub fn path_to<'a>(&'a self, mut state: &'a SeenState<S>) -> Vec<&'a S> {
        let mut result = vec![&state.state];
        while let Some(i) = state.prev_index {
            state = &self.seen_states[i];
            result.push(&state.state);
        }
        result.reverse();
        result
    }
}
