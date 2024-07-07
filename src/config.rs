use heapless::Vec;
use tastlib::lex::{Key, CHORD_SIZE};
use tastlib::parse::ChordEvent::*;
use tastlib::{alias, parse::ChordEvent};
use usbd_human_interface_device::page::Keyboard as Keyb;

alias!(TAB, L16);
alias!(BCK, L17);
alias!(RET, R17);
alias!(SPC, R16);

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

const OPT_HRMODS: ChordEvent = Optional(&Any(TAB, BCK, SPC, RET));

pub fn base_layer(key: Key) -> Vec<Keyb, CHORD_SIZE> {
    println!("RUN: BASE layer for {:?}", key);
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();

    macro_rules! key {
        ($keyb:ident) => {
            keyboard.push(Keyb::$keyb).unwrap()
        };
    }
    use tastlib::lex::Key::*;
    match key {
        // QWERTYUIOP
        L1 => key!(Q),
        L2 => key!(W),
        L3 => key!(E),
        L4 => key!(R),
        L5 => key!(T),
        R5 => key!(Y),
        R4 => key!(U),
        R3 => key!(I),
        R2 => key!(O),
        R1 => key!(P),
        // ASDFGHJKL;
        L6 => key!(A),
        L7 => key!(S),
        L8 => key!(D),
        L9 => key!(F),
        L10 => key!(G),
        R10 => key!(H),
        R9 => key!(J),
        R8 => key!(K),
        R7 => key!(L),
        R6 => key!(Semicolon),
        // ZXCVBNM,./
        L11 => key!(Z),
        L12 => key!(X),
        L13 => key!(C),
        L14 => key!(V),
        L15 => key!(B),
        R15 => key!(N),
        R14 => key!(M),
        R13 => key!(Comma),
        R12 => key!(Dot),
        R11 => key!(ForwardSlash),
        // thumb cluster
        L16 => key!(Tab),
        L17 => key!(DeleteBackspace),
        R17 => key!(ReturnEnter),
        R16 => key!(Space),
        LAny => todo!("LAny should not occur here"),
        RAny => todo!("RAny should not occur here"),
    };
    keyboard
}
/// Automatically shift base layer
pub fn tab_layer(key: Key) -> Vec<Keyb, CHORD_SIZE> {
    println!("RUN: TAB layer for {:?}", key);
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
    keyboard.push(Keyb::LeftShift).unwrap();
    keyboard.extend(base_layer(key));
    keyboard
}
pub fn bck_layer(key: Key) -> Vec<Keyb, CHORD_SIZE> {
    println!("RUN: BCK layer for {:?}", key);
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
    // TODO: try to make ÆØÅ work
    // switch language, then emit "Æ ;Ø [å
    match key {
        Key::L3 => keyboard.push(Keyb::Apostrophe).unwrap(),
        Key::L6 => keyboard.push(Keyb::Semicolon).unwrap(),
        Key::R2 => keyboard.push(Keyb::LeftBrace).unwrap(),
        _ => {
            keyboard.extend(base_layer(key));
        }
    };
    keyboard
}
pub fn spc_layer(key: Key) -> Vec<Keyb, CHORD_SIZE> {
    println!("RUN: SPC layer for {:?}", key);
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
    macro_rules! key {
        ($keyb:ident) => {
            keyboard.push(Keyb::$keyb).unwrap()
        };
    }
    match key {
        Key::L1 => key!(Keyboard1),
        Key::L2 => key!(Keyboard2),
        Key::L3 => key!(Keyboard3),
        Key::L4 => key!(Keyboard4),
        Key::L5 => key!(Keyboard5),
        Key::R5 => key!(Keyboard6),
        Key::R4 => key!(Keyboard7),
        Key::R3 => key!(Keyboard8),
        Key::R2 => key!(Keyboard9),
        Key::R1 => key!(Keyboard0),
        Key::L6 => key!(F1),
        Key::L7 => key!(F2),
        Key::L8 => key!(F3),
        Key::L9 => key!(F4),
        Key::L10 => key!(F5),
        Key::R10 => key!(F6),
        Key::R9 => key!(F7),
        Key::R8 => key!(F8),
        Key::R7 => key!(F9),
        Key::R6 => key!(F10),
        Key::L15 => key!(F11),
        Key::R15 => key!(F12),
        _ => {
            keyboard.extend(base_layer(key));
        }
    };
    keyboard
}

pub fn ret_layer(key: Key) -> Vec<Keyb, CHORD_SIZE> {
    println!("RUN: RET layer for {:?}", key);
    let mut keyboard: Vec<Keyb, CHORD_SIZE> = Vec::new();
    macro_rules! key {
        ($keyb:ident) => {
            keyboard.push(Keyb::$keyb).unwrap()
        };
    }
    #[rustfmt::skip]
    match key {
        /* QWERTYUIOP
           !@#$%^&*() */
        Key::L1 =>  { key!(LeftShift); key!(Keyboard1); }
        Key::L2 =>  { key!(LeftShift); key!(Keyboard2); }
        Key::L3 =>  { key!(LeftShift); key!(Keyboard3); }
        Key::L4 =>  { key!(LeftShift); key!(Keyboard4); }
        Key::L5 =>  { key!(LeftShift); key!(Keyboard5); }
        Key::R5 =>  { key!(LeftShift); key!(Keyboard6); }
        Key::R4 =>  { key!(LeftShift); key!(Keyboard7); }
        Key::R3 =>  { key!(LeftShift); key!(Keyboard8); }
        Key::R2 =>  { key!(LeftShift); key!(Keyboard9); }
        Key::R1 =>  { key!(LeftShift); key!(Keyboard0); }
        /* ASDFGHJKL;
           <[({|\})]> */
        Key::L6 =>  { key!(LeftShift); key!(LeftBrace); }
        Key::L7 =>  { key!(LeftShift); key!(Keyboard9); }
        Key::L8 =>  { key!(LeftBrace); } 
        Key::L9 =>  { key!(LeftShift); key!(Comma); }
        Key::L10 => { key!(LeftShift); key!(Backslash); }
        Key::R10 => { key!(Backslash); }
        Key::R9 =>  { key!(LeftShift); key!(Dot); }
        Key::R8 =>  { key!(RightBrace); }
        Key::R7 =>  { key!(LeftShift); key!(Keyboard0); }
        Key::R6 =>  { key!(LeftShift); key!(RightBrace); }
        /* ZXCVBNM,./
           ~`+'= "-_  */
        Key::L11 => { key!(LeftShift); key!(Grave); }
        Key::L12 => { key!(Grave); }
        Key::L13 => { key!(LeftShift); key!(Equal); }
        Key::L14 => { key!(Apostrophe); }
        Key::L15 => { key!(Equal); }
        Key::R15 => { /*NOT_BOUND*/ }
        Key::R14 => { key!(LeftShift); key!(Apostrophe); }
        Key::R13 => { key!(Minus); }
        Key::R12 => { key!(LeftShift); key!(Minus); }
        Key::R11 => { /*NOT_BOUND*/ }

        Key::R17 => { key!(ReturnEnter); }
        _ => {}
    };
    keyboard
}

#[rustfmt::skip]
mod unformatted {
    use tastlib::chord;
    // Homerow mods right
    chord!( R_GUI,          3, [OPT_HRMODS, On(R_G), LAny],                     Mod(&Identity));
    chord!( R_ALT,          3, [OPT_HRMODS, On(R_A), LAny],                     Alt(&Identity));
    chord!( R_SHIFT,        3, [OPT_HRMODS, On(R_S), LAny],                     Shift(&Identity));
    chord!( R_CTRL,         3, [OPT_HRMODS, On(R_C), LAny],                     Ctrl(&Identity));
    chord!( R_GUI_ALT,      3, [OPT_HRMODS, Both(R_G, R_A), LAny],              Mod(&Alt(&Identity)));
    chord!( R_GUI_SHIFT,    3, [OPT_HRMODS, Both(R_G, R_S), LAny],              Mod(&Shift(&Identity)));
    chord!( R_GUI_CTRL,     3, [OPT_HRMODS, Both(R_G, R_C), LAny],              Mod(&Ctrl(&Identity)));
    chord!( R_ALT_SHIFT,    3, [OPT_HRMODS, Both(R_A, R_S), LAny],              Alt(&Shift(&Identity)));
    chord!( R_CTRL_ALT,     3, [OPT_HRMODS, Both(R_C, R_A), LAny],              Ctrl(&Alt(&Identity)));
    chord!( R_CTRL_SHIFT,   3, [OPT_HRMODS, Both(R_C, R_S), LAny],              Ctrl(&Shift(&Identity)));

    // Homerow mods left
    chord!( L_GUI,          3, [OPT_HRMODS, On(L_G), RAny],                     Mod(&Identity));
    chord!( L_ALT,          3, [OPT_HRMODS, On(L_A), RAny],                     Alt(&Identity));
    chord!( L_SHIFT,        3, [OPT_HRMODS, On(L_S), RAny],                     Shift(&Identity));
    chord!( L_CTRL,         3, [OPT_HRMODS, On(L_C), RAny],                     Ctrl(&Identity));
    chord!( L_GUI_ALT,      3, [OPT_HRMODS, Both(L_G, L_A), RAny],              Mod(&Alt(&Identity)));
    chord!( L_GUI_SHIFT,    3, [OPT_HRMODS, Both(L_G, L_S), RAny],              Mod(&Shift(&Identity)));
    chord!( L_GUI_CTRL,     3, [OPT_HRMODS, Both(L_G, L_C), RAny],              Mod(&Ctrl(&Identity)));
    chord!( L_ALT_SHIFT,    3, [OPT_HRMODS, Both(L_A, L_S), RAny],              Alt(&Shift(&Identity)));
    chord!( L_CTRL_ALT,     3, [OPT_HRMODS, Both(L_C, L_A), RAny],              Ctrl(&Alt(&Identity)));
    chord!( L_CTRL_SHIFT,   3, [OPT_HRMODS, Both(L_C, L_S), RAny],              Ctrl(&Shift(&Identity)));


    chord!( L_ALLMOD,       4, [On(L_C), On(L_A), On(L_S), RAny],   Ctrl(&Alt(&Shift(&Identity))));
}
pub use unformatted::*;
