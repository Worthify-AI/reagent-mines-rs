use macroquad::prelude::*;
use reagent_mines_rs::{Cell, Game, Phase};
mod reference_board;
const INK: Color = Color::new(0.035, 0.075, 0.12, 1.0);
const PANEL: Color = Color::new(0.09, 0.15, 0.21, 1.0);
const TEAL: Color = Color::new(0.24, 0.88, 0.78, 1.0);
const MUTED: Color = Color::new(0.57, 0.68, 0.74, 1.0);
fn config() -> Conf {
    Conf {
        window_title: "Worthify — Mines in Rust".into(),
        window_width: 640,
        window_height: 780,
        high_dpi: true,
        ..Default::default()
    }
}
fn label(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
}
fn button(rect: Rect, text: &str, active: bool) -> bool {
    let mouse = vec2(mouse_position().0, mouse_position().1);
    let hover = rect.contains(mouse);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active {
            TEAL
        } else if hover {
            Color::new(0.16, 0.26, 0.33, 1.0)
        } else {
            PANEL
        },
    );
    let natural = measure_text(text, None, 16, 1.0);
    let font_size = (16.0 * ((rect.w - 12.0) / natural.width).min(1.0)) as u16;
    let d = measure_text(text, None, font_size, 1.0);
    label(
        text,
        rect.x + (rect.w - d.width) / 2.0,
        rect.y + 23.0,
        font_size as f32,
        if active { INK } else { WHITE },
    );
    hover && is_mouse_button_pressed(MouseButton::Left)
}
fn fixed() -> Game {
    Game::fixed(8, 8, reference_board::MINES, reference_board::FIRST)
}
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn worthify_status(phase: u32, remaining: i32, revealed: u32);
}
#[macroquad::main(config)]
async fn main() {
    let mut serial = 1_u64;
    let mut game = Game::new(8, 8, 10, serial);
    let mut cursor = (0_usize, 0_usize);
    let mut flag_mode = false;
    let mut reference = false;
    loop {
        clear_background(INK);
        let w = screen_width();
        let h = screen_height();
        let margin = 20.0;
        label("WORTHIFY / FIELD NOTES", margin, 25.0, 16.0, TEAL);
        label(
            "Mines, reconstructed",
            margin,
            56.0,
            if w < 360.0 { 24.0 } else { 28.0 },
            WHITE,
        );
        let bw = ((w - 48.0) / 3.0).min(180.0);
        let left = (w - (bw * 3.0 + 8.0)) / 2.0;
        if button(Rect::new(left, 72.0, bw, 34.0), "New board", false) || is_key_pressed(KeyCode::N)
        {
            serial = serial.wrapping_add(1);
            game = Game::new(8, 8, 10, (get_time() * 1_000_000.0) as u64 ^ serial);
            reference = false;
        }
        if button(Rect::new(left + bw + 4.0, 72.0, bw, 34.0), "Restart", false)
            || is_key_pressed(KeyCode::R)
        {
            game.reset();
        }
        if button(
            Rect::new(left + 2.0 * bw + 8.0, 72.0, bw, 34.0),
            "Reference",
            reference,
        ) {
            game = fixed();
            reference = true;
        }
        if is_key_pressed(KeyCode::F) {
            flag_mode = !flag_mode;
        }
        let phase = match game.phase {
            Phase::Ready => "First click safe",
            Phase::Playing => "Keep exploring",
            Phase::Won => "All clear. You won!",
            Phase::Lost => "Mine hit. Restart",
        };
        label(
            &format!("Flags left: {}", game.remaining()),
            margin,
            132.0,
            18.0,
            MUTED,
        );
        let d = measure_text(phase, None, 16, 1.0);
        label(
            phase,
            w - margin - d.width,
            132.0,
            16.0,
            if game.phase == Phase::Lost {
                PINK
            } else {
                TEAL
            },
        );
        let cell = ((w - 40.0) / 8.0).min((h - 260.0) / 8.0).clamp(20.0, 62.0);
        let ox = (w - cell * 8.0) / 2.0;
        let oy = 150.0;
        if is_key_pressed(KeyCode::Left) {
            cursor.0 = cursor.0.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Right) {
            cursor.0 = (cursor.0 + 1).min(7);
        }
        if is_key_pressed(KeyCode::Up) {
            cursor.1 = cursor.1.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            cursor.1 = (cursor.1 + 1).min(7);
        }
        if is_key_pressed(KeyCode::Space) {
            game.flag(cursor.0, cursor.1);
        }
        if is_key_pressed(KeyCode::Enter) {
            game.reveal(cursor.0, cursor.1);
        }
        let (mx, my) = mouse_position();
        if mx >= ox && my >= oy && mx < ox + cell * 8.0 && my < oy + cell * 8.0 {
            let pos = (((mx - ox) / cell) as usize, ((my - oy) / cell) as usize);
            if is_mouse_button_pressed(MouseButton::Right) {
                cursor = pos;
                game.flag(pos.0, pos.1);
            } else if is_mouse_button_pressed(MouseButton::Left) {
                cursor = pos;
                if flag_mode {
                    game.flag(pos.0, pos.1);
                } else {
                    game.reveal(pos.0, pos.1);
                }
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let px = ox + x as f32 * cell;
                let py = oy + y as f32 * cell;
                let c = game.cells[y * 8 + x];
                let bg = match c {
                    Cell::Hidden | Cell::Flag => PANEL,
                    Cell::Exploded => Color::new(0.55, 0.14, 0.23, 1.0),
                    Cell::Open(_) => Color::new(0.055, 0.11, 0.16, 1.0),
                };
                draw_rectangle(px + 1.5, py + 1.5, cell - 3.0, cell - 3.0, bg);
                if cursor == (x, y) {
                    draw_rectangle_lines(px + 3.0, py + 3.0, cell - 6.0, cell - 6.0, 1.2, TEAL);
                }
                match c {
                    Cell::Flag => {
                        draw_line(
                            px + cell * 0.36,
                            py + cell * 0.25,
                            px + cell * 0.36,
                            py + cell * 0.75,
                            2.0,
                            TEAL,
                        );
                        draw_triangle(
                            vec2(px + cell * 0.36, py + cell * 0.25),
                            vec2(px + cell * 0.70, py + cell * 0.39),
                            vec2(px + cell * 0.36, py + cell * 0.52),
                            TEAL,
                        );
                    }
                    Cell::Exploded => {
                        draw_circle(px + cell * 0.5, py + cell * 0.5, cell * 0.14, PINK);
                        for k in 0..8 {
                            let a = k as f32 * std::f32::consts::PI / 4.0;
                            draw_line(
                                px + cell * 0.5 + a.cos() * cell * 0.14,
                                py + cell * 0.5 + a.sin() * cell * 0.14,
                                px + cell * 0.5 + a.cos() * cell * 0.28,
                                py + cell * 0.5 + a.sin() * cell * 0.28,
                                2.0,
                                PINK,
                            );
                        }
                    }
                    Cell::Open(n) if n > 0 => {
                        let s = n.to_string();
                        let fs = (cell * 0.5) as u16;
                        let d = measure_text(&s, None, fs, 1.0);
                        let color = [TEAL, SKYBLUE, GOLD, PINK, VIOLET, ORANGE, WHITE, MUTED]
                            [n as usize - 1];
                        label(
                            &s,
                            px + (cell - d.width) / 2.0,
                            py + cell * 0.68,
                            fs as f32,
                            color,
                        );
                    }
                    _ => {}
                }
            }
        }
        let bottom = oy + cell * 8.0 + 12.0;
        if button(
            Rect::new(ox, bottom, cell * 8.0, 34.0),
            if flag_mode {
                "Flag mode ON - tap to mark"
            } else {
                "Reveal mode - tap to explore"
            },
            flag_mode,
        ) {
            flag_mode = !flag_mode;
        }
        label(
            if w < 400.0 {
                "Tap: reveal/chord | Mode: flag"
            } else {
                "Click: reveal / chord  |  Right-click: flag"
            },
            ox,
            bottom + 58.0,
            if w < 400.0 { 12.0 } else { 15.0 },
            MUTED,
        );
        label(
            if w < 400.0 {
                "Keys: arrows, Enter, Space | F: mode"
            } else {
                "Arrows + Enter / Space  |  F: mode  R: restart"
            },
            ox,
            bottom + 80.0,
            if w < 400.0 { 11.0 } else { 14.0 },
            MUTED,
        );
        #[cfg(target_arch = "wasm32")]
        unsafe {
            worthify_status(
                game.phase as u32,
                game.remaining() as i32,
                game.revealed() as u32,
            );
        }
        next_frame().await;
    }
}
