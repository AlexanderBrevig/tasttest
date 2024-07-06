use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::lex::{chord, Event, Key, Pressed, PRESS_SIZE, STACK_SIZE};
use tastlib::parse::parse_with;

#[rustfmt::skip]

/// This mod is mandatory when setting up a custom firmware
pub mod config {
    use tastlib::{alias, chord};

    alias!(TAB, L16);
    alias!(BCK, L17);
    alias!(SPC, R17);
    alias!(RET, R16);

    // Homerow mods left
    alias!(L_G, L6); //GUI/WIN/COMMAND
    alias!(L_A, L7); //ALT/OPTION
    alias!(L_S, L8);
    alias!(L_C, L9);
    // Homerow mods right
    alias!(R_G, R6);
    alias!(R_A, R7);
    alias!(R_S, R8);
    alias!(R_C, R9);
    /* Chords */
    // Homerow mods right
    chord!( R_GUI,          2, [On(R_G), LAny],                     Mod(&Identity));
    chord!( R_ALT,          2, [On(R_A), LAny],                     Alt(&Identity));
    chord!( R_SHIFT,        2, [On(R_S), LAny],                     Shift(&Identity));
    chord!( R_CTRL,         2, [On(R_C), LAny],                     Ctrl(&Identity));

    // Homerow mods left
    chord!( L_GUI,          2, [On(L_G), RAny],                     Mod(&Identity));
    chord!( L_ALT,          2, [On(L_A), RAny],                     Alt(&Identity));
    chord!( L_SHIFT,        2, [On(L_S), RAny],                     Shift(&Identity));
    chord!( L_CTRL,         2, [On(L_C), RAny],                     Ctrl(&Identity));


    chord!( R_CTRL_SHIFT,   2, [Both(R_C, R_S), LAny],              Ctrl(&Shift(&Identity)));
    chord!( L_ALLMOD,       4, [On(L_C), On(L_A), On(L_S), RAny],   Ctrl(&Alt(&Shift(&Identity))));
}

#[rustfmt::skip]
fn from_char_to_event(value: char) -> Event {
    use tastlib::lex::Event::Down as D;
    use tastlib::lex::Event::Up as U;
    match value {
        // DOWN
        'Q' => D(Key::L1), 'W' => D(Key::L2), 'E' => D(Key::L3), 'R' => D(Key::L4), 'T' => D(Key::L5),
        'A' => D(Key::L6), 'S' => D(Key::L7), 'D' => D(Key::L8), 'F' => D(Key::L9), 'G' => D(Key::L10),
        'Z' => D(Key::L11), 'X' => D(Key::L12), 'C' => D(Key::L13), 'V' => D(Key::L14), 'B' => D(Key::L15),
        'Y' => D(Key::R1), 'U' => D(Key::R2), 'I' => D(Key::R3), 'O' => D(Key::R4), 'P' => D(Key::R5),
        'H' => D(Key::R6), 'J' => D(Key::R7), 'K' => D(Key::R8), 'L' => D(Key::R9), ':' => D(Key::R10),
        'N' => D(Key::R11), 'M' => D(Key::R12), '<' => D(Key::R13), '>' => D(Key::R14), '?' => D(Key::R15),
        // UP
        'q' => U(Key::L1), 'w' => U(Key::L2), 'e' => U(Key::L3), 'r' => U(Key::L4), 't' => U(Key::L5),
        'a' => U(Key::L6), 's' => U(Key::L7), 'd' => U(Key::L8), 'f' => U(Key::L9), 'g' => U(Key::L10),
        'z' => U(Key::L11), 'x' => U(Key::L12), 'c' => U(Key::L13), 'v' => U(Key::L14), 'b' => U(Key::L15),
        'y' => U(Key::R1), 'u' => U(Key::R2), 'i' => U(Key::R3), 'o' => U(Key::R4), 'p' => U(Key::R5),
        'h' => U(Key::R6), 'j' => U(Key::R7), 'k' => U(Key::R8), 'l' => U(Key::R9), ';' => U(Key::R10),
        'n' => U(Key::R11), 'm' => U(Key::R12), ',' => U(Key::R13), '.' => U(Key::R14), '/' => U(Key::R15),
        _ => todo!("Key {} not implemented yet", value),
    }
}

fn eval(stack: &mut Vec<Event, STACK_SIZE>) {
    let chrd = chord(stack);

    let emit = parse_with(&chrd, [config::L_ALLMOD, config::R_CTRL_SHIFT]);
    println!("stack {:?}", stack);
    println!("chord {:?}", chrd);
    println!("emit  {:?} for {:?}", emit, chrd.last());

    do_emit(emit, chrd);
}

fn do_emit(emit: tastlib::parse::Emit<u8>, chrd: Vec<tastlib::lex::Pressed, PRESS_SIZE>) {
    let last = chrd.last();
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
            if let Some(Pressed(key)) = last {
                println!("{:?}", key)
            } else {
                println!("NONE");
            }
        }
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
