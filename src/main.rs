use macroquad::prelude::*;
use reagent_mines_rs::{Cell, Game, Phase};
mod reference_board;

// Board measurements and colors come from executable screenshots, not its code.
const INK: Color = Color::new(0.055, 0.055, 0.055, 1.0);
const PANEL: Color = Color::new(0.18, 0.19, 0.19, 1.0);
const ACCENT: Color = Color::new(0.88, 0.9, 0.9, 1.0);
const MUTED: Color = Color::new(0.67, 0.67, 0.66, 1.0);
const GTK_CANVAS: Color = Color::new(230.0 / 255.0, 230.0 / 255.0, 230.0 / 255.0, 1.0);
const GTK_OPEN: Color = Color::new(218.0 / 255.0, 218.0 / 255.0, 218.0 / 255.0, 1.0);
const GTK_SHADOW: Color = Color::new(0.6, 0.6, 0.6, 1.0);
const CLASSIC_RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
const CLASSIC_BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
const GTK_TEXT: Color = Color::new(46.0 / 255.0, 52.0 / 255.0, 54.0 / 255.0, 1.0);

fn config() -> Conf {
    Conf {
        window_title: "Worthify — Mines in Rust".into(),
        window_width: 640,
        window_height: 780,
        high_dpi: true,
        ..Default::default()
    }
}
fn label(s: &str, x: f32, y: f32, size: f32, color: Color, font: &Font) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size.round() as u16,
            color,
            ..Default::default()
        },
    );
}
fn button(rect: Rect, text: &str, active: bool, font: &Font) -> bool {
    let mouse = vec2(mouse_position().0, mouse_position().1);
    let hover = rect.contains(mouse);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active {
            ACCENT
        } else if hover {
            Color::new(0.25, 0.26, 0.26, 1.0)
        } else {
            PANEL
        },
    );
    let natural = measure_text(text, Some(font), 14, 1.0);
    let fs = (14.0 * ((rect.w - 12.0) / natural.width).min(1.0)) as u16;
    let d = measure_text(text, Some(font), fs, 1.0);
    label(
        text,
        rect.x + (rect.w - d.width) / 2.0,
        rect.y + 22.0,
        fs as f32,
        if active { INK } else { WHITE },
        font,
    );
    hover && is_mouse_button_pressed(MouseButton::Left)
}
fn fixed(index: usize) -> Game {
    let board = &reference_board::BOARDS[index];
    Game::fixed(8, 8, board.mines, board.first)
}
fn bevel(x: f32, y: f32, size: f32, edge: f32, raised: bool) {
    let (light, dark) = if raised {
        (WHITE, GTK_SHADOW)
    } else {
        (GTK_SHADOW, WHITE)
    };
    draw_rectangle(x, y, size, size, dark);
    draw_triangle(vec2(x, y), vec2(x + size, y), vec2(x, y + size), light);
    draw_rectangle(
        x + edge,
        y + edge,
        size - edge * 2.0,
        size - edge * 2.0,
        GTK_CANVAS,
    );
}
fn flag(x: f32, y: f32, s: f32) {
    draw_rectangle(x + 11.0 * s, y + 10.0 * s, 2.0 * s, 5.0 * s, BLACK);
    draw_triangle(
        vec2(x + 4.0 * s, y + 7.0 * s),
        vec2(x + 13.0 * s, y + 3.0 * s),
        vec2(x + 13.0 * s, y + 11.0 * s),
        CLASSIC_RED,
    );
    draw_triangle(
        vec2(x + 4.0 * s, y + 17.0 * s),
        vec2(x + 9.0 * s, y + 14.0 * s),
        vec2(x + 16.0 * s, y + 17.0 * s),
        BLACK,
    );
    draw_triangle(
        vec2(x + 9.0 * s, y + 14.0 * s),
        vec2(x + 12.0 * s, y + 14.0 * s),
        vec2(x + 16.0 * s, y + 17.0 * s),
        BLACK,
    );
}
fn mine(x: f32, y: f32, s: f32) {
    draw_rectangle(x + 9.0 * s, y + 3.0 * s, 3.0 * s, 15.0 * s, BLACK);
    draw_rectangle(x + 3.0 * s, y + 9.0 * s, 15.0 * s, 3.0 * s, BLACK);
    draw_circle(x + 10.5 * s, y + 10.5 * s, 5.0 * s, BLACK);
    draw_rectangle(x + 8.0 * s, y + 8.0 * s, 2.0 * s, s, WHITE);
}
fn number_color(n: u8) -> Color {
    match n {
        1 => CLASSIC_BLUE,
        2 => Color::new(0.0, 128.0 / 255.0, 0.0, 1.0),
        3 => CLASSIC_RED,
        4 => Color::new(0.0, 0.0, 128.0 / 255.0, 1.0),
        5 => Color::new(128.0 / 255.0, 0.0, 0.0, 1.0),
        6 => Color::new(0.0, 128.0 / 255.0, 128.0 / 255.0, 1.0),
        7 => BLACK,
        _ => GTK_SHADOW,
    }
}
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn worthify_status(phase: u32, remaining: i32, revealed: u32);
}
#[macroquad::main(config)]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/DejaVuSans.ttf")).unwrap();
    let bold = load_ttf_font_from_bytes(include_bytes!("../assets/DejaVuSans-Bold.ttf")).unwrap();
    let mut serial = 1_u64;
    let mut game = Game::new(8, 8, 10, serial);
    let mut cursor = (0_usize, 0_usize);
    let mut keyboard_cursor = false;
    let mut flag_mode = false;
    let mut reference = false;
    let mut reference_index = 0_usize;
    let mut replay_step: Option<usize> = None;
    let mut original_size = false;
    let mut started = None;
    let mut elapsed = 0_u64;
    loop {
        clear_background(INK);
        let w = screen_width();
        let h = screen_height();
        let margin = 20.0;
        label("WORTHIFY / FIELD NOTES", margin, 25.0, 14.0, ACCENT, &font);
        label(
            "Mines, reconstructed",
            margin,
            56.0,
            if w < 360.0 { 22.0 } else { 26.0 },
            WHITE,
            &font,
        );
        let bw = ((w - 48.0) / 3.0).min(180.0);
        let left = (w - (bw * 3.0 + 8.0)) / 2.0;
        if button(Rect::new(left, 72.0, bw, 34.0), "New board", false, &font)
            || is_key_pressed(KeyCode::N)
        {
            serial = serial.wrapping_add(1);
            game = Game::new(8, 8, 10, (get_time() * 1_000_000.0) as u64 ^ serial);
            reference = false;
            replay_step = None;
            started = None;
            elapsed = 0;
        }
        if button(
            Rect::new(left + bw + 4.0, 72.0, bw, 34.0),
            "Restart",
            false,
            &font,
        ) || is_key_pressed(KeyCode::R)
        {
            game.reset();
            replay_step = None;
            started = None;
            elapsed = 0;
        }
        let reference_label = if reference {
            format!(
                "Reference {} / {}",
                reference_index + 1,
                reference_board::BOARDS.len()
            )
        } else {
            "Reference".to_string()
        };
        if button(
            Rect::new(left + 2.0 * bw + 8.0, 72.0, bw, 34.0),
            &reference_label,
            reference,
            &font,
        ) || is_key_pressed(KeyCode::B)
        {
            reference_index = if reference {
                (reference_index + 1) % reference_board::BOARDS.len()
            } else {
                0
            };
            game = fixed(reference_index);
            reference = true;
            replay_step = None;
            original_size = true;
            keyboard_cursor = false;
            started = None;
            elapsed = 0;
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
            15.0,
            MUTED,
            &font,
        );
        let d = measure_text(phase, Some(&font), 14, 1.0);
        label(phase, w - margin - d.width, 132.0, 14.0, ACCENT, &font);
        // The screenshot's 20px cells are available without changing the board.
        // Larger touch mode scales the same panel and icons together.
        let scale = if original_size {
            1.0
        } else {
            ((w - 40.0) / 220.0)
                .min((h - if reference { 430.0 } else { 260.0 }) / 258.0)
                .clamp(1.0, 2.0)
        };
        let panel_x = ((w - 220.0 * scale) / 2.0).round();
        let panel_y = 150.0;
        let ox = panel_x + 30.0 * scale;
        let oy = panel_y + 31.0 * scale;
        let cell = 20.0 * scale;
        draw_rectangle(panel_x, panel_y, 220.0 * scale, 258.0 * scale, GTK_CANVAS);
        bevel(
            ox - 3.0 * scale,
            oy - 3.0 * scale,
            166.0 * scale,
            3.0 * scale,
            false,
        );
        for (key, dx, dy) in [
            (KeyCode::Left, -1, 0),
            (KeyCode::Right, 1, 0),
            (KeyCode::Up, 0, -1),
            (KeyCode::Down, 0, 1),
        ] {
            if is_key_pressed(key) {
                cursor.0 = (cursor.0 as i32 + dx).clamp(0, 7) as usize;
                cursor.1 = (cursor.1 as i32 + dy).clamp(0, 7) as usize;
                keyboard_cursor = true;
            }
        }
        if is_key_pressed(KeyCode::Space) {
            keyboard_cursor = true;
            replay_step = None;
            game.flag(cursor.0, cursor.1);
        }
        if is_key_pressed(KeyCode::Enter) {
            keyboard_cursor = true;
            replay_step = None;
            game.reveal(cursor.0, cursor.1);
        }
        let (mx, my) = mouse_position();
        if mx >= ox && my >= oy && mx < ox + cell * 8.0 && my < oy + cell * 8.0 {
            let pos = (((mx - ox) / cell) as usize, ((my - oy) / cell) as usize);
            if is_mouse_button_pressed(MouseButton::Right) {
                cursor = pos;
                keyboard_cursor = false;
                replay_step = None;
                game.flag(pos.0, pos.1);
            } else if is_mouse_button_pressed(MouseButton::Left) {
                cursor = pos;
                keyboard_cursor = false;
                replay_step = None;
                if flag_mode {
                    game.flag(pos.0, pos.1);
                } else {
                    game.reveal(pos.0, pos.1);
                }
            }
        }
        if game.phase == Phase::Playing {
            let t = *started.get_or_insert_with(get_time);
            elapsed = (get_time() - t) as u64;
        }
        for y in 0..8 {
            for x in 0..8 {
                let px = ox + x as f32 * cell;
                let py = oy + y as f32 * cell;
                let c = game.cells[y * 8 + x];
                match c {
                    Cell::Hidden | Cell::Flag => bevel(px, py, cell, 2.0 * scale, true),
                    Cell::Open(_) | Cell::Exploded => {
                        draw_rectangle(px, py, cell, cell, GTK_SHADOW);
                        draw_rectangle(
                            px + scale,
                            py + scale,
                            cell - scale,
                            cell - scale,
                            if c == Cell::Exploded {
                                CLASSIC_RED
                            } else {
                                GTK_OPEN
                            },
                        );
                    }
                }
                match c {
                    Cell::Flag => flag(px, py, scale),
                    Cell::Exploded => mine(px, py, scale),
                    Cell::Open(n) if n > 0 => {
                        let s = n.to_string();
                        let fs = (18.0 * scale).round() as u16;
                        let d = measure_text(&s, Some(&bold), fs, 1.0);
                        label(
                            &s,
                            px + (cell - d.width) / 2.0,
                            py + 16.0 * scale,
                            fs as f32,
                            number_color(n),
                            &bold,
                        );
                    }
                    _ => {}
                }
                if keyboard_cursor && cursor == (x, y) {
                    draw_rectangle_lines(
                        px + 3.0 * scale,
                        py + 3.0 * scale,
                        cell - 6.0 * scale,
                        cell - 6.0 * scale,
                        scale,
                        GTK_TEXT,
                    );
                }
            }
        }
        let minutes = elapsed / 60;
        let seconds = elapsed % 60;
        let status = match game.phase {
            Phase::Lost => format!("[{minutes}:{seconds:02}] DEAD!"),
            _ => format!(
                "[{minutes}:{seconds:02}] Marked: {} / {}",
                game.mine_count as isize - game.remaining(),
                game.mine_count
            ),
        };
        label(
            &status,
            panel_x + 14.0 * scale,
            panel_y + 244.0 * scale,
            13.0 * scale,
            GTK_TEXT,
            &font,
        );
        let bottom = panel_y + 258.0 * scale + 12.0;
        let controls = 220.0 * scale;
        if button(
            Rect::new(panel_x, bottom, controls * 0.64 - 2.0, 34.0),
            if flag_mode {
                "Flag mode ON"
            } else {
                "Reveal mode"
            },
            flag_mode,
            &font,
        ) {
            flag_mode = !flag_mode;
        }
        if button(
            Rect::new(
                panel_x + controls * 0.64 + 2.0,
                bottom,
                controls * 0.36 - 2.0,
                34.0,
            ),
            if original_size {
                "Touch size"
            } else {
                "Original size"
            },
            false,
            &font,
        ) {
            original_size = !original_size;
        }
        label(
            if w < 400.0 {
                "Tap: reveal/chord | Mode: flag"
            } else {
                "Click: reveal / chord  |  Right-click: flag"
            },
            margin,
            bottom + 58.0,
            if w < 400.0 { 12.0 } else { 14.0 },
            MUTED,
            &font,
        );
        label(
            if w < 400.0 {
                "Keys: arrows/Enter/Space · F: mode · B/Q: replay"
            } else {
                "Arrows + Enter / Space  |  F: mode  R: restart  B: board  Q: replay"
            },
            margin,
            bottom + 80.0,
            if w < 400.0 { 11.0 } else { 13.0 },
            MUTED,
            &font,
        );
        if reference {
            let id = reference_board::BOARDS[reference_index].game_id;
            label(
                id,
                margin,
                bottom + 104.0,
                if w < 400.0 { 12.0 } else { 14.0 },
                ACCENT,
                &font,
            );
            label(
                "Observed board · click Reference to cycle",
                margin,
                bottom + 124.0,
                if w < 400.0 { 11.0 } else { 13.0 },
                MUTED,
                &font,
            );
            let replay_label = match replay_step {
                None => "Flagged empty square",
                Some(1) => "Step 2: reveal (6,0)",
                Some(2) => "Step 3: unflag (7,0)",
                Some(3) => "Step 4: click open (6,0)",
                _ => "Replay again",
            };
            if button(
                Rect::new(margin, bottom + 136.0, w - margin * 2.0, 34.0),
                replay_label,
                false,
                &font,
            ) || is_key_pressed(KeyCode::Q)
            {
                match replay_step {
                    None | Some(4..) => {
                        game = fixed(0);
                        reference_index = 0;
                        original_size = true;
                        keyboard_cursor = false;
                        started = None;
                        elapsed = 0;
                        game.flag(7, 0);
                        replay_step = Some(1);
                    }
                    Some(1) => {
                        game.reveal(6, 0);
                        replay_step = Some(2);
                    }
                    Some(2) => {
                        game.flag(7, 0);
                        replay_step = Some(3);
                    }
                    Some(3) => {
                        game.reveal(6, 0);
                        replay_step = Some(4);
                    }
                    _ => {}
                }
            }
            let replay_status = match replay_step {
                Some(1) => "1 / 4 · Flagged (7,0)",
                Some(2) => "2 / 4 · Revealed (6,0); flag stops the flood",
                Some(3) => "3 / 4 · Unflagged (7,0); square stays covered",
                Some(4) => "4 / 4 · Clicked open zero; (7,0) stays covered",
                _ => "Replay: board 1, one real action per step",
            };
            label(
                replay_status,
                margin,
                bottom + 190.0,
                if w < 400.0 { 11.0 } else { 13.0 },
                MUTED,
                &font,
            );
        }
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
