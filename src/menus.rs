use macroquad::ui::widgets::*;

pub struct Ui<'a> {
    pub text: &'a str,
    pub button_text: &'a str,
}

pub fn start_menu() -> Ui<'static>{
    Ui {
        text: "Exodus",
        button_text: "PLAY",
    }
    
}