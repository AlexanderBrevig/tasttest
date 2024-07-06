pub mod lex;
pub mod parse;

#[macro_export]
macro_rules! chord {
    ($rule:ident, $len:expr, $chord_event:expr, $emit:expr) => {
        #[allow(non_snake_case)]
        mod $rule {
            use tastlib::lex::qwerty::*;
            use tastlib::parse::ChordEvent;
            use tastlib::parse::ChordEvent::*;
            use tastlib::parse::Emit;
            use tastlib::parse::Emit::*;
            pub const CHORD_EVENTS: [ChordEvent; $len] = $chord_event;
            pub const CHORD_EMIT: Emit = $emit;
        }
        use tastlib::parse::ChordEmit;
        const $rule: ChordEmit = ChordEmit(&$rule::CHORD_EVENTS, $rule::CHORD_EMIT);
    };
    ($rule:ident, $chord_event:expr, $emit:expr, $layout:ident) => {
        #[allow(non_snake_case)]
        mod $rule {
            use tastlib::lex::$layout::*;
            use tastlib::parse::ChordEvent;
            use tastlib::parse::ChordEvent::{Both, RAny};
            use tastlib::parse::Emit;
            use tastlib::parse::Emit::{Ctrl, Identity, Shift};
            pub const CHORD_EVENTS: [ChordEvent; 2] = $chord_event;
            pub const CHORD_EMIT: Emit = $emit;
        }
        use tastlib::parse::ChordEmit;
        const $rule: ChordEmit = ChordEmit(&$rule::CHORD_EVENTS, $rule::CHORD_EMIT);
    };
}
