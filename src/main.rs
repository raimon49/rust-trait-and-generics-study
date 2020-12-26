use std::io::{Write, Result};
use std::ops::Range;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello, world\n")?;
    out.flush()
}

// 型パラメータWはWriteトレイトを実装した何らかの型なら引数で受け付けられる
fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello, world\n")?;
    out.flush()
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

struct Canvas {
    width: i32,
    height: i32
}

// trait構文によるメソッドシグニチャの宣言
trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

// impl Visible for Broomブロックの中ではtrait Visibleで宣言されたメソッドしか実装できないため
// fn draw()の中から使いたいメソッドは別途impl Broomブロックの中に定義する
impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            let _ = canvas;
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.width < y && self.height < x
    }
}

// 何もしないWriteの実装としてSinkを定義
pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // バッファにすべて書き出したことにする
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    // fn write_all() はWriteトレイトにデフォルト実装を持っているため、定義しなくてもよい
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

// 任意の組み込み型に対してもトレイトを使ってメソッドが追加できる
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

// トレイトVisibleのサブトレイト
// Creatureトレイトを実装するすべての型ではVisibleで定義されたメソッドも実装しなければならない
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> i32;
}

fn main() {
    // Vec<u8>はstd::io::Writeを実装している
    // traitメソッドはスコープ内で見えている（useされている）必要があり、見えないと呼べずエラーになる
    let mut bytes = vec![];
    let _ = say_hello(&mut bytes);
    assert_eq!(bytes, b"hello, world\n");

    let mut buf: Vec<u8> = vec![];
    let _ = buf.write_all(b"hello");

    let _ = min(5, 4);

    // 変数のサイズがコンパイル時に決められないためエラーになる
    // let _writer: Write = buf;
    // トレイト型への参照（trait object）であればC#やJavaのように変数へ代入可能
    let _writer: &mut dyn Write = &mut buf;

    // ジェネリック関数の呼び出し
    let _ = say_hello_generic(&mut bytes);

    assert_eq!('$'.is_emoji(), false);
}
