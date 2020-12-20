use std::io::Write;

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
}
