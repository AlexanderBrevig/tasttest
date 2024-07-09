# Temporary Sample for [Tastlib](https://github.com/alexanderbrevig/tastlib)

This is a small library for convergin key up/down events to outputting chords.

This is a playground for my own 34 key layout

- QWERTY layout on top of this naming scheme
- Home row modifiers (GASC / Gui Alt Shift Ctrl)
- Momentary layers on thumb clusters

## Playing around

Since I want to play around with the logic and test without hardware in the loop, I've written this as a kind of emulator.

To play, please read the usage guide and try it out!

```txt
UPPERCASE = key down
lowercase = key up

HOME  tab (L16)
END   backspace (L17)
ENTER return/enter (R17)
SPACE space (R16)

Try typing simple letters:
  Oo              => Keyboard: [O]                     -> `O`

Try using home row modifiers:
  JCcj            => Keyboard: [RightCtrl, C]          -> `C-c`

Try using a momentary layer:
  SPACE I i SPACE => Keyboard: [RightShift, Keyboard8] -> `*`
```

```sh
cargo run
```

## The 34 key layout for this example

```
    L1  L2  L3  L4  L5        R5  R4  R3  R2  R1
    L6  L7  L8  L9  L10       R10 R9  R8  R7  R6
    L11 L12 L13 L14 L15       R15 R14 R13 R12 R11
                L16 L17       R17 R16
```

Mapped to base layer QWERTY:

```
    Q   W   E   R   T         Y   U   I   O   P
    A   S   D   F   G         H   J   K   L   ;
    Z   X   C   V   B         N   M   ,   .   /
                TAB BAC       RET SPC
```
