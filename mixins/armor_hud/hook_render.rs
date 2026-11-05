use crate::game::native_hooks::HudContext;
use crate::game::official_armor_hud::ArmorHudState;

fn on_render_hud(ctx: &mut HudContext) {
    let state = ctx.get_mod_state::<ArmorHudState>();
    if !state.enabled { return; }

    let player = ctx.player();
    let lines = state.get_armor_status_lines(player);

    let mut y_offset = 10.0;
    for (text, color) in lines {
        ctx.renderer.draw_text(
            &text,
            10.0, // X координата (левый край)
            y_offset,
            color,
        );
        y_offset += 12.0;
    }
}
