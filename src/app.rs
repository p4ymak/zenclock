use std::f32::consts::{PI, TAU};

use chrono::{Datelike, Local, Timelike};
use eframe::{
    egui::{self, LayerId, Painter},
    epaint::{Color32, Pos2, Rounding, Stroke},
    App,
};
use lerp::Lerp;

const BLACK: Color32 = Color32::from_rgb(44, 44, 44);
const WHITE: Color32 = Color32::from_rgb(222, 222, 222);
const ORANGE: Color32 = Color32::from_rgb(255, 102, 0);

const RADIUS: f32 = 0.95;
const PIXEL: f32 = 0.002;
const TICK: f32 = 0.95;
const TICK_ACCENT: f32 = 0.9;

const HOUR_W: f32 = 4.0;
const MINUTE_W: f32 = 3.0;
const SECOND_W: f32 = 2.0;
const HOUR_L: f32 = 0.5;
const MINUTE_L: f32 = 0.85;
const SECOND_L: f32 = 1.0;

#[derive(Default)]
pub struct ZenClockApp {}
impl App for ZenClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let scr_rect = ctx.screen_rect();
        let scr_min_dim = scr_rect.width().min(scr_rect.height());
        let radius = scr_min_dim * 0.5 * RADIUS;
        let pixel = scr_min_dim * PIXEL;
        let center = scr_rect.center();
        let painter = Painter::new(ctx.clone(), LayerId::background(), scr_rect);
        painter.rect_filled(scr_rect, Rounding::ZERO, BLACK);

        // Disk
        for tick in 0..60 {
            let (start, stroke) = if tick % 5 == 0 {
                (radius * TICK_ACCENT, Stroke::new(pixel * 2.0, WHITE))
            } else {
                (radius * TICK, Stroke::new(pixel, WHITE))
            };
            let phi = tick as f32 * TAU / 60.0;
            painter.line_segment(
                [polar(center, start, phi), polar(center, radius, phi)],
                stroke,
            );
        }

        let (hours, minutes) = time();
        // Hour
        painter.line_segment(
            [center, polar(center, radius * HOUR_L, hours)],
            Stroke::new(pixel * HOUR_W, WHITE),
        );
        // Minute
        painter.line_segment(
            [center, polar(center, radius * MINUTE_L, minutes)],
            Stroke::new(pixel * MINUTE_W, WHITE),
        );
        // Wild Second
        painter.line_segment(
            [center, polar(center, radius * SECOND_L, wildsecond())],
            Stroke::new(pixel * SECOND_W, ORANGE),
        );

        // Center Dot
        painter.circle_filled(center, pixel * 8.0, ORANGE);

        // Mark 4
        let tan = (TAU * (0.25 + 1.0 / 3.0)).tan() * radius;
        let c4 = Pos2::new(center.x + radius, center.y + tan);
        let c2 = Pos2::new(center.x + radius * 0.9, center.y + tan * 0.9);

        let mark_stroke = Stroke::new(pixel * 1.0, ORANGE);

        painter.hline(c2.x..=c4.x, c4.y, mark_stroke);
        painter.vline(c2.x, c2.y..=c4.y, mark_stroke);
        painter.vline(c4.x, c2.y..=c4.y + (c4.y - c2.y), mark_stroke);
        ctx.request_repaint();
    }
}

fn time() -> (f32, f32) {
    let now = Local::now();
    let h = match now.hour() < 12 {
        true => now.hour(),
        false => now.hour() - 12,
    };

    let m = now.minute();

    let hand_h = -PI * (h as f32) / 6.0 - PI * (m as f32) / 360.0;
    let hand_m = -PI * (m as f32) / 30.0;
    (hand_h, hand_m)
}

fn polar(center: Pos2, radius: f32, phi: f32) -> Pos2 {
    let x = center.x - phi.sin() * radius;
    let y = center.y - phi.cos() * radius;
    Pos2::new(x, y)
}

fn wildsecond() -> f32 {
    let now = Local::now();
    let ns = now.nanosecond() as f32;
    let s = now.second() as f32 + ns * 0.000000001; // 1000000000.0;
    let h = now.hour() as f32;
    let m = now.minute() as f32;
    let d = now.day() as f32;
    let l = now.month() as f32;
    let y = now.year() as f32;

    let weight = 1.0
        - ((PI * 0.5 + PI * s / 20.0).sin().powf(2.0)
            + (PI * 0.5 + PI * s / 12.0).sin().powf(2.0)
            + (PI * 0.5 + PI * s / 30.0).sin().powf(2.0))
            / 3.0;

    let target = 30.0
        + 29.0
            * (m + PI * (m / 30.0 - s * h / 10.0 - m / 20.0 + d / 30.0 + l / 12.0 + y)
                / (10.0 + m / (h + 1.0))
                - m / 20.0)
                .sin()
            * (PI / 2.0 + PI * (y + s * m / (1.0 + m)) / (m + 10.0)).sin();
    let wild = s.lerp(target, weight);
    -PI * wild / 30.0
}
