use {
    crate::{
        event::{
            MouseDownEvent,
            MouseUpEvent,
            MouseMoveEvent,
            ScrollEvent,
            KeyEvent,
            TextInputEvent,
            TimerEvent,
        },
    }
};

#[derive(Debug)]
pub enum OpenHarmonyEvent {
    Paint,
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMove(MouseMoveEvent),
    Scroll(ScrollEvent),
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    TextInput(TextInputEvent),
    Timer(TimerEvent),
}
