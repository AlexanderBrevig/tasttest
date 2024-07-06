use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::chord;
use tastlib::lex::{chord, Event, Key, STACK_SIZE};
use tastlib::parse::parse_with;

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

// chord!(R_CTRL_SHIFT, [Both(H, J), RAny], Ctrl(&Shift(&Identity)));
chord!(
    R_CTRL_SHIFT,
    3,
    [On(H), On(J), LAny],
    Ctrl(&Shift(&Identity))
);

fn eval(stack: &mut Vec<Event, STACK_SIZE>) {
    let chrd = chord(stack);

    let emit = parse_with(&chrd, [R_CTRL_SHIFT]);
    println!("stack {:?}", stack);
    println!("chord {:?}", chrd);
    println!("emit  {:?} for {:?}", emit, chrd.last());
}

fn main() {
    println!("{:?}", R_CTRL_SHIFT);
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
