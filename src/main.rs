use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
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
    let mut bytes = vec![];
    say_hello(&mut bytes);
    assert_eq!(bytes, b"hello, world\n");

    let _ = min(5, 4);
}
