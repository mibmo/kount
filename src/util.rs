#[macro_export]
macro_rules! string_to_field {
    ($field:expr, $state:expr) => {
        match $field {
            "keyboard" => Some($state.keyboard_presses),
            _ => None,
        }
    };
}
