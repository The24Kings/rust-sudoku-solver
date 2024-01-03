pub struct State {
    pub states: Vec<u16>,
    pub active: u16,
}

impl State {
    pub fn active(&self) -> u16 {
        self.active
    }
}

impl IntoIterator for State {
    type Item = u16;
    type IntoIter = StateIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        StateIntoIterator {
            state: self,
            index: 0,
        }
    }
}

pub struct StateIntoIterator {
    state: State,
    index: usize,
}

impl Iterator for StateIntoIterator {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        let result = match self.index {
            0 => self.state.states[0],
            1 => self.state.states[1],
            2 => self.state.states[2],
            3 => self.state.states[3],
            4 => self.state.states[4],
            5 => self.state.states[5],
            6 => self.state.states[6],
            7 => self.state.states[7],
            8 => self.state.states[8],
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a State {
    type Item = u16;
    type IntoIter = StateIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StateIterator {
            state: self,
            index: 0,
        }
    }
}

pub struct StateIterator<'a> {
    state: &'a State,
    index: usize,
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        let result = match self.index {
             0 => self.state.states[0],
             1 => self.state.states[1],
             2 => self.state.states[2],
             3 => self.state.states[3],
             4 => self.state.states[4],
             5 => self.state.states[5],
             6 => self.state.states[6],
             7 => self.state.states[7],
             8 => self.state.states[8],
             _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}
