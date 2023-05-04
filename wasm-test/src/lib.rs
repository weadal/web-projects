mod utils;
use core::fmt;
use rand::Rng;

use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console::{self, log};

//web_sysが提供するブラウザのコンソールにログを表示させるためのマクロをいじってprintln!風に書けるようにしている……らしい
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//多分メモリ周りの最適化用のおまじない 消しても問題なかったけどとりあえず付けとく
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen] //rustとjsをつなぐ最重要アトリビュート
#[repr(u8)]
//メモリ構造をu8(バイト)に限定するアトリビュート wasmのメモリ構造的にこうするのが効率良いっぽい
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next: Vec<Cell>,
    delta: Vec<Cell>,
}

impl Universe {
    //列と行から線形になった配列の添字を取得する
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    //自身に隣接する生きているセルの数をカウントする
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        //上下左右のセルを力技で定義する　ここで上下左右の端に対する例外処理を埋め込んでおく
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        //8方向の隣接セルを力技で取得していく
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        count
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    //配列で渡された複数の番地のセルを一括でAliveにするメソッド
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn generate_init_cells(width: u32, height: u32) -> Vec<Cell> {
        let cells = (0..width * height)
            .map(|_i| {
                if rand::thread_rng().gen_bool(0.5) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        cells
    }
}

//implを分けてjsに公開するメソッドだけwasm_bindgenアトリビュートをつける
//特定のメソッドだけアトリビュートをつけるってことは出来ないっぽい
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.delta = self.delta.iter().map(|_i| Cell::Dead).collect();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    //セルが生きていて、隣接セルに生きているセルが1つ以下ならそのセルは人口不足により死ぬ
                    (Cell::Alive, x) if x < 2 => {
                        self.delta[index] = Cell::Alive;
                        Cell::Dead
                    }
                    //セルが生きていて、隣接セルに生きているセルが2個か3個の場合はそのセルは行き続ける
                    (Cell::Alive, 2) | (Cell::Alive, 3) => {
                        self.delta[index] = Cell::Dead;
                        Cell::Alive
                    }
                    //セルが生きていて、隣接セルに生きているセルが4つ以上ならそのセルは人口過多により死ぬ
                    (Cell::Alive, x) if x > 3 => {
                        self.delta[index] = Cell::Alive;
                        Cell::Dead
                    }
                    //死んでいるセルは、隣接セルが3つの場合入植により生き返る
                    (Cell::Dead, 3) => {
                        self.delta[index] = Cell::Alive;
                        Cell::Alive
                    }
                    //それ以外の場合は以前の状態を維持する(入力されたcellの状態を変数名otherwiseに束縛し、それをそのままcellに適用する)
                    (otherwise, _) => {
                        self.delta[index] = Cell::Dead;
                        otherwise
                    }
                };

                self.next[index] = next_cell;
            }
        }

        self.cells.swap_with_slice(&mut self.next);
    }

    pub fn new() -> Universe {
        //落ちたときにデバッグメッセージがコンソールに表示されるようにするユーティリティ これgame-of-lifeのテンプレートについてきたのかな…
        //多分console_error_panic_hookクレートに入ってるっぽい.tomlにそれっぽい記述があった
        utils::set_panic_hook();

        let width = 512;
        let height = 256;

        let cells = Universe::generate_init_cells(width, height);
        let next = cells.clone();
        let delta = vec![Cell::Dead; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
            next,
            delta,
        }
    }

    pub fn reset(&mut self) {
        self.cells = Universe::generate_init_cells(self.width, self.height);
        self.delta = vec![Cell::Dead; (self.width * self.height) as usize];
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    //Cellの不変な(*staticな)ポインタを出力する
    //この場合の*staticは、ポインタそれ自体の不変性ではなく、ポインタが指し示す対象の不変性を保証する
    //ちなみに可変ポインタは*mut Hogeなので*をつけることがポインタを意味すると考えて良さそう　参照外しの*もポインタの参照先を扱う宣言としてみれば割りと一貫してる
    pub fn cells(&self) -> *const Cell {
        //cellsの先頭のポインタを得る
        self.cells.as_ptr()
    }

    pub fn delta(&self) -> *const Cell {
        self.delta.as_ptr()
    }

    //widthを設定し、セルを全て初期化(Deadに)する
    pub fn set_width(&mut self, width: u32) {
        self.width = width;

        //self.cellsを新しく設定する
        //0から新しいwidth * もともとのheightで得られる全セル数分のRangeを作って、それをすべてCell::Deadにした配列を作成する
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    //heightを設定し、セルを全て初期化(Deadに)する
    pub fn set_height(&mut self, height: u32) {
        self.height = height;

        //self.cellsを新しく設定する
        //0から新しいheight * もともとのwidthで得られる全セル数分のRangeを作って、それをすべてCell::Deadにした配列を作成する
        self.cells = (0..height * self.width).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

//文字列を出力するときのDisprayトレイトをいじる 構造体のUniverse型はフツーに文字列として出力するのは無理なので自前で用意する必要がある
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //vecをas_sliceでスライス型にキャストして、self.width(幅)でひとまとまりにしたチャンクとして切り出す
        //つまりlineには一行分のcell要素が入っている
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { "□" } else { "■" };

                //formatter(多分フォーマット文)にシンボルを書き込み、エラー起こった場合"?"で早期リターン
                write!(f, "{}", symbol)?;
            }
            //formatterに改行を書き込み
            write!(f, "\n")?;
        }

        Ok(())
    }
}

//web_sysによってコンソールに生成された時間とDropした時間を表示するためのトークン
//コンストラクタでweb_sysのタイマースタート的なメソッドを走らせ、Drop時にタイマーストップとそれまでにかかった時間をコンソールに表示する
pub struct Timer<'a> {
    name: &'a str,
}
impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
