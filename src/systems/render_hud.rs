use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Energy)]
#[read_component(Spawner)]
#[read_component(Point)]
pub fn render_hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0);
    if let Some(player_health) = player_health {
        draw_batch.bar_horizontal(
            Point::new(1, 41),
            SCREEN_WIDTH - 2,
            player_health.hp,
            player_health.max_hp,
            ColorPair::new(RED, BLACK),
        );
    }
    draw_batch.print_centered(41, "Health");
    let player_energy = <&Energy>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0);
    if let Some(player_energy) = player_energy {
        draw_batch.bar_horizontal(
            Point::new(1, 42),
            SCREEN_WIDTH - 2,
            player_energy.energy,
            player_energy.max_energy,
            ColorPair::new(BLUE, BLACK),
        );
    }
    draw_batch.print_centered(42, "Energy");
    draw_batch.draw_hollow_box(Rect::with_size(0, 40, 39, 9), ColorPair::new(BLUE, BLACK));
    draw_batch.print_color_centered(40, "Test HUD", ColorPair::new(RED, WHITE));
    draw_batch.print(Point::new(2, 44), "1. Laser");
    //check if the player stopped on a spawner and wants to leave
    let player = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0);
    let spawners: Vec<Point> = <&Point>::query()
        .filter(component::<Spawner>())
        .iter(ecs)
        .map(|point| *point)
        .collect();
    if let Some(player) = player {
        if spawners.contains(player) {
            draw_batch.print(Point::new(2, 48), "6. Leave");
        }
    }
    draw_batch.submit(2200).expect("Batch error");
}
