use macroquad::ui::widgets::*;

pub struct Ui<'a> {
    text: &'a str,
    button: Button<'a>
}

pub fn start_menu() -> Ui<'static>{
    Ui {
        text: "Game",
        button: Button::new("Text")
    }
    
}