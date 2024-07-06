use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::lex::{chord, Event, Key, Pressed, PRESS_SIZE, STACK_SIZE};
use tastlib::parse::parse_with;

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

fn eval(stack: &mut Vec<Event, STACK_SIZE>) {
    let chrd = chord(stack);

    let emit = parse_with(
        &chrd,
        [
            R_GUI,
            R_ALT,
            R_SHIFT,
            R_CTRL,
            R_GUI_ALT,
            R_GUI_SHIFT,
            R_GUI_CTRL,
            R_ALT_SHIFT,
            R_CTRL_ALT,
            R_CTRL_SHIFT,
            L_GUI,
            L_ALT,
            L_SHIFT,
            L_CTRL,
            L_ALLMOD,
            L_GUI_ALT,
            L_GUI_SHIFT,
            L_GUI_CTRL,
            L_ALT_SHIFT,
            L_CTRL_ALT,
            L_CTRL_SHIFT,
        ],
    );
    println!("stack {:?}", stack);
    println!("chord {:?}", chrd);
    println!("emit  {:?} for {:?}", emit, chrd.last());

    do_emit(emit, chrd);
}

fn do_emit(emit: tastlib::parse::Emit<u8>, chrd: Vec<tastlib::lex::Pressed, PRESS_SIZE>) {
    if chrd.is_empty() {
        return;
    }
    let Pressed(first) = chrd.first().unwrap();
    let Pressed(last) = chrd.last().unwrap();
    match emit {
        tastlib::parse::Emit::Mod(next) => {
            print!("M-");
            do_emit(*next, chrd);
            return;
        }
        tastlib::parse::Emit::Alt(next) => {
            print!("A-");
            do_emit(*next, chrd);
            return;
        }
        tastlib::parse::Emit::Shift(next) => {
            print!("S-");
            do_emit(*next, chrd);
            return;
        }
        tastlib::parse::Emit::Ctrl(next) => {
            print!("C-");
            do_emit(*next, chrd);
            return;
        }
        tastlib::parse::Emit::String(str) => {
            println!("{}", str);
        }
        tastlib::parse::Emit::Code(code) => {
            println!("{}", code);
        }
        tastlib::parse::Emit::Identity => {
            println!("{:?}", last)
        }
    }
    let mut chord_no_layer: Vec<Pressed, PRESS_SIZE> = Vec::new();
    chord_no_layer.extend_from_slice(&chrd[1..]).unwrap();
    match Pressed(*first) {
        TAB => println!("{:?}", tab_layer(*last)),
        BCK => println!("{:?}", bck_layer(*last)),
        SPC => println!("{:?}", spc_layer(*last)),
        RET => println!("{:?}", ret_layer(*last)),
        _ => println!("{:?}", base_layer(*last)),
    }
    println!("Done");
}

fn main() {
    let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
    let mut charstck: std::vec::Vec<char> = vec![];
    let render = |stack: &Vec<Event, STACK_SIZE>, charstack: &std::vec::Vec<char>| {
        std::process::Command::new("clear").status().unwrap();
        println!("{:?}", &stack);
        println!("{:?}", &charstack);
    };
    let mut l16toggle = false;
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
                    charstck.push('$');
                } else {
                    stack.push(Event::Down(Key::L16)).unwrap();
                    charstck.push('%');
                }
                l16toggle = !l16toggle;
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
