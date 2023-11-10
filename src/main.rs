#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
use app::ZenClockApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,
        resizable: true,
        maximized: false,
        drag_and_drop_support: true,
        transparent: false,
        vsync: true,
        min_window_size: Some(eframe::egui::Vec2 { x: 100.0, y: 100.0 }),
        icon_data: None,
        initial_window_size: Some(eframe::egui::Vec2 { x: 480.0, y: 480.0 }),
        ..Default::default()
    };

    eframe::run_native(
        "Zen Clock",
        options,
        Box::new(|_| Box::<ZenClockApp>::default()),
    )
}
