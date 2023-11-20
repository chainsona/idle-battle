pub fn xorshift64_enemy(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn xorshift64_hero(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 17;
    x ^= x >> 13;
    x ^= x << 7;
    x
}
