#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum IfState {
    /// The condition was met, executing the block.
    ReadingIf,
    /// The condition was not met, skipping the block.
    PassingIf,
    /// The condition was met, executing the block.
    ReadingElse,
    /// The condition was not met, skipping the block.
    PassingElse,
    /// A parent condition was not met, skipping the block.
    PassingChild,
}

impl IfState {
    pub fn reading(&self) -> bool {
        match &self {
            IfState::ReadingIf => true,
            IfState::PassingIf => false,
            IfState::ReadingElse => true,
            IfState::PassingElse => false,
            IfState::PassingChild => false,
        }
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct IfStates(Vec<IfState>);
impl IfStates {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn reading(&self) -> bool {
        self.0.iter().all(|f| f.reading())
    }

    pub fn push(&mut self, s: IfState) {
        self.0.push(s)
    }

    pub fn pop(&mut self) -> Option<IfState> {
        self.0.pop()
    }

    pub fn flip(&mut self) {
        if self.0.iter().take(self.0.len() - 1).all(|f| f.reading()) {
            if let Some(new) = match self.pop() {
                Some(IfState::PassingChild) => Some(IfState::PassingChild),
                Some(IfState::PassingIf) => Some(IfState::ReadingElse),
                Some(IfState::ReadingIf) => Some(IfState::PassingElse),
                Some(IfState::PassingElse) => None,
                Some(IfState::ReadingElse) => None,
                None => None,
            } {
                self.push(new);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IfState, IfStates};

    #[test]
    fn basic_if() {
        let mut state = IfStates::new();

        assert!(state.reading());
        // if true
        state.push(IfState::ReadingIf);
        assert!(state.reading());
        // else
        state.flip();
        assert!(!state.reading());
    }

    #[test]
    fn basic_negative_if() {
        let mut state = IfStates::new();

        assert!(state.reading());
        // if false
        state.push(IfState::PassingIf);
        assert!(!state.reading());
        // else
        state.flip();
        assert!(state.reading());
    }

    #[test]
    fn nested_if() {
        let mut state = IfStates::new();

        assert!(state.reading());
        // if true
        state.push(IfState::ReadingIf);
        assert!(state.reading());
        {
            // if false
            state.push(IfState::PassingIf);
            assert!(!state.reading());
            // else
            state.flip();
            assert!(state.reading());
        }
        state.pop();
        // else
        state.flip();
        assert!(!state.reading());
    }

    #[test]
    fn nested_negative_if() {
        let mut state = IfStates::new();

        assert!(state.reading());
        // if false
        state.push(IfState::PassingIf);
        assert!(!state.reading());
        {
            // if true
            state.push(IfState::ReadingIf);
            assert!(!state.reading());
            // else
            state.flip();
            assert!(!state.reading());
        }
        state.pop();
        // else
        state.flip();
        assert!(state.reading());
    }
}
