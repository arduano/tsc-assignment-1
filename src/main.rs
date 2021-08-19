mod tokenize;

#[cfg(test)]
mod tests;

#[allow(unused_must_use)]
fn main() {
    let tokens = tokenize::tokenize("1.24 +43");

    dbg!(tokens);
}
