fn horizon_ultimate_hud_list(
    mods: &mut Vec<crate::game::native_hooks::NativeMod>,
) {
    use crate::game::horizon_ultimate_hud as m;

    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Horizon Ultimate HUD {}", m::VERSION),
        description: "Coordinates HUD and a small status panel.".into(),
        keys: vec![[
            "menu".into(),
            "Toggle HUD menu".into(),
            "key.keyboard.right.shift".into(),
        ]],
    });
}
