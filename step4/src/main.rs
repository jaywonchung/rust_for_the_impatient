/// Step 4 of the Rust for the Impatient tutorial.
///
/// Papers typically have more than one reviewers. We're going to simulate
/// this by having multiple threads generate random scores for a paper.
///
/// The only change we made from step3/src/main.rs is calling `s.spawn` in a
/// loop, but now, hell, this doesn't compile. Why?
///
/// Well, it *shouldn't* compile! The reason is that multiple threads are
/// concurrently mutating the same variable, `paper` (precisely `paper.reviews`),
/// which is a classic data race.
///
/// Rust is able to catch this at compile time because of its ownership system.
/// To reiterate, in Rust, there can only be at most one mutable reference to a
/// single variable at any given time. And this is fine on the first iteration
/// of the thread spawn loop, but on the second iteration, the variable `paper`
/// is already borrowed by the first thread, so the second thread can't borrow
/// it mutably. This is caught by the Rust compiler.
///
/// How do we fix this? Locks. We'll update `Paper` like this:
/// ```rust
/// use std::sync::Mutex;
///
/// struct Paper {
///     id: u32,
///     title: String,
///     reviews: Mutex<Vec<u8>>,
/// }
/// ```
///
/// Then, when we spawn the thread:
/// ```rust
/// s.spawn(|| {
///     let mut rng = rand::rng();
///     let score = rng.random_range(1..6);
///     let mut reviews = paper.reviews.lock().unwrap();
///     reviews.push(score);
/// });
/// ```
///
/// Note that you'll need to update `print_decision` to handle the new type of
/// `reviews`.

use rand::Rng;

struct Paper {
    id: u32,
    title: String,
    reviews: Vec<u8>,
}

fn print_decision(paper: Paper) {
    let total_score: u8 = paper.reviews.iter().sum();
    let accept_threshold = paper.reviews.len() as u8 * 3;

    if total_score > accept_threshold {
        println!("Accepted submission #{} \"{}\"", paper.id, paper.title);
    } else {
        println!("Rejected submission #{} \"{}\"", paper.id, paper.title);
    }
}

fn main() {
    let mut paper = Paper {
        id: 574,
        title: String::from("Perseus: Removing Energy Bloat ..."),
        reviews: Vec::new(),
    };

    let num_reivewers = 3;
    std::thread::scope(|s| {
        for _ in 0..num_reivewers {
            s.spawn(|| {
                let mut rng = rand::rng();
                let score = rng.random_range(1..6);
                paper.reviews.push(score);
            });
        }
    });

    print_decision(paper);
}
