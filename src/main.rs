use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::lex::{chord, Event, Key, Pressed, CHORD_SIZE, STACK_SIZE};
use tastlib::parse::parse_with;
use tastlib::report::build_keyboard_report;
use usbd_human_interface_device::page::Keyboard as Keyb;

/// This mod is mandatory when setting up a custom firmware
mod config;
use config::*;

#[rustfmt::skip]
fn from_char_to_event(value: char) -> Event {
    use tastlib::lex::Event::Down as D;
    use tastlib::lex::Event::Up as U;
    match value {
        // DOWN
        'Q' => D(Key::L1), 'W' => D(Key::L2), 'E' => D(Key::L3), 'R' => D(Key::L4), 'T' => D(Key::L5),
        'A' => D(Key::L6), 'S' => D(Key::L7), 'D' => D(Key::L8), 'F' => D(Key::L9), 'G' => D(Key::L10),
        'Z' => D(Key::L11), 'X' => D(Key::L12), 'C' => D(Key::L13), 'V' => D(Key::L14), 'B' => D(Key::L15),
        'Y' => D(Key::R5), 'U' => D(Key::R4), 'I' => D(Key::R3), 'O' => D(Key::R2), 'P' => D(Key::R1),
        'H' => D(Key::R10), 'J' => D(Key::R9), 'K' => D(Key::R8), 'L' => D(Key::R7), ':' => D(Key::R6),
        'N' => D(Key::R15), 'M' => D(Key::R14), '<' => D(Key::R13), '>' => D(Key::R12), '?' => D(Key::R11),
        // UP
        'q' => U(Key::L1), 'w' => U(Key::L2), 'e' => U(Key::L3), 'r' => U(Key::L4), 't' => U(Key::L5),
        'a' => U(Key::L6), 's' => U(Key::L7), 'd' => U(Key::L8), 'f' => U(Key::L9), 'g' => U(Key::L10),
        'z' => U(Key::L11), 'x' => U(Key::L12), 'c' => U(Key::L13), 'v' => U(Key::L14), 'b' => U(Key::L15),
        'y' => U(Key::R5), 'u' => U(Key::R4), 'i' => U(Key::R3), 'o' => U(Key::R2), 'p' => U(Key::R1),
        'h' => U(Key::R10), 'j' => U(Key::R9), 'k' => U(Key::R8), 'l' => U(Key::R7), ';' => U(Key::R6),
        'n' => U(Key::R15), 'm' => U(Key::R14), ',' => U(Key::R13), '.' => U(Key::R12), '/' => U(Key::R11),
        _ => todo!("Key {} not implemented yet", value),
    }
}

fn eval(stack: &mut Vec<Event, STACK_SIZE>) -> Vec<Keyb, CHORD_SIZE> {
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();

    let chrd = chord(stack);

    if chrd.is_empty() {
        return keyboard;
    }

    #[rustfmt::skip]
    let emit = parse_with(
        &chrd,
        [ R_GUI, R_ALT, R_SHIFT, R_CTRL, R_GUI_ALT, R_GUI_SHIFT, R_GUI_CTRL, R_ALT_SHIFT, R_CTRL_ALT, R_CTRL_SHIFT,
          L_GUI, L_ALT, L_SHIFT, L_CTRL, L_ALLMOD, L_GUI_ALT, L_GUI_SHIFT, L_GUI_CTRL, L_ALT_SHIFT, L_CTRL_ALT, L_CTRL_SHIFT,],
    );
    println!("chord: {:?}", chrd);

    let Pressed(first) = chrd.first().unwrap();
    let Pressed(last) = chrd.last().unwrap();

    build_keyboard_report(emit, first, last, identity, &mut keyboard);
    // TODO: do whatever you want with the report
    println!("keyboard {:?}", keyboard);
    keyboard
}

fn identity(first: Key, last: Key) -> Vec<Keyb, CHORD_SIZE> {
    match Pressed(first) {
        TAB if first != last => tab_layer(last),
        BCK if first != last => bck_layer(last),
        SPC if first != last => spc_layer(last),
        RET if first != last => ret_layer(last),
        _ => base_layer(last),
    }
}

/*

    This is a nasty playground for playing with Tastlib
    UPPERCASE = key down
    lowercase = key up

    LEFT_ARROW   tab
    RIGHT_ARROW  backspace
    UP_ARROW     return/enter
    DOWN_ARROW   space
*/
fn main() {
    let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
    let mut charstck: std::vec::Vec<char> = vec![];
    let render = |stack: &Vec<Event, STACK_SIZE>, charstack: &std::vec::Vec<char>| {
        std::process::Command::new("clear").status().unwrap();
        println!("{:?}", &stack);
        println!("{:?}", &charstack);
    };

    let mut l16toggle = false;
    let mut l17toggle = false;
    let mut r16toggle = false;
    let mut r17toggle = false;

    for key in Keyboard::new() {
        match key {
            Keys::Char(chr) => {
                if stack.push(from_char_to_event(chr)).is_err() {
                    panic!("Should have enough capacity to push on stack");
                }
                charstck.push(chr);
                render(&stack, &charstck);
            }
            Keys::Delete => {
                stack.pop();
                charstck.pop();
                render(&stack, &charstck);
            }

            Keys::Left => {
                if l16toggle {
                    stack.push(Event::Up(Key::L16)).unwrap();
                    charstck.push('<');
                } else {
                    stack.push(Event::Down(Key::L16)).unwrap();
                    charstck.push('>');
                }
                l16toggle = !l16toggle;
                render(&stack, &charstck);
            }
            Keys::Right => {
                if l17toggle {
                    stack.push(Event::Up(Key::L17)).unwrap();
                    charstck.push('[');
                } else {
                    stack.push(Event::Down(Key::L17)).unwrap();
                    charstck.push(']');
                }
                l17toggle = !l17toggle;
                render(&stack, &charstck);
            }
            Keys::Up => {
                if r16toggle {
                    stack.push(Event::Up(Key::R16)).unwrap();
                    charstck.push('(');
                } else {
                    stack.push(Event::Down(Key::R16)).unwrap();
                    charstck.push(')');
                }
                r16toggle = !r16toggle;
                render(&stack, &charstck);
            }
            Keys::Down => {
                if r17toggle {
                    stack.push(Event::Up(Key::R17)).unwrap();
                    charstck.push('{');
                } else {
                    stack.push(Event::Down(Key::R17)).unwrap();
                    charstck.push('}');
                }
                r17toggle = !r17toggle;
                render(&stack, &charstck);
            }
            Keys::Escape => {
                break;
            }
            Keys::Enter => {
                charstck.clear();
                eval(&mut stack);
            }
            _ => {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Event::*;
    use super::Key::*;
    use super::*;

    #[test]
    fn test_empty() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        let keyboard = eval(&mut stack);
        assert!(keyboard.is_empty());
    }

    #[test]
    fn test_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L1)).unwrap();
        stack.push(Up(L1)).unwrap();

        let keyboard = eval(&mut stack);
        assert_eq!(Keyb::Q, keyboard[0]);
    }

    #[test]
    fn test_copy() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(R9)).unwrap();
        stack.push(Down(L13)).unwrap();
        stack.push(Up(L13)).unwrap();
        stack.push(Up(R9)).unwrap();

        let keyboard = eval(&mut stack);
        assert_eq!(Keyb::RightControl, keyboard[0]);
        assert_eq!(Keyb::C, keyboard[1]);
    }

    #[test]
    fn test_layer_pipe() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(R17)).unwrap();
        stack.push(Down(L10)).unwrap();
        stack.push(Up(L10)).unwrap();
        stack.push(Up(R17)).unwrap();

        let keyboard = eval(&mut stack);
        assert_eq!(Keyb::LeftShift, keyboard[0]);
        assert_eq!(Keyb::Backslash, keyboard[1]);
    }

    #[test]
    fn test_tab_only() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(L16)).unwrap();
        stack.push(Up(L16)).unwrap();

        let keyboard = eval(&mut stack);
        assert_eq!(Keyb::Tab, keyboard[0]);
    }
}
