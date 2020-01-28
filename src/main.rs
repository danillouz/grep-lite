/// Search text for a pattern.
/// Based on this grep specification: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html
fn main() {
    let quote = "It wasn’t always so clear, but the Rust programming language
is fundamentally about empowerment: no matter what kind of code you are
writing now, Rust empowers you to reach farther, to program with confidence
in a wider variety of domains than you did before.";
    let query = "Rust";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(query) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
