use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KeyScan {
    key_status: HashMap<KeyCode, bool>,
}

impl KeyScan {
    pub fn new() -> KeyScan {
        let key_status = HashMap::<KeyCode, bool>::new();

        KeyScan { key_status }
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
}
