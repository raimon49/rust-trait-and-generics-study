use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello, world\n")?;
    out.flush()
}

fn main() {
    let mut bytes = vec![];
    say_hello(&mut bytes);
    assert_eq!(bytes, b"hello, world\n");
}
