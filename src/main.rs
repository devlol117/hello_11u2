/*
By: <Leo Smilovici>
Date: 2026-09-22
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::utils::preload_image::GifLoadingScreenInfo; // If you want to add animated GIFs to loading screen
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::preload_image::TextureManager;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hello_11u2".to_string(),
        window_width: 1300,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let btn_name = TextButton::new(50.0, 650.0, 200.0, 60.0, "Name", BLUE, GREEN, 30);
    let btn_school = TextButton::new(300.0, 650.0, 200.0, 60.0, "School", BLUE, GREEN, 30);
    let btn_sports = TextButton::new(550.0, 650.0, 200.0, 60.0, "Sports", BLUE, GREEN, 30);
    let btn_games = TextButton::new(800.0, 650.0, 200.0, 60.0, "Games", BLUE, GREEN, 30);
    let btn_exit = TextButton::new(1050.0, 650.0, 200.0, 60.0, "Exit", BLUE, GREEN, 30);
    let mut lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    let tm = TextureManager::new();
    let all_assets = ["assets/me.png", "assets/bhs.png", "assets/wow.png", "assets/wrestle.png"];
    tm.preload_with_loading_screen(&all_assets, None, None).await;
    let mut img_out = StillImage::new(
        "assets/me.png",
        300.0, // width
        300.0, // height
        700.0, // x position
        60.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    loop {
        clear_background(WHITE);
       //draw_grid(50.0, BROWN);
        lbl_out.draw();
        img_out.draw();
        if btn_name.click() {
            lbl_out.set_text("Leo Smilovici");
            img_out.set_preload(tm.get_preload("assets/me.png").unwrap());
        }

        if btn_school.click() {
            lbl_out.set_text("Bowmanville High");
            img_out.set_preload(tm.get_preload("assets/bhs.png").unwrap());
        }

        if btn_sports.click() {
            lbl_out.set_text("I wrestle and play rugby.");
            img_out.set_preload(tm.get_preload("assets/wrestle.png").unwrap());
        }

        if btn_games.click() {
            lbl_out.set_text("I play extraction shooter games and MMOs.");
            img_out.set_preload(tm.get_preload("assets/wow.png").unwrap());
        }

        if btn_exit.click() {
            break;
        }
        next_frame().await;
    }
}
