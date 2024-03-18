/// Step 4 of the Rust for the Impatient tutorial.
///
/// Multithreading and the type system.

use std::sync::{Arc, Mutex};
use rand::Rng;

struct Paper {
    id: u32,
    title: String,
    reviews: Arc<Mutex<Vec<Review>>>,
}

struct Review {
    score: u8,
}

fn print_decision(paper: Paper) {
    let reviews = paper.reviews.lock().unwrap();
    let mut total_score: u8 = 0;
    for (i, review) in reviews.iter().enumerate() {
        total_score += review.score;
        println!("Review #{}: {}", i, review.score);
    }
    let average = total_score as f32 / reviews.len() as f32;
    drop(reviews);

    if average > 3.0 {
        println!("\x1b[32mAccepted\x1b[0m submission #{} \"{}\"", paper.id, paper.title);
    } else {
        println!("\x1b[31mRejected\x1b[0m submission #{} \"{}\"", paper.id, paper.title);
    }
}

fn main() {
    let paper = Paper {
        id: 574,
        title: String::from("Perseus: Removing Energy Bloat ..."),
        reviews: Arc::new(Mutex::new(Vec::new())),
    };

    let num_reviews = 3;
    std::thread::scope(|s| {
        for _ in 0..num_reviews {
            let reviews_ref = Arc::clone(&paper.reviews);
            s.spawn(move || {
                let mut rng = rand::thread_rng();
                let score = rng.gen_range(1..6);
                match reviews_ref.lock() {
                    Ok(mut guard) => {
                        guard.push(Review { score });
                    },
                    Err(e) => {
                        println!("Cannot add review: {}", e);
                    }
                }
            });
        }
    });

    print_decision(paper);
}
