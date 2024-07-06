#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Key {
    L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16, L17, LAny,
    R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, R16, R17, RAny,
    //ANY, OTHER
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Down(Key),
    Up(Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pressed(pub Key);

pub mod qwerty {
    use super::{Key, Pressed};

    pub const Q: Pressed = Pressed(Key::L1);
    pub const D: Pressed = Pressed(Key::L8);
    pub const H: Pressed = Pressed(Key::R6);
    pub const J: Pressed = Pressed(Key::R7);
}

pub mod colemak {
    use super::{Key, Pressed};

    pub const H: Pressed = Pressed(Key::R6);
    pub const N: Pressed = Pressed(Key::R7);
}

use heapless::Vec;

pub const STACK_SIZE: usize = 128;
pub const PRESS_SIZE: usize = 64;

pub fn chord(stack: &mut Vec<Event, STACK_SIZE>) -> Vec<Pressed, PRESS_SIZE> {
    let mut pressed: Vec<Pressed, PRESS_SIZE> = Vec::new();
    if !stack.is_empty() {
        let remaining = rec_chord(stack, &mut pressed);
        let start_len = stack.len();
        for _ in 0..(start_len - remaining) {
            stack.remove(0);
        }
    }
    pressed
}

fn rec_chord(stack: &[Event], pressed: &mut Vec<Pressed, PRESS_SIZE>) -> usize {
    assert!(!stack.is_empty(), "Stack cannot be empty in rec_chord");
    let root_key = if !pressed.is_empty() {
        Some(pressed[0])
    } else {
        None
    };
    if let Some(Pressed(root_key)) = root_key {
        if let Event::Up(key) = &stack[0] {
            if root_key == *key {
                return stack.len() - 1; // ignore current token
            }
        }
    }
    if let Event::Down(start_key) = &stack[0] {
        for entry in stack {
            if let Event::Up(key) = entry {
                if key == start_key {
                    if pressed.push(Pressed(*start_key)).is_err() {
                        panic!("Should have enough capacity to push pressed");
                    }
                    break;
                }
            }
        }
    }
    if stack.len() >= 2 {
        return rec_chord(&stack[1..], pressed);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_key() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Event::Down(Key::L1)).unwrap();
        stack.push(Event::Up(Key::L1)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Key::L1), presses[0]);
    }
    #[test]
    fn single_key_with_surplus() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Event::Down(Key::L1)).unwrap();
        stack.push(Event::Up(Key::L1)).unwrap();
        stack.push(Event::Down(Key::L2)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Key::L1), presses[0]);
        assert_eq!(Some(&Event::Down(Key::L2)), stack.first());
    }
    #[test]
    fn two_single_key_strokes() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Event::Down(Key::L1)).unwrap();
        stack.push(Event::Up(Key::L1)).unwrap();
        stack.push(Event::Down(Key::L2)).unwrap();
        stack.push(Event::Up(Key::L2)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 2);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Key::L1), presses[0]);
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Key::L2), presses[0]);
    }
    #[test]
    fn two_key_chord() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Event::Down(Key::L1)).unwrap();
        stack.push(Event::Down(Key::L2)).unwrap();
        stack.push(Event::Up(Key::L2)).unwrap();
        stack.push(Event::Up(Key::L1)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(Key::L1), presses[0]);
        assert_eq!(Pressed(Key::L2), presses[1]);
    }
    #[test]
    fn two_key_chord_surplus_then_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Event::Down(Key::L1)).unwrap();
        stack.push(Event::Down(Key::L2)).unwrap();
        stack.push(Event::Up(Key::L2)).unwrap();
        stack.push(Event::Up(Key::L1)).unwrap();
        stack.push(Event::Down(Key::L3)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(Key::L1), presses[0]);
        assert_eq!(Pressed(Key::L2), presses[1]);
        assert_eq!(Some(&Event::Down(Key::L3)), stack.first());

        stack.push(Event::Up(Key::L3)).unwrap();
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Key::L3), presses[0]);
    }
}
