#[derive(Debug)]
pub enum ColorControl {
    AlwaysColor,
    NeverColor,
    UseEnvironment,
}

pub fn set_color_control(control: ColorControl) {
    match control {
        ColorControl::AlwaysColor => always_color(),
        ColorControl::NeverColor => never_color(),
        ColorControl::UseEnvironment => use_envionment(),
    }
}

pub fn never_color() {
    colored::control::set_override(false);
}

pub fn always_color() {
    colored::control::set_override(true);
}

pub fn use_envionment() {
    colored::control::unset_override();
}
