use crate::objects::Object;
use crate::quad::Quad;
use ggez;
use ggez::audio;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use std::collections::HashMap;
use std::path::Path;

pub struct Reg {
    pub sounds: HashMap<String, audio::Source>,
    pub fonts: HashMap<String, ggez::graphics::Font>,
    pub images: HashMap<String, ggez::graphics::Image>,
    pub texts: HashMap<String, ggez::graphics::Text>,
    pub key_status: HashMap<KeyCode, bool>,
    pub objects: HashMap<String, Box<dyn Object>>,
    pub f32_values: HashMap<String, f32>,
    pub i32_values: HashMap<String, i32>,
    pub sprites: Option<Quad>,
    pub hearts: Option<Quad>,
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
            i32_values: HashMap::<String, i32>::new(),
            sprites: None,
            hearts: None,
        }
    }

    // sprites 초기화
    pub fn init_sprite(&mut self, ctx: &mut Context, path: &Path) {
        self.sprites = Some(Quad::new(ctx, path));
    }

    pub fn init_heart(&mut self, ctx: &mut Context, path: &Path) {
        self.hearts = Some(Quad::new(ctx, path));
    }

    // Heart 생성하기
    pub fn register_heart(&mut self, key: i32, x: f32, y: f32, w: f32, h: f32) {
        match &mut self.hearts {
            Some(sp) => {
                (*sp).add_sprite(key, x, y, w, h);
                ()
            }
            None => (),
        }
    }

    // sprites를 등록하기
    pub fn register_sprite(&mut self, key: i32, x: f32, y: f32, w: f32, h: f32) {
        match &mut self.sprites {
            Some(sp) => {
                (*sp).add_sprite(key, x, y, w, h);
                ()
            }
            None => (),
        }
    }

    // sprite drawing
    pub fn draw_sprite(&mut self, ctx: &mut Context, key: i32, x: f32, y: f32) {
        self.sprites.as_mut().unwrap().draw_sprite(ctx, key, x, y)
    }

    // heart drawing
    pub fn draw_heart(&mut self, ctx: &mut Context, key: i32, x: f32, y: f32) {
        self.hearts.as_mut().unwrap().draw_sprite(ctx, key, x, y)
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
        let fvalue = self.f32_values.entry(key).or_insert(0.);

        *fvalue = f32_;
    }

    pub fn get_f32(&self, key: String) -> f32 {
        *self.f32_values.get(&key).unwrap()
    }

    pub fn get_f32_mut(&mut self, key: String) -> &mut f32 {
        self.f32_values.get_mut(&key).unwrap()
    }

    pub fn add_i32(&mut self, key: String, i32_: i32) {
        let ivalue = self.i32_values.entry(key).or_insert(0);

        *ivalue = i32_;
    }

    pub fn get_i32(&self, key: String) -> i32 {
        *self.i32_values.get(&key).unwrap()
    }

    pub fn get_i32_mut(&mut self, key: String) -> &mut i32 {
        self.i32_values.get_mut(&key).unwrap()
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

    pub fn clear_f32_values(&mut self) {
        self.f32_values.clear();
    }

    pub fn clear_i32_values(&mut self) {
        self.i32_values.clear();
    }

    pub fn clear_all(&mut self) {
        self.clear_sound();
        self.clear_text();
        self.clear_font();
        self.clear_image();
        self.clear_objects();
        self.clear_f32_values();
        self.clear_i32_values();
    }
}
