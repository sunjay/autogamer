use sdl2::keyboard::Keycode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Key {
    /// The '`' key to the left of the row of numbers
    Backquote,
    /// The `1` key over the letters.
    Num1,
    /// The `2` key over the letters.
    Num2,
    /// The `3` key over the letters.
    Num3,
    /// The `4` key over the letters.
    Num4,
    /// The `5` key over the letters.
    Num5,
    /// The `6` key over the letters.
    Num6,
    /// The `7` key over the letters.
    Num7,
    /// The `8` key over the letters.
    Num8,
    /// The `9` key over the letters.
    Num9,
    /// The `0` key over the letters.
    Num0,
    /// The `-` key, right after the row of numbers
    Minus,
    /// The `=` key, right after the row of numbers and the `-` key
    Equals,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// The `Home` key
    Home,
    /// The `End` key
    End,
    /// The Backspace/Delete key
    Backspace,
    /// The Delete key (performs forward delete)
    Delete,
    /// The PageUp (PgUp) key
    PageUp,
    /// The PageDown (PgDn) key
    PageDown,
    /// The Enter/Return key, under Backspace
    Enter,
    /// The spacebar key
    Space,
    /// The tab key
    Tab,

    /// The up arrow key
    Up,
    /// The down arrow key
    Down,
    /// The left arrow key
    Left,
    /// The right arrow key
    Right,

    /// The `1` key on the number pad
    Numpad1,
    /// The `2` key on the number pad
    Numpad2,
    /// The `3` key on the number pad
    Numpad3,
    /// The `4` key on the number pad
    Numpad4,
    /// The `5` key on the number pad
    Numpad5,
    /// The `6` key on the number pad
    Numpad6,
    /// The `7` key on the number pad
    Numpad7,
    /// The `8` key on the number pad
    Numpad8,
    /// The `9` key on the number pad
    Numpad9,
    /// The `0` key on the number pad
    Numpad0,
    /// The Enter/Return key on the number pad
    NumpadEnter,
    /// The `+` key on the number pad
    NumpadPlus,
    /// The `-` key on the number pad
    NumpadMinus,
    /// The `*` key on the number pad
    NumpadMultiply,
    /// The `/` key on the number pad
    NumpadDivide,
    /// The `=` key on the number pad
    NumpadEquals,
    /// The `.` key on the number pad
    NumpadPeriod,
    /// The `,` key on the number pad
    NumpadComma,

    /// The `[` key
    LeftBracket,
    /// The `]` key
    RightBracket,
    /// The `\` key
    Backslash,
    /// The `;` key
    Semicolon,
    /// The `'` key
    Quote,
    /// The `,` key
    Comma,
    /// The `.` key
    Period,
    /// The `/` key
    Slash,

    /// The left or right `Ctrl` key
    Ctrl,
    /// The left or right `Shift` key
    Shift,
    /// The left or right `Alt` key
    Alt,

    /// The volume up key
    VolumeUp,
    /// The volume down key
    VolumeDown,

    /// The brightness up key
    BrightnessUp,
    /// The brightness down key
    BrightnessDown,
}

impl Key {
    pub(crate) fn from_sdl2_key(key: Keycode) -> Option<Self> {
        use Keycode::*;
        #[deny(unreachable_patterns)]
        Some(match key {
            Backquote => Key::Backquote,
            Exclaim |
            Num1 => Key::Num1,
            At |
            Num2 => Key::Num2,
            Hash |
            Num3 => Key::Num3,
            Dollar |
            Num4 => Key::Num4,
            Percent |
            Num5 => Key::Num5,
            Caret |
            Num6 => Key::Num6,
            Ampersand |
            Num7 => Key::Num7,
            Asterisk |
            Num8 => Key::Num8,
            LeftParen |
            Num9 => Key::Num9,
            RightParen |
            Num0 => Key::Num0,
            Underscore |
            Minus => Key::Minus,
            Plus |
            Equals => Key::Equals,

            A => Key::A,
            B => Key::B,
            C => Key::C,
            D => Key::D,
            E => Key::E,
            F => Key::F,
            G => Key::G,
            H => Key::H,
            I => Key::I,
            J => Key::J,
            K => Key::K,
            L => Key::L,
            M => Key::M,
            N => Key::N,
            O => Key::O,
            P => Key::P,
            Q => Key::Q,
            R => Key::R,
            S => Key::S,
            T => Key::T,
            U => Key::U,
            V => Key::V,
            W => Key::W,
            X => Key::X,
            Y => Key::Y,
            Z => Key::Z,

            Escape => Key::Escape,

            F1 => Key::F1,
            F2 => Key::F2,
            F3 => Key::F3,
            F4 => Key::F4,
            F5 => Key::F5,
            F6 => Key::F6,
            F7 => Key::F7,
            F8 => Key::F8,
            F9 => Key::F9,
            F10 => Key::F10,
            F11 => Key::F11,
            F12 => Key::F12,
            F13 => Key::F13,
            F14 => Key::F14,
            F15 => Key::F15,
            F16 => Key::F16,
            F17 => Key::F17,
            F18 => Key::F18,
            F19 => Key::F19,
            F20 => Key::F20,
            F21 => Key::F21,
            F22 => Key::F22,
            F23 => Key::F23,
            F24 => Key::F24,

            Home => Key::Home,
            End => Key::End,
            Backspace => Key::Backspace,
            Delete => Key::Delete,
            PageUp => Key::PageUp,
            PageDown => Key::PageDown,
            Return => Key::Enter,
            Space => Key::Space,
            Tab => Key::Tab,

            Right => Key::Right,
            Left => Key::Left,
            Down => Key::Down,
            Up => Key::Up,

            Kp1 => Key::Numpad1,
            Kp2 => Key::Numpad2,
            Kp3 => Key::Numpad3,
            Kp4 => Key::Numpad4,
            Kp5 => Key::Numpad5,
            Kp6 => Key::Numpad6,
            Kp7 => Key::Numpad7,
            Kp8 => Key::Numpad8,
            Kp9 => Key::Numpad9,
            Kp0 => Key::Numpad0,
            KpEnter => Key::NumpadEnter,
            KpPlus => Key::NumpadPlus,
            KpMinus => Key::NumpadMinus,
            KpMultiply => Key::NumpadMultiply,
            KpDivide => Key::NumpadDivide,
            KpEquals => Key::NumpadEquals,
            KpPeriod => Key::NumpadPeriod,
            KpComma => Key::NumpadComma,

            LeftBracket => Key::LeftBracket,
            RightBracket => Key::RightBracket,
            Backslash => Key::Backslash,
            Colon |
            Semicolon => Key::Semicolon,
            Quotedbl |
            Quote => Key::Quote,
            Less |
            Comma => Key::Comma,
            Greater |
            Period => Key::Period,
            Question |
            Slash => Key::Slash,

            LCtrl |
            RCtrl => Key::Ctrl,
            LShift |
            RShift => Key::Shift,
            LAlt |
            RAlt => Key::Alt,

            VolumeUp => Key::VolumeUp,
            VolumeDown => Key::VolumeDown,

            BrightnessDown => Key::BrightnessDown,
            BrightnessUp => Key::BrightnessUp,

            CapsLock | PrintScreen | ScrollLock | Pause | Insert | NumLockClear | Application |
            Power | Execute | Help | Menu | Select | Stop | Again | Undo | Cut | Copy | Paste |
            Find | Mute | KpEqualsAS400 | AltErase | Sysreq | Cancel | Clear | Prior | Return2 |
            Separator | Out | Oper | ClearAgain | CrSel | ExSel | Kp00 | Kp000 |
            ThousandsSeparator | DecimalSeparator | CurrencyUnit | CurrencySubUnit | KpLeftParen |
            KpRightParen | KpLeftBrace | KpRightBrace | KpTab | KpBackspace | KpA | KpB | KpC |
            KpD | KpE | KpF | KpXor | KpPower | KpPercent | KpLess | KpGreater | KpAmpersand |
            KpDblAmpersand | KpVerticalBar | KpDblVerticalBar | KpColon | KpHash | KpSpace | KpAt |
            KpExclam | KpMemStore | KpMemRecall | KpMemClear | KpMemAdd | KpMemSubtract |
            KpMemMultiply | KpMemDivide | KpPlusMinus | KpClear | KpClearEntry | KpBinary |
            KpOctal | KpDecimal | KpHexadecimal | LGui | RGui | Mode | AudioNext | AudioPrev | AudioStop |
            AudioPlay | AudioMute | MediaSelect | Www | Mail | Calculator | Computer | AcSearch |
            AcHome | AcBack | AcForward | AcStop | AcRefresh | AcBookmarks | DisplaySwitch |
            KbdIllumToggle | KbdIllumDown | KbdIllumUp | Eject | Sleep => {
                return None;
            },
        })
    }
}
