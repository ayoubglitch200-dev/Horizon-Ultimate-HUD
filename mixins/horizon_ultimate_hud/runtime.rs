use std::cell::Cell;

pub const ID: &str = "horizon_ultimate_hud";
pub const VERSION: &str = "0.1.0";

thread_local! {
    static MENU_OPEN: Cell<bool> = const { Cell::new(false) };
}

pub fn toggle_menu() {
    MENU_OPEN.with(|open| open.set(!open.get()));
}

pub fn menu_open() -> bool {
    MENU_OPEN.with(Cell::get)
}

#[cfg(test)]
mod tests {
    #[test]
    fn menu_starts_closed_and_toggles() {
        assert!(!super::menu_open());
        super::toggle_menu();
        assert!(super::menu_open());
        super::toggle_menu();
        assert!(!super::menu_open());
    }
}
