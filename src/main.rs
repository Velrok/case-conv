use std::io;

fn main() -> io::Result<()> {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;
    let input = buff.trim();
    println!("Hello, {}!", input);
    Ok(())
}
