use std::fmt;
#[derive(Clone, Copy)]


pub struct TreeState {
    pub tree_state: char,
}

impl TreeState {
    pub fn new(c: char) -> Self {
        TreeState {
            tree_state: c.to_ascii_lowercase(),
        }
    }

    pub fn get(&self) -> char {
        self.tree_state
    }

}

impl fmt::Display for TreeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self.tree_state {
            'e' => "Empty",
            't' => "Tree",
            'b' => "Burning",
            'h' => "Hitted",
            other => {
                return write!(f, "Unknown({})", other);
            }
        };
        write!(f, "{}", word)
    }
}

impl fmt::Debug for TreeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TreeState('{}' => {})",
            self.tree_state, self
        )
    }
}
