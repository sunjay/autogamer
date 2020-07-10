use sdl2::{
    event::Event as SDLEvent,
    keyboard::Mod,
    mouse::{MouseWheelDirection, MouseButton as SDLMouseButton},
};

use crate::Vec2;

use super::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Modifiers {
    /// true if either the left shift button or the right shift button are
    /// currently pressed down
    pub shift_pressed: bool,
    /// true if either the left ctrl button or the right ctrl button are
    /// currently pressed down
    pub ctrl_pressed: bool,
    /// true if either the left alt button or the right alt button are
    /// currently pressed down
    pub alt_pressed: bool,
}

impl Modifiers {
    pub(crate) fn from_sdl2_mods(keymod: Mod) -> Self {
        Self {
            shift_pressed: keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD),
            ctrl_pressed: keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD),
            alt_pressed: keymod.contains(Mod::LALTMOD) || keymod.contains(Mod::RALTMOD),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    pub(crate) fn from_sdl2_mouse_button(btn: SDLMouseButton) -> Option<Self> {
        use SDLMouseButton::*;
        Some(match btn {
            Left => MouseButton::Left,
            Middle => MouseButton::Middle,
            Right => MouseButton::Right,

            Unknown |
            X1 |
            X2 => return None,
        })
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EventKind {
    /// The user has requested that the application quit
    ///
    /// This can occur if the user presses the close button on the window. On
    /// MacOS, this can also occur if the user presses Ctrl+Q.
    Quit,

    /// A key on the keyboard was pressed down
    ///
    /// This event will only be sent once, even if the key is being held down
    KeyDown {
        /// The key that was pressed down
        key: Key,
        /// The current state of the modifier keys on the keyboard
        modifiers: Modifiers,
        /// true if this event is repeated (caused by key repeat)
        repeat: bool,
    },
    /// A key on the keyboard was released
    KeyUp {
        /// The key that was released
        key: Key,
        /// The current state of the modifier keys on the keyboard
        modifiers: Modifiers,
    },

    /// The mouse was moved somewhere in the window
    MouseMove {
        /// The position of the mouse in world coordinates
        mouse_pos: Vec2,
    },
    /// A mouse button was pressed down within the window
    MouseButtonDown {
        button: MouseButton,
        double_click: bool,
    },
    /// A mouse button was released down within the window
    MouseButtonUp {
        button: MouseButton,
    },
    /// The mouse scroll wheel (horizontal or vertical) was used
    MouseWheel {
        /// The amount scrolled horizontally
        ///
        /// A positive value means that the mouse was scrolled to the right and
        /// a negative value means the mouse was scrolled to the left.
        x: i32,
        /// The amount scrolled vertically
        ///
        /// A positive value means the mouse was scrolled forward (towards the
        /// top of the mouse) and a negative value means the mouse was scrolled
        /// backward (towards the bottom of the mouse).
        y: i32,
    },
}

impl EventKind {
    pub(crate) fn from_sdl2_event(event: SDLEvent) -> Option<Self> {
        use SDLEvent::*;
        Some(match event {
            Quit {timestamp: _} => EventKind::Quit,

            KeyDown {
                timestamp: _,
                window_id: _,
                keycode: Some(keycode),
                scancode: _,
                keymod,
                repeat,
            } => {
                EventKind::KeyDown {
                    key: Key::from_sdl2_key(keycode)?,
                    modifiers: Modifiers::from_sdl2_mods(keymod),
                    repeat,
                }
            },
            KeyDown {..} => return None,

            KeyUp {
                timestamp: _,
                window_id: _,
                keycode: Some(keycode),
                scancode: _,
                keymod,
                repeat: _,
            } => {
                EventKind::KeyUp {
                    key: Key::from_sdl2_key(keycode)?,
                    modifiers: Modifiers::from_sdl2_mods(keymod),
                }
            },
            KeyUp {..} => return None,

            MouseMotion {
                timestamp: _,
                window_id: _,
                which: _,
                mousestate: _,
                x,
                y,
                xrel: _,
                yrel: _,
            } => {
                EventKind::MouseMove {
                    mouse_pos: Vec2::new(x as f64, y as f64),
                }
            },

            MouseButtonDown {
                timestamp: _,
                window_id: _,
                which: _,
                mouse_btn,
                clicks,
                x: _,
                y: _,
            } => {
                EventKind::MouseButtonDown {
                    button: MouseButton::from_sdl2_mouse_button(mouse_btn)?,
                    double_click: clicks == 2,
                }
            },

            MouseButtonUp {
                timestamp: _,
                window_id: _,
                which: _,
                mouse_btn,
                clicks: _,
                x: _,
                y: _,
            } => {
                EventKind::MouseButtonUp {
                    button: MouseButton::from_sdl2_mouse_button(mouse_btn)?,
                }
            },

            MouseWheel {
                timestamp: _,
                window_id: _,
                which: _,
                x,
                y,
                direction,
            } => {
                let multiplier = if direction == MouseWheelDirection::Flipped {
                    -1
                } else {
                    1
                };

                EventKind::MouseWheel {
                    x: x * multiplier,
                    y: y * multiplier,
                }
            },

            AppTerminating {..} | AppLowMemory {..} | AppWillEnterBackground {..} | AppDidEnterBackground {..} |
            AppWillEnterForeground {..} | AppDidEnterForeground {..} | Window {..} | TextEditing {..} | TextInput {..} |
            JoyAxisMotion {..} | JoyBallMotion {..} | JoyHatMotion {..} | JoyButtonDown {..} | JoyButtonUp {..} |
            JoyDeviceAdded {..} | JoyDeviceRemoved {..} | ControllerAxisMotion {..} | ControllerButtonDown {..} |
            ControllerButtonUp {..} | ControllerDeviceAdded {..} | ControllerDeviceRemoved {..} |
            ControllerDeviceRemapped {..} | FingerDown {..} | FingerUp {..} | FingerMotion {..} | DollarGesture {..} |
            DollarRecord {..} | MultiGesture {..} | ClipboardUpdate {..} | DropFile {..} | DropText {..} |
            DropBegin {..} | DropComplete {..} | AudioDeviceAdded {..} | AudioDeviceRemoved {..} |
            RenderTargetsReset {..} | RenderDeviceReset {..} | User {..} | Unknown {..} => {
                return None;
            },
        })
    }
}
