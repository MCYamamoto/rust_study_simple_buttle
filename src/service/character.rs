// ベースキャラクタ
pub struct CharacterBase {
    name: String,
    hp: i16,
    atk: i16,
    def: i16,
}

pub trait Character {
    fn as_foo(&self) -> &CharacterBase;
    fn as_mut_foo(&mut self) -> &mut CharacterBase;
    fn get_name(&self) -> &String {
        &self.as_foo().name
    }
    fn get_hp(&self) -> i16 {
        self.as_foo().hp
    }
    fn get_atk(&self) -> i16 {
        self.as_foo().atk
    }
    fn get_def(&self) -> i16 {
        self.as_foo().def
    }
    fn do_dmg(&mut self, atk: i16) {
        let dmg = atk - self.as_foo().def;
        if dmg > 0 {
            self.as_mut_foo().hp -= dmg;
        }
    }
}

// 味方キャラクタ
pub struct Player {
    base: CharacterBase,
}
impl Player {
    pub fn new(name: String, hp: String, atk: String, def: String) -> Self {
        Self {
            base: CharacterBase {
                name: name,
                hp: hp.parse().unwrap(),
                atk: atk.parse().unwrap(),
                def: def.parse().unwrap(),
            },
        }
    }
}
impl Character for Player {
    fn as_foo(&self) -> &CharacterBase {
        &self.base
    }
    fn as_mut_foo(&mut self) -> &mut CharacterBase {
        &mut self.base
    }
}

// 敵キャラクタ
pub struct Enemy {
    base: CharacterBase,
}
impl Enemy {
    pub fn new(name: String, hp: String, atk: String, def: String) -> Self {
        Self {
            base: CharacterBase {
                name: name,
                hp: hp.parse().unwrap(),
                atk: atk.parse().unwrap(),
                def: def.parse().unwrap(),
            },
        }
    }
}
impl Character for Enemy {
    fn as_foo(&self) -> &CharacterBase {
        &self.base
    }
    fn as_mut_foo(&mut self) -> &mut CharacterBase {
        &mut self.base
    }
}

// キャラクタ返却するファクトリ
pub struct CharacterFactory;
impl CharacterFactory {
    pub fn create_character(
        char_type: &str,
        name: String,
        hp: String,
        atk: String,
        def: String
    ) -> Box<dyn Character> {
        match char_type {
            "1" => Box::new(Player::new(name, hp, atk, def)),
            "2" => Box::new(Enemy::new(name, hp, atk, def)),
            _ => panic!("タイプ不正"),
        }
    }
}