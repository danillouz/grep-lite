/// Search text for a pattern.
/// Based on this grep specification: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html
fn main() {
    let quote = "It wasn’t always so clear, but the Rust programming language
is fundamentally about empowerment: no matter what kind of code you are
writing now, Rust empowers you to reach farther, to program with confidence
in a wider variety of domains than you did before.";
    let query = "Rust";
    let ctx_lines = 2;
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (i, line) in quote.lines().enumerate() {
        if line.contains(query) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        println!("No match for pattern: \"{}\"", query);
        return;
    }

    for (i, line) in quote.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, String::from(line));
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
