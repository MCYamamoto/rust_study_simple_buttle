use std::fs::File;
use std::io::{ BufReader, BufRead };
mod character;
use crate::logger::Logger;

pub struct Service {
    char_list: [Vec<Box<dyn character::Character>>; 2],
}

impl Service {
    pub fn new() -> Self {
        Self { char_list: [Vec::new(), Vec::new()] }
    }
    // ファイルを開いてオブジェクト生成
    pub fn create_character_obj(&mut self, filename: String) {
        let f = File::open(filename).expect("ファイルがありません");
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            // コメント業は飛ばす
            if line.len() == 0 {
                Logger::log_println("空白行なので飛ばす");
                continue;
            }
            if line.chars().nth(0).unwrap() == '#' {
                // println!("コメント行なので飛ばす");
                continue;
            }
            let value: Vec<&str> = line.split(',').collect();
            // println!("{:?}", value);
            // 先頭文字列を数値に変換し、Player領域、敵領域のどっちに入れるか分岐
            let index: usize = (value[0].parse::<u64>().unwrap() - 1).try_into().unwrap();
            self.char_list[index].push(
                character::CharacterFactory::create_character(
                    value[0],
                    value[1].to_string(),
                    value[2].to_string(),
                    value[3].to_string(),
                    value[4].to_string()
                )
            );
        }
    }

    // 戦闘開始
    pub fn run(&mut self) -> bool {
        let mut cnt;
        let mut turn = 0;
        loop {
            let atk;
            let def;
            if turn == 0 {
                // Playerターン
                (atk, def) = self.char_list.split_at_mut(1);
            } else {
                // 敵のターン
                (def, atk) = self.char_list.split_at_mut(1);
            }
            for i in atk[0].iter_mut() {
                if i.get_hp() <= 0 {
                    continue;
                }
                cnt = 0;
                for l in def[0].iter_mut() {
                    if l.get_hp() > 0 {
                        print!("{}が", i.get_name());
                        println!("{}に攻撃", l.get_name());
                        let atk = i.get_atk();
                        l.do_dmg(atk);
                        if l.get_hp() <= 0 {
                            println!("{}を倒した", l.get_name());
                        } else {
                            println!("{}の残りHP:{}", l.get_name(), l.get_hp());
                        }
                        cnt += 1;
                        break;
                    }
                }
                if cnt <= 0 {
                    if turn == 0 {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            turn = (turn + 1) % 2;
        }
    }
}