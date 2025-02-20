/// Step 3 of the Rust for the Impatient tutorial.
///
/// Let's get into threading.
///
/// We're going to use `std::thread::scope`, which is a helper that
/// automatically joins all threads spawned inside it. `s.spawn` takes the
/// closure that contains the code to be executed in a separate thread.
///
/// This compiles and works well.
///
/// Additionally, we're using the `rand` crate to generate random numbers.
/// When you run `cargo add rand` in your command line, Cargo automatically
/// adds the new dependency into your `Cargo.toml` file.

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

    std::thread::scope(|s| {
        s.spawn(|| {
            let mut rng = rand::rng();
            let score = rng.random_range(1..6);
            paper.reviews.push(score);
        });
    });

    print_decision(paper);
}
