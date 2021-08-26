mod tokenize;

#[cfg(test)]
mod ported_tests;

#[allow(unused_must_use)]
fn main() {
    let tokens = tokenize::tokenize("1.24 +43");

    dbg!(tokens);
}
