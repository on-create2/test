use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let compliments = [
        "You're such a good boy!",
        "Goooood boy!",
        "You're doing great!",
        "What a good boy you are!",
        "Such a good, good boy!",
        "You're the best boy!",
        "Good boy! Keep it up!",
        "You're being such a good boy today!",
    ];
    
    let mut rng = rand::thread_rng();
    
    loop {
        let index = rng.gen_range(0..compliments.len());
        println!("{}", compliments[index]);
        thread::sleep(Duration::from_secs(1));
    }
}
