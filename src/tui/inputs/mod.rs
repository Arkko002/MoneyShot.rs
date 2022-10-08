use self::keys::Key;

pub mod events;
pub mod keys;

#[derive(Debug)]
pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}
