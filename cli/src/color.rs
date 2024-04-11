use concolor_clap::{Color, ColorChoice};

pub trait ColorInit {
    fn init(&self);
}

impl ColorInit for Color {
    fn init(&self) {
        match self.color {
            ColorChoice::Auto => colored::control::unset_override(),
            ColorChoice::Always => {
                colored::control::set_override(true);
                console::set_colors_enabled(true);
            }
            ColorChoice::Never => {
                colored::control::set_override(false);
                console::set_colors_enabled(false);
            }
        }
    }
}
