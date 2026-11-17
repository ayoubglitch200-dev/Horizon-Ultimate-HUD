fn horizon_ultimate_hud_draw(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::horizon_ultimate_hud as m;
    use horizon_client::extras::Corner;

    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }

    let Some(player) = game.native_player() else {
        return;
    };

    let coords = format!("XYZ: {:.1} / {:.1} / {:.1}", player.x, player.y, player.z);
    let width = game.native_text_width(&coords) + 8.0;
    let (x, y) = corners.place(Corner::TopRight, width, 14.0);
    game.native_fill(batch, [x, y, width, 14.0], 0x90000000);
    game.native_text(batch, &coords, [x + 4.0, y + 3.0], 0xFFFFFF, true);

    if m::menu_open() {
        let lines = [
            "HORIZON ULTIMATE HUD",
            "Right Shift: close menu",
            "Coordinates HUD: active",
            "X-Ray / ESP / Fly / Speed: not included",
            "Fullbright / Teleport: not included",
        ];
        let panel_width = 224.0;
        let panel_height = lines.len() as f32 * 13.0 + 10.0;
        game.native_fill(batch, [8.0, 28.0, panel_width, panel_height], 0xB0000000);
        for (index, line) in lines.iter().enumerate() {
            game.native_text(
                batch,
                line,
                [14.0, 34.0 + index as f32 * 13.0],
                if index == 0 { 0x55FFFF } else { 0xFFFFFF },
                true,
            );
        }
    }
}
