use macroquad::ui::widgets::*;

pub struct Ui<'a> {
    pub text: &'a str,
    pub button: Button<'a>
}

pub fn start_menu() -> Ui<'static>{
    Ui {
        text: "Exodus",
        button: Button::new("Text")
    }
    
}