fn main() {
    let context_lines = 1;
    let needle = "oo";
    let haystack = "\
Every face, every shop, 
bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek 
through millions of pages?";

    let mut tags: Vec<usize> = Default::default();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            ctx.push(Vec::with_capacity(2 * context_lines + 1));
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines); //instead of overflow
            let upper_bound = tag + context_lines;
            if (lower_bound <= i) && (i <= upper_bound) {
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
        println!("");
    }
}
