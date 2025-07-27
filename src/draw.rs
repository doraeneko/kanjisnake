// Snaker
// (C) 2025, part of Kanjiban by JoAn.
// Drawing facilities.

use crate::arena::*;
use macroquad::prelude::*;

pub fn draw_arena(arena: &Arena, kanji_font: &Font) {
    clear_background(BLACK);
    // Get current window size
    let win_w = screen_width();
    let win_h = screen_height();

    // Compute cell size dynamically (float for scaling)
    let cell_w = win_w / arena.width as f32;
    let cell_h = win_h / arena.height as f32;
    for y in 0..arena.height {
        for x in 0..arena.width {
            let mut kanji_to_draw: char = ' ';
            let color = match arena.get(x, y) {
                CellContent::Empty => LIGHTGRAY,
                CellContent::Snake(c) => {
                    kanji_to_draw = c;
                    if c == '$' { ORANGE } else { SKYBLUE }
                }
                CellContent::Food(c) => {
                    kanji_to_draw = c;
                    GREEN
                }
            };
            let px = x as f32 * cell_w;
            let py = y as f32 * cell_h;

            draw_rectangle(px, py, cell_w, cell_h, color);
            if kanji_to_draw != ' ' {
                let font_size = (cell_h * 0.8) as u16;
                let text_dimensions =
                    measure_text(&kanji_to_draw.to_string(), Some(kanji_font), font_size, 1.0);
                let text_x = px + (cell_w - text_dimensions.width) / 2.0;
                let text_y = py + (cell_h + text_dimensions.height) / 2.0;

                draw_text_ex(
                    &kanji_to_draw.to_string(),
                    text_x,
                    text_y,
                    TextParams {
                        font: Some(kanji_font),
                        font_size: font_size as u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        }
    }

    draw_text(
        &format!("Counter: {}", arena.food_left()),
        20.0,
        40.0,
        30.0,
        BLACK,
    );
}

pub fn game_over(won: bool) {
    let status = if won { "You won!" } else { "You lost!" };
    clear_background(WHITE);
    let text = format!("Game Over. {} Press [enter] to play again.", status);
    let font_size = 30.;
    let text_size = measure_text(&text, None, font_size as _, 1.0);

    draw_text(
        &text,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. + text_size.height / 2.,
        font_size,
        DARKGRAY,
    );
}
