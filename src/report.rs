use heapless::Vec;
use usbd_human_interface_device::page::Keyboard as Keyb;

use crate::{
    lex::{Key, CHORD_SIZE},
    parse::Emit,
};

type IdentityFn = fn(Key, Key) -> Vec<Keyb, CHORD_SIZE>;

pub fn build_keyboard_report(
    emit: Emit<Keyb>,
    first: &Key,
    last: &Key,
    identity: IdentityFn,
    keyboard: &mut Vec<Keyb, CHORD_SIZE>,
) {
    let emit = build_keyboard_report_modifiers(emit, first, keyboard);
    build_keyboard_report_layers(emit, first, last, identity, keyboard);
}

fn build_keyboard_report_modifiers(
    emit: Emit<Keyb>,
    first: &Key,
    keyboard: &mut Vec<Keyb, CHORD_SIZE>,
) -> Emit<Keyb> {
    match emit {
        Emit::Mod(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftGUI).unwrap();
            } else {
                keyboard.push(Keyb::RightGUI).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Alt(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftAlt).unwrap();
            } else {
                keyboard.push(Keyb::RightAlt).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Shift(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftShift).unwrap();
            } else {
                keyboard.push(Keyb::RightShift).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Ctrl(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftControl).unwrap();
            } else {
                keyboard.push(Keyb::RightControl).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        _ => emit,
    }
}

fn report_from_chr(chr: char) -> Keyb {
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
    if chr.is_uppercase() {
        keyboard.push(Keyb::LeftShift).unwrap();
    }
    match chr.to_lowercase().last().unwrap() {
        'h' => Keyb::H,
        'e' => Keyb::E,
        'l' => Keyb::L,
        'o' => Keyb::O,
        // TODO: add more keys for Emit::String
        _ => Keyb::NoEventIndicated,
    }
}

fn build_keyboard_report_layers(
    emit: Emit<Keyb>,
    first: &Key,
    last: &Key,
    identity: IdentityFn,
    keyboard: &mut Vec<Keyb, CHORD_SIZE>,
) {
    match emit {
        Emit::String(str) => {
            for chr in str.chars() {
                keyboard.push(report_from_chr(chr)).unwrap();
                keyboard.push(Keyb::Out).unwrap();
            }
        }
        Emit::Code(code) => {
            keyboard.push(code).unwrap();
        }
        Emit::Identity => {
            let evts = identity(*first, *last);
            keyboard.extend(evts);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Emit::*;
    use crate::{
        lex::{Key, CHORD_SIZE},
        parse::Emit,
        report::{
            build_keyboard_report, build_keyboard_report_layers, build_keyboard_report_modifiers,
        },
    };
    use heapless::Vec;
    use usbd_human_interface_device::page::Keyboard as Keyb;

    #[allow(unused_variables)]
    fn identity(first: Key, last: Key) -> Vec<Keyb, CHORD_SIZE> {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        if first == Key::R16 {
            keyboard.push(Keyb::Keyboard1).unwrap();
        } else {
            keyboard.push(Keyb::Q).unwrap();
        }
        keyboard
    }

    #[test]
    fn test_report_identity() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = crate::parse::Emit::Identity;
        let first = &crate::lex::Key::L1;
        let last = first;
        build_keyboard_report_layers(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::Q, keyboard[0]);
    }

    #[test]
    fn test_report_identity_layer() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = crate::parse::Emit::Identity;
        let first = &crate::lex::Key::R16;
        let last = &crate::lex::Key::L1;
        build_keyboard_report_layers(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::Keyboard1, keyboard[0]);
    }

    #[test]
    fn test_report_string() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::String("Hello");
        let first = &crate::lex::Key::L1;
        let last = first;
        build_keyboard_report_layers(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::H, keyboard[0]);
        assert_eq!(Keyb::Out, keyboard[1]);
        assert_eq!(Keyb::E, keyboard[2]);
        assert_eq!(Keyb::Out, keyboard[3]);
        assert_eq!(Keyb::L, keyboard[4]);
        assert_eq!(Keyb::Out, keyboard[5]);
        assert_eq!(Keyb::L, keyboard[6]);
        assert_eq!(Keyb::Out, keyboard[5]);
        assert_eq!(Keyb::O, keyboard[8]);
    }

    #[test]
    fn test_report_code() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::Code(Keyb::A);
        let first = &crate::lex::Key::L1; // originally Q, but translated to A
        let last = first;
        build_keyboard_report_layers(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::A, keyboard[0]);
    }

    #[test]
    fn test_report_modifiers_left() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::Shift(&Emit::Identity);
        let first = &crate::lex::Key::L8;
        let emit = build_keyboard_report_modifiers(emit, first, &mut keyboard);
        assert_eq!(Keyb::LeftShift, keyboard[0]);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn test_report_modifiers_right() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::Ctrl(&Emit::Identity);
        let first = &crate::lex::Key::R8;
        let emit = build_keyboard_report_modifiers(emit, first, &mut keyboard);
        assert_eq!(Keyb::RightControl, keyboard[0]);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn test_build_keyboard_report_no_mods() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::Identity;
        let first = &crate::lex::Key::R16;
        let last = &crate::lex::Key::L1;
        build_keyboard_report(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::Keyboard1, keyboard[0]);
    }

    #[test]
    fn test_build_keyboard_report_shift() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Emit::Shift(&Emit::Identity);
        let first = &crate::lex::Key::R16;
        let last = &crate::lex::Key::L1;
        build_keyboard_report(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::RightShift, keyboard[0]);
        assert_eq!(Keyb::Keyboard1, keyboard[1]);
    }

    #[test]
    fn test_build_keyboard_report_god() {
        let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
        let emit = Mod(&Ctrl(&Alt(&Shift(&Emit::Identity))));
        let first = &Key::R6; // right gui
        let last = &Key::L1;
        build_keyboard_report(emit, first, last, identity, &mut keyboard);
        assert_eq!(Keyb::RightGUI, keyboard[0]);
        assert_eq!(Keyb::RightControl, keyboard[1]);
        assert_eq!(Keyb::RightAlt, keyboard[2]);
        assert_eq!(Keyb::RightShift, keyboard[3]);
        assert_eq!(Keyb::Q, keyboard[4]);
    }
}
