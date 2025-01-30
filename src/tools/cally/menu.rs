use once_cell::sync::Lazy;

pub struct Menu {}

impl Menu {
    pub fn onInit(&self, width: i32, height: i32) {}
}

pub static theMenu: Lazy<Menu> = Lazy::new(|| Menu {});
