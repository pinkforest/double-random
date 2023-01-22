fn main() {
    let mut rng = rand::thread_rng();

    let (inumber, ratchet_seed) = play_transmute::generate_double_random_safe(&mut rng);

    println!("Safe-A: {inumber:?}");
    println!("Safe-B: {ratchet_seed:?}");

    let (inumber, ratchet_seed) = play_transmute::generate_double_random_safe(&mut rng);

    println!("Unsafe-A: {inumber:?}");
    println!("Unsafe-B: {ratchet_seed:?}");
}
