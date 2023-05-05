use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub struct Cpu {
    cmds: VecDeque<Cmd>,
    cycles: usize,
    is_done: bool,
    current_cycles: usize,
    current_cmd: Option<Cmd>,
    x: isize,
}

impl Cpu {
    pub fn new(cmds: Vec<Cmd>) -> Cpu {
        Cpu {
            cmds: cmds.into(),
            cycles: 0,
            is_done: false,
            current_cycles: 0,
            current_cmd: None,
            x: 1,
        }
    }

    // Returns false when there is no more work to do
    pub fn run_cycle(&mut self) {
        use Cmd::*;

        // Get the next command if none is processing
        if self.current_cmd.is_none() {
            self.current_cmd = self.cmds.pop_front();
        }

        // Exit if there is no more work
        if self.current_cmd.is_none() {
            self.is_done = true;
            return;
        }

        let cmd = self.current_cmd.clone().unwrap();

        self.cycles += 1;
        self.current_cycles += 1;

        // Perform command if enough cycles have passed
        let required_cycles = match cmd {
            Addx { .. } => 2,
            Noop => 1,
        };
        if self.current_cycles == required_cycles {
            match cmd {
                Addx { v } => self.x += v,
                Noop => {}
            }

            self.current_cmd = None;
            self.current_cycles = 0;

            if self.cmds.is_empty() {
                self.is_done = true;
            }
        }
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn x(&self) -> isize {
        self.x
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Cmd {
    Addx { v: isize },
    Noop,
}

impl FromStr for Cmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Cmd::*;

        let cmd_parts: Vec<_> = s.split_ascii_whitespace().collect();

        Ok(match cmd_parts[0] {
            "addx" => Addx {
                v: cmd_parts[1].parse().unwrap(),
            },
            "noop" => Noop {},
            _ => Err(format!("Unknown command: {}", cmd_parts[0]))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_cmd_from_str() {
        assert_eq!(get_test_input::<Cmd>()[0], Cmd::Addx { v: 15 });
    }
}
