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
        crate::parse::Emit::Mod(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftGUI).unwrap();
            } else {
                keyboard.push(Keyb::RightGUI).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        crate::parse::Emit::Alt(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftAlt).unwrap();
            } else {
                keyboard.push(Keyb::RightAlt).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        crate::parse::Emit::Shift(next) => {
            if first.is_left() {
                keyboard.push(Keyb::LeftShift).unwrap();
            } else {
                keyboard.push(Keyb::RightShift).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        crate::parse::Emit::Ctrl(next) => {
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

fn build_keyboard_report_layers(
    emit: crate::parse::Emit<Keyb>,
    first: &Key,
    last: &Key,
    identity: IdentityFn,
    keyboard: &mut Vec<Keyb, CHORD_SIZE>,
) {
    match emit {
        crate::parse::Emit::String(str) => {
            for chr in str.chars() {
                keyboard.push((chr as u8).into()).unwrap();
            }
        }
        crate::parse::Emit::Code(code) => {
            keyboard.push(code).unwrap();
        }
        crate::parse::Emit::Identity => {
            let evts = identity(*first, *last);
            keyboard.extend(evts);
        }
        _ => {}
    }
}
