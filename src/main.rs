use regex::RegexSet;
use std::io;

fn main() -> io::Result<()> {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;
    let input = buff.trim();

    let pascal_case = r"^([A-Z][a-z]+)+$"; // PascalCase
    let camel_case = r"^[a-z]+([A-Z][a-z]+)*$"; // camelCase
    let snake_case = r"^([a-z]+(_[a-z]+)*)$"; // snake_case;
    let screaming_snake_case = r"^([A-Z]+(_[A-Z]+)*)$"; // SCREAMING_SNAKE_CASE
                                                        // kebab-case
                                                        // Camel_Snake_Case
                                                        // HTTP-Header-Case

    let set = RegexSet::new(&[pascal_case, camel_case, snake_case, screaming_snake_case]).unwrap();
    let matches: Vec<_> = set.matches(input).into_iter().collect();

    // TODO
    // - accept target case as cmd arg
    // - detect input case
    // - disect input into words
    // - reassemble words into target case
    // - output target case to stdout

    println!("{:#?} matches {:#?}", input, matches);
    Ok(())
}
