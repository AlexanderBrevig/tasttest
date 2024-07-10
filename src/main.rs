use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::{
    lex::{Event, Key, KeyId, STACK_SIZE},
    report::eval,
};

/// This is where you define your layout
mod config;

fn main() {
    print_usage();
    let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
    let mut charstck: std::vec::Vec<char> = vec![];
    let debug = |stack: &Vec<Event, STACK_SIZE>, charstack: &std::vec::Vec<char>| {
        std::process::Command::new("clear").status().unwrap();
        print_usage();
        println!("\t{:?}", &stack);
        println!("\t{:?}", &charstack);
    };

    let mut tab_toggle = false;
    let mut bck_toggle = false;
    let mut spc_toggle = false;
    let mut ret_toggle = false;

    for key in Keyboard::new() {
        match key {
            Keys::Char(chr) => {
                if stack.push(simulate_tastlib_input(chr)).is_err() {
                    println!("You're trying too hard ;) Resetting stacks.");
                    println!("`tastlib` is meant for embedded devices, so I've capped the input event buffer to 128.");
                    stack.clear();
                    charstck.clear();
                }
                charstck.push(chr);
                debug(&stack, &charstck);
            }
            Keys::Delete => {
                stack.pop();
                charstck.pop();
                debug(&stack, &charstck);
            }

            Keys::Home => {
                tab_sim(&mut tab_toggle, &mut stack, &mut charstck);
                debug(&stack, &charstck);
            }
            Keys::End => {
                bck_sim(&mut bck_toggle, &mut stack, &mut charstck);
                debug(&stack, &charstck);
            }
            Keys::Space => {
                ret_sim(&mut ret_toggle, &mut stack, &mut charstck);
                debug(&stack, &charstck);
            }
            Keys::Enter => {
                spc_sim(&mut spc_toggle, &mut stack, &mut charstck);
                debug(&stack, &charstck);
            }
            Keys::Escape => {
                break;
            }
            _ => {}
        }
        let keyboard = eval(&mut stack, &config::RULES);
        if !keyboard.is_empty() {
            charstck.clear();
            println!("\tKeyboard: {:?}", keyboard);
        }
    }
}

fn sim(
    key: Key,
    toggler: &mut bool,
    stack: &mut Vec<Event, 128>,
    charstck: &mut std::vec::Vec<char>,
    chr: char,
) {
    let evt = if *toggler {
        charstck.push(chr);
        Event::Up(key)
    } else {
        charstck.push(chr);
        Event::Down(key)
    };
    *toggler = !*toggler;
    stack.push(evt).unwrap();
}

fn spc_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>, charstck: &mut std::vec::Vec<char>) {
    sim(Key::Right(KeyId::K16), toggler, stack, charstck, '_');
}

fn ret_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>, charstck: &mut std::vec::Vec<char>) {
    sim(Key::Right(KeyId::K17), toggler, stack, charstck, '|');
}

fn bck_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>, charstck: &mut std::vec::Vec<char>) {
    sim(Key::Left(KeyId::K17), toggler, stack, charstck, '<');
}

fn tab_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>, charstck: &mut std::vec::Vec<char>) {
    sim(Key::Left(KeyId::K16), toggler, stack, charstck, '>');
}

#[rustfmt::skip]
fn simulate_tastlib_input(value: char) -> Event {
    use tastlib::lex::Event::Down as D;
    use tastlib::lex::Event::Up as U;
    use tastlib::lex::Key::Left as L;
    use tastlib::lex::Key::Right as R;
    use tastlib::lex::KeyId::*;
    match value {
        // DOWN
        'Q' => D(L(K1)), 'W' => D(L(K2)), 'E' => D(L(K3)), 'R' => D(L(K4)), 'T' => D(L(K5)),
        'A' => D(L(K6)), 'S' => D(L(K7)), 'D' => D(L(K8)), 'F' => D(L(K9)), 'G' => D(L(K10)),
        'Z' => D(L(K11)), 'X' => D(L(K12)), 'C' => D(L(K13)), 'V' => D(L(K14)), 'B' => D(L(K15)),
        'Y' => D(R(K5)), 'U' => D(R(K4)), 'I' => D(R(K3)), 'O' => D(R(K2)), 'P' => D(R(K1)),
        'H' => D(R(K10)), 'J' => D(R(K9)), 'K' => D(R(K8)), 'L' => D(R(K7)), ':' => D(R(K6)),
        'N' => D(R(K15)), 'M' => D(R(K14)), '<' => D(R(K13)), '>' => D(R(K12)), '?' => D(R(K11)),
        // UP
        'q' => U(L(K1)), 'w' => U(L(K2)), 'e' => U(L(K3)), 'r' => U(L(K4)), 't' => U(L(K5)),
        'a' => U(L(K6)), 's' => U(L(K7)), 'd' => U(L(K8)), 'f' => U(L(K9)), 'g' => U(L(K10)),
        'z' => U(L(K11)), 'x' => U(L(K12)), 'c' => U(L(K13)), 'v' => U(L(K14)), 'b' => U(L(K15)),
        'y' => U(R(K5)), 'u' => U(R(K4)), 'i' => U(R(K3)), 'o' => U(R(K2)), 'p' => U(R(K1)),
        'h' => U(R(K10)), 'j' => U(R(K9)), 'k' => U(R(K8)), 'l' => U(R(K7)), ';' => U(R(K6)),
        'n' => U(R(K15)), 'm' => U(R(K14)), ',' => U(R(K13)), '.' => U(R(K12)), '/' => U(R(K11)),
        _ => todo!("Key {} not implemented yet", value),
    }
}

fn print_usage() {
    println!(
        "\
        This is a nasty playground for playing with Tastlib \n\
        UPPERCASE = key down\n\
        lowercase = key up\n\
        \n\
        HOME   tab          (L16)\n\
        END    backspace    (L17)\n\
        ENTER  return/enter (R17)\n\
        SPACE  space        (R16)\n\
        \n\
        Try typing:\n\
            Oo                 => Keyboard: [O] -> emit an O \n\
            JCcj               => Keyboard: [RightCtrl, C] -> Copy command \n\
            SPACE I i SPACE    => Keyboard: [RightShift, Keyboard8] -> emit *\n\
        \n\
        ESC to quit
    "
    );
}

#[cfg(test)]
mod tests {
    use tastlib::lex::qwerty::*;
    use usbd_human_interface_device::page::Keyboard as Keyb;

    use super::Event::*;
    use super::*;
    use crate::config::*;

    #[test]
    fn test_empty() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        let keyboard = eval(&mut stack, &config::RULES);
        assert!(keyboard.is_empty());
    }

    #[test]
    fn test_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Q.into())).unwrap();
        stack.push(Up(Q.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::Q, keyboard[0]);
    }

    #[test]
    fn test_copy() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(J.into())).unwrap();
        stack.push(Down(C.into())).unwrap();
        stack.push(Up(C.into())).unwrap();
        stack.push(Up(J.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::RightControl, keyboard[0]);
        assert_eq!(Keyb::C, keyboard[1]);
    }

    #[test]
    fn test_layer_pipe() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(RET.into())).unwrap();
        stack.push(Down(G.into())).unwrap();
        stack.push(Up(G.into())).unwrap();
        stack.push(Up(RET.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::RightShift, keyboard[0]);
        assert_eq!(Keyb::Backslash, keyboard[1]);
    }

    #[test]
    fn test_tab_only() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(TAB.into())).unwrap();
        stack.push(Up(TAB.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::Tab, keyboard[0]);
    }
}
