use crate::objects::Object;
use crate::states;
use ggez;
use ggez::audio;
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

pub struct Reg {
    pub sounds: HashMap<String, audio::Source>,
    pub fonts: HashMap<String, ggez::graphics::Font>,
    pub images: HashMap<String, ggez::graphics::Image>,
    pub texts: HashMap<String, ggez::graphics::Text>,
    pub key_status: HashMap<KeyCode, bool>,
    pub objects: HashMap<String, Box<dyn Object>>,
    pub f32_values: HashMap<String, f32>,
}

impl Reg {
    pub fn new() -> Reg {
        Reg {
            sounds: HashMap::<String, audio::Source>::new(),
            fonts: HashMap::<String, ggez::graphics::Font>::new(),
            images: HashMap::<String, ggez::graphics::Image>::new(),
            texts: HashMap::<String, ggez::graphics::Text>::new(),
            key_status: HashMap::<KeyCode, bool>::new(),
            objects: HashMap::<String, Box<dyn Object>>::new(),
            f32_values: HashMap::<String, f32>::new(),
        }
    }

    // 방금 전까지는 안 눌린 것인지 확인
    // 이후에 눌린 것이라면 해당 값은 true이며
    // 이제는 해당하는 값에 눌림효과를 넣음
    pub fn just_pressed(&mut self, key: KeyCode) -> bool {
        let status = self.key_status.entry(key).or_insert(false);

        if *status == false {
            *status = true;
            true
        } else {
            false
        }
    }

    // 키가 안눌리면 release하기
    pub fn just_released(&mut self, key: KeyCode) {
        let status = self.key_status.entry(key).or_insert(false);

        *status = false;
    }

    pub fn add_sound(&mut self, key: String, sound: audio::Source) {
        self.sounds.insert(key, sound);
    }

    pub fn get_sound_mut(&mut self, key: String) -> Option<&mut audio::Source> {
        self.sounds.get_mut(&key)
    }

    pub fn add_object(&mut self, key: String, object: Box<dyn Object>) {
        self.objects.insert(key, object);
    }

    pub fn get_object_mut(&mut self, key: String) -> Option<&mut Box<dyn Object>> {
        self.objects.get_mut(&key)
    }

    pub fn add_font(&mut self, key: String, font: ggez::graphics::Font) {
        self.fonts.insert(key, font);
    }

    pub fn get_font(&self, key: String) -> Option<&ggez::graphics::Font> {
        self.fonts.get(&key)
    }

    pub fn add_text(&mut self, key: String, text: ggez::graphics::Text) {
        self.texts.insert(key, text);
    }

    pub fn get_text(&self, key: String) -> Option<&ggez::graphics::Text> {
        self.texts.get(&key)
    }

    pub fn add_f32(&mut self, key: String, f32_: f32) {
        self.f32_values.insert(key, f32_);
    }

    pub fn get_f32(&self, key: String) -> Option<&f32> {
        self.f32_values.get(&key)
    }

    pub fn add_image(&mut self, key: String, image: ggez::graphics::Image) {
        self.images.insert(key, image);
    }

    pub fn get_image(&self, key: String) -> Option<&ggez::graphics::Image> {
        self.images.get(&key)
    }

    pub fn clear_sound(&mut self) {
        self.sounds.clear();
    }

    pub fn clear_text(&mut self) {
        self.texts.clear();
    }

    pub fn clear_font(&mut self) {
        self.fonts.clear();
    }

    pub fn clear_image(&mut self) {
        self.images.clear();
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }
}
