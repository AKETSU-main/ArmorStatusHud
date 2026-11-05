use horizon::render::ui::Color;

pub struct ArmorHudState {
    pub enabled: bool,
    pub color: Color,
}

impl ArmorHudState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            color: Color::WHITE,
        }
    }

    pub fn get_armor_status_lines(&self, player: &horizon::world::player::Player) -> Vec<(String, Color)> {
        let mut lines = Vec::new();
        let armor_slots = player.inventory.get_armor_slots();

        for slot in armor_slots {
            if let Some(item) = &slot.item {
                let durability = item.durability();
                let max_durability = item.max_durability();
                let name = item.name();

                // Цвет становится красным, если прочность ниже 20%
                let color = if durability < (max_durability / 5) {
                    Color::RED
                } else {
                    self.color
                };

                lines.push((
                    format!("{}: {}/{}", name, durability, max_durability),
                    color,
                ));
            }
        }
        lines
    }
}
