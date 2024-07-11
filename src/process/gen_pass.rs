use rand::Rng;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*";

pub fn gen_password(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut password = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    let mut source = Vec::new();
    if uppercase {
        source.extend_from_slice(UPPER);
        password.push(UPPER[rng.gen_range(0..UPPER.len())]);
    }
    if lowercase {
        source.extend_from_slice(LOWER);
        password.push(LOWER[rng.gen_range(0..LOWER.len())]);
    }
    if number {
        source.extend_from_slice(NUMBER);
        password.push(NUMBER[rng.gen_range(0..NUMBER.len())]);
    }
    if symbol {
        source.extend_from_slice(SYMBOL);
        password.push(SYMBOL[rng.gen_range(0..SYMBOL.len())]);
    }
    for _ in 0..(length - password.len()) {
        let idx = rng.gen_range(0..source.len());
        password.push(source[idx]);
    }
    let pass = String::from_utf8(password)?;
    println!("{}", pass);
    eprintln!("Password strength:{}", zxcvbn(&pass, &[]).score());
    Ok(())
}
