use crate::lex::{Pressed, PRESS_SIZE};
use heapless::Vec;

#[derive(Debug, Clone, Copy)]
pub enum ChordEvent {
    Any(Pressed, Pressed, Pressed, Pressed),
    Both(Pressed, Pressed),
    On(Pressed),
    Optional(&'static ChordEvent),
    RAny,
    LAny,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Emit<T: 'static> {
    Mod(&'static Emit<T>),
    Ctrl(&'static Emit<T>),
    Shift(&'static Emit<T>),
    Alt(&'static Emit<T>),
    String(&'static str),
    Code(T),
    Identity,
}

#[derive(Debug)]
pub struct ChordEmit<T: 'static>(pub &'static [ChordEvent], pub Emit<T>);

fn rule_match(chord: &Vec<Pressed, PRESS_SIZE>, rule_events: &[ChordEvent]) -> bool {
    let mut ixoffset: i8 = 0;

    //TODO: skip if chord.len != events.len
    for (ix, event) in rule_events.iter().enumerate() {
        let ix = (ix as i8 + ixoffset) as usize;
        if ix >= chord.len() {
            return false;
        }
        // println!("ix {} chrd {:?} evt {:?}", ix, chord[ix], event);
        match event {
            ChordEvent::Optional(opt) => {
                let mut subchord: Vec<Pressed, PRESS_SIZE> = Vec::new();
                subchord.extend_from_slice(&chord[ix..]).unwrap(); //TODO: fix me
                let event = *(*opt);
                if !rule_match(&subchord, &[event]) {
                    ixoffset -= 1;
                }
            }
            ChordEvent::Any(a, b, c, d) => {
                if chord[ix] != *a && chord[ix] != *b && chord[ix] != *c && chord[ix] != *d {
                    return false;
                }
            }
            ChordEvent::Both(p1, p2) => {
                // we need to look ahead when encountering a both scenario
                if chord.len() < ix + 2 {
                    return false;
                }
                let ch1 = chord[ix];
                let ch2 = chord[ix + 1];
                if ch1 != *p1 && ch2 != *p2 && ch1 != *p2 && ch2 != *p1 {
                    return false;
                }
                ixoffset += 1;
            }
            ChordEvent::On(pressed) => {
                if *pressed != chord[ix] {
                    return false;
                }
            }
            ChordEvent::RAny => {
                let Pressed(key) = chord[ix];
                if !key.is_right() {
                    return false;
                }
            }
            ChordEvent::LAny => {
                let Pressed(key) = chord[ix];
                if !key.is_left() {
                    return false;
                }
            }
        }
    }
    true
}

pub fn parse_with<T: 'static, const RULE_SIZE: usize>(
    chord: &Vec<Pressed, PRESS_SIZE>,
    rules: [ChordEmit<T>; RULE_SIZE],
) -> Emit<T> {
    for rule in rules {
        if rule_match(chord, rule.0) {
            return rule.1;
        }
    }
    Emit::Identity
}

#[cfg(test)]
mod tests {
    use heapless::Vec;

    use super::*;
    use crate::{
        lex::{qwerty::*, PRESS_SIZE},
        parse::{
            ChordEmit,
            ChordEvent::{self, *},
            Emit::{self, *},
        },
    };

    // chord!(SHIFT_L, 2, [On(D), RAny], Shift(&Identity));
    const SHIFT_L_EVENTS: [ChordEvent; 2] = [On(D), RAny];
    const SHIFT_L: ChordEmit<u8> = ChordEmit(&SHIFT_L_EVENTS, Shift(&Identity));

    const CONTROL_SHIFT_R_EVENTS: [ChordEvent; 2] = [Both(H, J), LAny];
    const CONTROL_SHIFT_R: ChordEmit<u8> =
        ChordEmit(&CONTROL_SHIFT_R_EVENTS, Ctrl(&Shift(&Identity)));

    const Q_CODE_EVENTS: [ChordEvent; 2] = [Both(H, J), On(Q)];
    const Q_CODE: ChordEmit<u8> = ChordEmit(&Q_CODE_EVENTS, Ctrl(&Shift(&Code(42))));

    const W_STRING_EVENTS: [ChordEvent; 2] = [Both(H, J), On(W)];
    const W_STRING: ChordEmit<u8> =
        ChordEmit(&W_STRING_EVENTS, Ctrl(&Shift(&String("Hello World"))));

    const OPT_CTRL_R1_EVENTS: [ChordEvent; 3] = [
        Optional(&On(Pressed(crate::lex::Key::L16))),
        On(Pressed(crate::lex::Key::L9)),
        On(Pressed(crate::lex::Key::R1)),
    ];
    const OPT_CTRL_R1: ChordEmit<u8> = ChordEmit(&OPT_CTRL_R1_EVENTS, Ctrl(&String("Optional")));

    const OPT_MOD_SHIFT_R1_EVENTS: [ChordEvent; 3] = [
        Optional(&Any(
            Pressed(crate::lex::Key::L16),
            Pressed(crate::lex::Key::L17),
            Pressed(crate::lex::Key::R16),
            Pressed(crate::lex::Key::R17),
        )),
        On(Pressed(crate::lex::Key::L8)),
        On(Pressed(crate::lex::Key::R1)),
    ];
    const OPT_MOD_SHIFT_R1: ChordEmit<u8> =
        ChordEmit(&OPT_MOD_SHIFT_R1_EVENTS, Ctrl(&String("Optional Modrow")));

    // NB: order matters
    const RULES: [ChordEmit<u8>; 6] = [
        OPT_CTRL_R1,
        OPT_MOD_SHIFT_R1,
        Q_CODE,
        W_STRING,
        CONTROL_SHIFT_R,
        SHIFT_L,
    ];

    #[test]
    fn single_key() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.push(D).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn two_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[D, H]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Emit::Shift(&Emit::Identity), emit);
    }

    #[test]
    fn three_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, D]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&Identity)), emit);
    }

    #[test]
    fn code_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, Q]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&Code(42))), emit);
    }

    #[test]
    fn string_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, W]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&String("Hello World"))), emit);
    }

    #[test]
    fn optional_chord() {
        use crate::lex::Key::{L16, L17, L9};
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord
            .extend_from_slice(&[Pressed(L16), Pressed(L9), P])
            .unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&String("Optional")), emit);

        chord.clear();
        chord
            .extend_from_slice(&[Pressed(L17), Pressed(L9), P])
            .unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Identity, emit);
    }

    #[test]
    fn optional_any_chord() {
        use crate::lex::Key::{L15, L16, L17, L8};
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord
            .extend_from_slice(&[Pressed(L16), Pressed(L8), P])
            .unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&String("Optional Modrow")), emit);

        chord.clear();
        chord
            .extend_from_slice(&[Pressed(L17), Pressed(L8), P])
            .unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&String("Optional Modrow")), emit);

        chord.clear();
        chord
            .extend_from_slice(&[Pressed(L15), Pressed(L8), P])
            .unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Identity, emit);
    }
}
