use chrono::prelude::*;
use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use lerp::Lerp;
use std::f32::consts::PI;

struct MainState {
    c_x: f32,
    c_y: f32,
    col: (f32, f32, f32, f32),
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            c_x: 0.0,
            c_y: 0.0,
            col: (1.0, 0.4, 0.0, 1.0),
        };
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let w = graphics::drawable_size(ctx).0;
        let h = graphics::drawable_size(ctx).1;

        self.c_x = w / 2.0;
        self.c_y = h / 2.0;

        let scr = ggez::graphics::Rect::new(0.0, 0.0, w, h);
        ggez::graphics::set_screen_coordinates(ctx, scr).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        for m in 0..60 {
            let coords = polar(-PI * (m as f32) / 30.0);
            let click = graphics::Mesh::new_line(
                ctx,
                &[
                    na::Point2::new(
                        coords.0 * self.c_y * 0.9 + self.c_x,
                        coords.1 * self.c_y * 0.9 + self.c_y,
                    ),
                    na::Point2::new(
                        coords.0 * self.c_y * 0.85 + self.c_x,
                        coords.1 * self.c_y * 0.85 + self.c_y,
                    ),
                ],
                self.c_y / 250.0,
                [1.0, 1.0, 1.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &click, (na::Point2::new(0.0, 0.0),))?;
        }

        for h in 0..12 {
            let coords = polar(-PI * (h as f32) / 6.0);
            let click = graphics::Mesh::new_line(
                ctx,
                &[
                    na::Point2::new(
                        coords.0 * self.c_y * 0.9 + self.c_x,
                        coords.1 * self.c_y * 0.9 + self.c_y,
                    ),
                    na::Point2::new(
                        coords.0 * self.c_y * 0.8 + self.c_x,
                        coords.1 * self.c_y * 0.8 + self.c_y,
                    ),
                ],
                self.c_y / 100.0,
                [1.0, 1.0, 1.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &click, (na::Point2::new(0.0, 0.0),))?;
        }

        let h_coords = polar(time().0);
        let hand_h = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(self.c_x, self.c_y),
                na::Point2::new(
                    h_coords.0 * self.c_y * 0.5 + self.c_x,
                    h_coords.1 * self.c_y * 0.5 + self.c_y,
                ),
            ],
            self.c_y / 40.0,
            [1.0, 1.0, 1.0, 0.8].into(),
        )?;
        graphics::draw(ctx, &hand_h, (na::Point2::new(0.0, 0.0),))?;

        let m_coords = polar(time().1);
        let hand_m = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(self.c_x, self.c_y),
                na::Point2::new(
                    m_coords.0 * self.c_y * 0.75 + self.c_x,
                    m_coords.1 * self.c_y * 0.75 + self.c_y,
                ),
            ],
            self.c_y / 50.0,
            [1.0, 1.0, 1.0, 0.8].into(),
        )?;
        graphics::draw(ctx, &hand_m, (na::Point2::new(0.0, 0.0),))?;

        let true_s_coords = polar(-PI * (Local::now().second() as f32) / 30.0);
        let _true_hand_s = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(self.c_x, self.c_y),
                na::Point2::new(
                    true_s_coords.0 * self.c_y * 0.9 + self.c_x,
                    true_s_coords.1 * self.c_y * 0.9 + self.c_y,
                ),
            ],
            self.c_y / 100.0,
            [1.0, 1.0, 1.0, 0.1].into(),
        )?;

        let s_coords = polar(wildsecond());
        let hand_s = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(self.c_x, self.c_y),
                na::Point2::new(
                    s_coords.0 * self.c_y * 0.9 + self.c_x,
                    s_coords.1 * self.c_y * 0.9 + self.c_y,
                ),
            ],
            self.c_y / 100.0,
            self.col.into(),
        )?;
        graphics::draw(ctx, &hand_s, (na::Point2::new(0.0, 0.0),))?;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.c_x, self.c_y),
            self.c_y / 30.0,
            0.05,
            self.col.into(),
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        let four_center = polar(-PI * 4.0 / 6.0);
        let offset = 0.075;
        let width = self.c_y / 100.0;
        let four_part1 = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(
                    self.c_x + four_center.0 * self.c_y,
                    self.c_y + four_center.1 * self.c_y - self.c_y * offset,
                ),
                na::Point2::new(
                    self.c_x + four_center.0 * self.c_y,
                    self.c_y + four_center.1 * self.c_y,
                ),
                na::Point2::new(
                    self.c_x + four_center.0 * self.c_y + self.c_y * offset * 1.1,
                    self.c_y + four_center.1 * self.c_y,
                ),
            ],
            width,
            self.col.into(),
        )?;
        graphics::draw(ctx, &four_part1, (na::Point2::new(0.0, 0.0),))?;
        let four_part2 = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(
                    self.c_x + four_center.0 * self.c_y + self.c_y * offset * 1.1,
                    self.c_y + four_center.1 * self.c_y - self.c_y * offset,
                ),
                na::Point2::new(
                    self.c_x + four_center.0 * self.c_y + self.c_y * offset * 1.1,
                    self.c_y + four_center.1 * self.c_y + self.c_y * offset,
                ),
            ],
            width,
            self.col.into(),
        )?;
        graphics::draw(ctx, &four_part2, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Zen Clock", "Roman Chumak")
        .with_conf_file(false)
        .window_setup(
            conf::WindowSetup::default()
                .title("Zen Clock")
                .vsync(true)
                .samples(conf::NumSamples::Eight),
        )
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::True)
                .borderless(false)
                .resizable(true),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    ggez::input::mouse::set_cursor_hidden(ctx, true);

    event::run(ctx, event_loop, state)
}

fn polar(phi: f32) -> (f32, f32) {
    let x = -phi.sin();
    let y = -phi.cos();
    (x, y)
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

fn wildsecond() -> f32 {
    let now = Local::now();
    let ns = now.nanosecond() as f32;
    let s = now.second() as f32 + ns / 1000000000.0;
    let h = now.hour() as f32;
    let m = now.minute() as f32;
    let d = now.day() as f32;
    let l = now.month() as f32;
    let y = now.year() as f32;

    let weight = 1.0
        - ((PI / 2.0 + PI * s / 20.0).sin().powf(2.0)
            + (PI / 2.0 + PI * s / 12.0).sin().powf(2.0)
            + (PI / 2.0 + PI * s / 30.0).sin().powf(2.0))
            / 3.0;

    let target = 30.0
        + 29.0
            * (m + PI * (m / 30.0 - s * h / 10.0 - m / 20.0 + d / 30.0 + l / 12.0 + y)
                / (10.0 + m / (h + 1.0))
                - m / 20.0)
                .sin()
            * (PI / 2.0 + PI * (y + s * m / (1.0 + m)) / (m + 10.0)).sin();
    let wild = s.lerp(target, weight);
    let hand_s = -PI * wild / 30.0;

    hand_s
}
