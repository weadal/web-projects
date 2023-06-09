use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    structs::ecs::*,
    structs::util::*,
    user_consts::{coat_size, color, *},
};

pub fn draw_loop(rx: Receiver<DrawMap>) {
    let mut loop_count: u32 = 0;
    let mut cache_drawmap: DrawMap = DrawMap::new();
    cache_drawmap.scroll_message = vec![];
    let mut line_buffer: String;
    let mut next_frame_time: Instant;
    let mut sleep_duration: Duration;
    let mut sleep_duration_sum: u128 = 0;

    loop {
        next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        //frame_time += Duration::from_nanos(8_333_334);

        match rx.try_recv() {
            Ok(mut recieve_drawmap) => {
                //送信されてきた中から一番新しい要素を抽出
                //Errを返すまで(Okが帰ってくる限り)Whileでループ
                while let Ok(value) = rx.try_recv() {
                    recieve_drawmap = Some(value).unwrap(); //Resultが返ってくるので(また、Errはありえない――Errだったらwhileの条件で弾かれる――ので)unwrapで中身を出してdrawmapに上書き
                }

                loop_count += 1;
                println!("\x1b[1;1H"); //カーソルを1行目1文字目に

                //キャッシュと比較して差分だけ抽出する
                //差分だけ描画する

                for i in 0..recieve_drawmap.string_map.len() {
                    //変更がなかったらループ継続
                    if cache_drawmap.string_map[i] == recieve_drawmap.string_map[i] {
                        continue;
                    }

                    //1行分のstringを作る処理
                    line_buffer = String::from("");

                    for str in recieve_drawmap.string_map[i].iter() {
                        let word_buffer: String = create_color_word(&str, color::WHITE);

                        line_buffer += &word_buffer;
                    }

                    print!("\x1b[?25l"); //カーソル非表示
                    println!("\x1b[{0};1H{1}", i + 1, line_buffer); //指定された行に出力
                }

                //下部メッセージ描画
                for (num, value) in recieve_drawmap.scroll_message.iter().enumerate() {
                    if cache_drawmap.scroll_message == recieve_drawmap.scroll_message {
                        break;
                    }

                    //指定行の行頭に移動し、行をクリア
                    print!("\x1b[{};1H\x1b[2K", coat_size::Y + num as i32 + 1);
                    println!("{}", value);
                }

                //固定メッセージ描画
                for (num, value) in recieve_drawmap.static_message.iter().enumerate() {
                    if cache_drawmap.static_message == recieve_drawmap.static_message {
                        break;
                    }

                    //指定行の行頭に移動し、行をクリア
                    print!(
                        "\x1b[{};1H\x1b[2K",
                        coat_size::Y + MAX_SCROLL_MESSAGE as i32 + num as i32 + 2
                    );
                    println!("{}", value);
                }

                //処理後キャッシュ更新
                cache_drawmap = recieve_drawmap;

                sleep_duration = next_frame_time.duration_since(Instant::now());
                sleep(sleep_duration);

                sleep_duration_sum += sleep_duration.as_micros();
            }
            Err(TryRecvError::Empty) => {
                continue;
            }
            Err(TryRecvError::Disconnected) => break,
        }

        //loop_count = 0;
    }
}

fn create_color_word(str: &str, color: i32) -> String {
    let word_buffer: String = format!("\x1b[{0}m{1}\x1b[m", color, str);

    word_buffer
}

pub struct DrawMap {
    pub string_map: Vec<Vec<String>>,
    pub scroll_message: Vec<String>,
    pub static_message: Vec<String>,
}
impl DrawMap {
    pub fn new() -> DrawMap {
        let mut map = DrawMap {
            string_map: vec![vec![String::from("  ")]],
            scroll_message: vec![],
            static_message: vec![],
        };

        for row_y in 0..coat_size::Y {
            if row_y > 0 {
                map.string_map.push(vec![String::from("  ")])
            }

            for _ in 1..coat_size::X {
                map.string_map[row_y as usize].push(String::from("  "));
            }
        }

        for _ in 0..MAX_SCROLL_MESSAGE {
            map.scroll_message.push(String::from(""));
        }

        for _ in 0..MAX_STATIC_MESSAGE {
            map.static_message.push(String::from(""));
        }
        map
    }
}

pub struct DrawAddress {
    pub x: i32,
    pub y: i32,
}
impl DrawAddress {
    pub fn from_position(pos: &Vector2) -> Option<DrawAddress> {
        let address = DrawAddress {
            x: pos.x.round() as i32,
            y: pos.y.round() as i32,
        };

        if address.x >= 0 && address.x <= coat_size::X - 1 {
            if address.y >= 0 && address.y <= coat_size::Y - 1 {
                return Some(address);
            }
        }

        None
    }
}
