use std::env;
mod service;
mod logger;

fn main() {
    // コマンドライン引数取得
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // ファイル名取得
    let filename: String = get_file_name(args);
    // println!("{:?}", filename);

    let mut service_obj = service::Service::new();
    // ファイルを開いてオブジェクト生成
    service_obj.create_character_obj(filename);

    // 実行
    let result = service_obj.run();
    if result == true {
        logger::Logger::log_println("Playerの勝利!!");
    } else {
        logger::Logger::log_println("敵の勝利!!");
    }
    return ();
}

// ファイル名取得
fn get_file_name(args: Vec<String>) -> String {
    match args.get(1) {
        Some(filename) => filename.to_string(),
        None => panic!("ファイル名を指定してください"),
    }
}