fn horizon_ultimate_hud_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    pressed: bool,
) {
    use crate::game::horizon_ultimate_hud as m;

    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }

    let menu_key = game.native_mod_key(
        m::ID,
        "menu",
        "key.keyboard.right.shift",
    );
    if key == menu_key {
        m::toggle_menu();
    }
}
