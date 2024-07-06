use tastlib::lex::Key;
use tastlib::parse::ChordEvent::*;
use tastlib::{alias, parse::ChordEvent};

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

const LAYER_KEYS: ChordEvent = Optional(&Any(TAB, BCK, SPC, RET));

pub fn base_layer(key: Key) -> usbd_human_interface_device::page::Keyboard {
    println!("Base layer for {:?}", key);
    if key == Key::R2 {
        println!("emit: o");
    }
    usbd_human_interface_device::page::Keyboard::NoEventIndicated
}
pub fn tab_layer(key: Key) -> usbd_human_interface_device::page::Keyboard {
    println!("Tab layer for {:?}", key);
    if key == Key::R2 {
        println!("emit: Ω");
    }
    usbd_human_interface_device::page::Keyboard::NoEventIndicated
}
pub fn bck_layer(key: Key) -> usbd_human_interface_device::page::Keyboard {
    println!("Bck layer for {:?}", key);
    usbd_human_interface_device::page::Keyboard::NoEventIndicated
}
pub fn spc_layer(key: Key) -> usbd_human_interface_device::page::Keyboard {
    println!("Spc layer for {:?}", key);
    usbd_human_interface_device::page::Keyboard::NoEventIndicated
}
pub fn ret_layer(key: Key) -> usbd_human_interface_device::page::Keyboard {
    println!("Ret layer for {:?}", key);
    usbd_human_interface_device::page::Keyboard::NoEventIndicated
}

#[rustfmt::skip]
mod unformatted {
    use tastlib::chord;
    // Homerow mods right
    chord!( R_GUI,          3, [LAYER_KEYS, On(R_G), LAny],                     Mod(&Identity));
    chord!( R_ALT,          3, [LAYER_KEYS, On(R_A), LAny],                     Alt(&Identity));
    chord!( R_SHIFT,        3, [LAYER_KEYS, On(R_S), LAny],                     Shift(&Identity));
    chord!( R_CTRL,         3, [LAYER_KEYS, On(R_C), LAny],                     Ctrl(&Identity));
    chord!( R_GUI_ALT,      3, [LAYER_KEYS, Both(R_G, R_A), LAny],              Mod(&Alt(&Identity)));
    chord!( R_GUI_SHIFT,    3, [LAYER_KEYS, Both(R_G, R_S), LAny],              Mod(&Shift(&Identity)));
    chord!( R_GUI_CTRL,     3, [LAYER_KEYS, Both(R_G, R_C), LAny],              Mod(&Ctrl(&Identity)));
    chord!( R_ALT_SHIFT,    3, [LAYER_KEYS, Both(R_A, R_S), LAny],              Alt(&Shift(&Identity)));
    chord!( R_CTRL_ALT,     3, [LAYER_KEYS, Both(R_C, R_A), LAny],              Ctrl(&Alt(&Identity)));
    chord!( R_CTRL_SHIFT,   3, [LAYER_KEYS, Both(R_C, R_S), LAny],              Ctrl(&Shift(&Identity)));

    // Homerow mods left
    chord!( L_GUI,          3, [LAYER_KEYS, On(L_G), RAny],                     Mod(&Identity));
    chord!( L_ALT,          3, [LAYER_KEYS, On(L_A), RAny],                     Alt(&Identity));
    chord!( L_SHIFT,        3, [LAYER_KEYS, On(L_S), RAny],                     Shift(&Identity));
    chord!( L_CTRL,         3, [LAYER_KEYS, On(L_C), RAny],                     Ctrl(&Identity));
    chord!( L_GUI_ALT,      3, [LAYER_KEYS, Both(L_G, L_A), RAny],              Mod(&Alt(&Identity)));
    chord!( L_GUI_SHIFT,    3, [LAYER_KEYS, Both(L_G, L_S), RAny],              Mod(&Shift(&Identity)));
    chord!( L_GUI_CTRL,     3, [LAYER_KEYS, Both(L_G, L_C), RAny],              Mod(&Ctrl(&Identity)));
    chord!( L_ALT_SHIFT,    3, [LAYER_KEYS, Both(L_A, L_S), RAny],              Alt(&Shift(&Identity)));
    chord!( L_CTRL_ALT,     3, [LAYER_KEYS, Both(L_C, L_A), RAny],              Ctrl(&Alt(&Identity)));
    chord!( L_CTRL_SHIFT,   3, [LAYER_KEYS, Both(L_C, L_S), RAny],              Ctrl(&Shift(&Identity)));


    chord!( L_ALLMOD,       4, [On(L_C), On(L_A), On(L_S), RAny],   Ctrl(&Alt(&Shift(&Identity))));
}
pub use unformatted::*;
