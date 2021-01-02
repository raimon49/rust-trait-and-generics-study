use std::io::{Write, Result};
use std::ops::Range;
use std::iter::Iterator;
use std::fmt::Debug;

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

trait StringSet {
    // 引数selfを取らないスタティックメソッド（コンストラクタ）
    // サポートしなくて良いようコンパイラに伝えるSizedトレイトにするとトレイトオブジェクトを作ることができる
    fn new() -> Self
        where Self: Sized;
    fn from_slice(strings: &[&str]) -> Self
        where Self: Sized;

    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

// {:?}フォーマット指示子で出力できるようIteratorとDebugを実装していることをwhereで明示
fn dump<I>(iter: I)
    where I: Iterator, I::Item: Debug
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// Iteratorトレイトの関連型ItemがStringであることを明示してもコンパイルを通る
// 関連型（associated type）はIteratorトレイトで宣言されている
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }
fn dumpOther<I>(iter: I)
    where I: Iterator<Item=String>
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// 自分で関連型（associated type）を活用する例
trait Pattern {
    type Match;
    fn search(&self, string: &str) -> Option<Self::Match>;
}

impl Pattern for char {
    type Match = usize;
    fn search(&self, string: &str) -> Option<Self::Match> {
        // マッチした文字列の位置を返す
        Some(10)
    }
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

    // 次の4つのメソッド呼び出しは等価
    // value.method()は修飾メソッド呼び出し
    // <type as Trait>.method()完全修飾（fully qualified）メソッド呼び出し
    assert_eq!("hello".to_string(), str::to_string("hello"));
    assert_eq!(ToString::to_string("hello"), <str as ToString>::to_string("hello"));

    let zero = 0;
    // let _ = zero.abs();
    // 型が指定されていないためコンパイラにはabsメソッドが見付けられない
    let _ = i64::abs(zero);

    use rand::random;
    // Rngトレイトを実装したrandom()で要求される型を推論できないためジェネリック関数で呼ぶ必要がある
    // let _x = random();
    let _y = random::<f64>();  // 0.0 <= x < 1.0の値を生成
    let _z = random::<bool>(); // true or falseを生成
}
