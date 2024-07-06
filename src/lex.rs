#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Key {
    L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16, L17, LAny,
    R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, R16, R17, RAny,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Down(Key),
    Up(Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pressed(pub Key);

pub mod qwerty {
    use crate::alias;

    alias!(Q, L1);
    alias!(W, L2);
    alias!(E, L3);
    alias!(R, L4);
    alias!(T, L5);
    alias!(A, L6);
    alias!(S, L7);
    alias!(D, L8);
    alias!(F, L9);
    alias!(G, L10);
    alias!(Z, L11);
    alias!(X, L12);
    alias!(C, L13);
    alias!(V, L14);
    alias!(B, L15);

    alias!(Y, R5);
    alias!(U, R4);
    alias!(I, R3);
    alias!(O, R2);
    alias!(P, R1);
    alias!(H, R10);
    alias!(J, R9);
    alias!(K, R8);
    alias!(L, R7);
    alias!(SEMI, R6);
    alias!(N, R15);
    alias!(M, R14);
    alias!(COMMA, R13);
    alias!(PERIOD, R12);
    alias!(QUESTION, R11);
}

pub mod colemak {
    // TODO: implement colemak
}

pub mod dvorak {
    // TODO: implement colemak
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
    use super::Event::*;
    use super::Key::*;
    use super::*;

    #[test]
    fn single_key() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Up(L1)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(L1), presses[0]);
    }

    #[test]
    fn single_key_with_surplus() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Up(L1)).unwrap();
        stack.push(Down(L2)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(L1), presses[0]);
        assert_eq!(Some(&Down(L2)), stack.first());
    }

    #[test]
    fn two_single_key_strokes() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Up(L1)).unwrap();
        stack.push(Down(L2)).unwrap();
        stack.push(Up(L2)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 2);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(L1), presses[0]);
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(L2), presses[0]);
    }

    #[test]
    fn two_key_chord() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Down(L2)).unwrap();
        stack.push(Up(L2)).unwrap();
        stack.push(Up(L1)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(L1), presses[0]);
        assert_eq!(Pressed(L2), presses[1]);
    }
    #[test]
    fn two_key_chord_surplus_then_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Down(L2)).unwrap();
        stack.push(Up(L2)).unwrap();
        stack.push(Up(L1)).unwrap();
        stack.push(Down(L3)).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(L1), presses[0]);
        assert_eq!(Pressed(L2), presses[1]);
        assert_eq!(Some(&Down(L3)), stack.first());

        stack.push(Up(L3)).unwrap();
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(L3), presses[0]);
    }
}
