/// Step 3 of the Rust for the Impatient tutorial.
///
/// Multithreading and the type system.

use rand::Rng;

struct Paper {
    id: u32,
    title: String,
    reviews: Vec<Review>,
}

struct Review {
    score: u8,
}

fn print_decision(paper: Paper) {
    let mut total_score: u8 = 0;
    for (i, review) in paper.reviews.iter().enumerate() {
        total_score += review.score;
        println!("Review #{}: {}", i, review.score);
    }
    let average = total_score as f32 / paper.reviews.len() as f32;

    if average > 3.0 {
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

    let num_reviews = 3;
    std::thread::scope(|s| {
        for _ in 0..num_reviews {
            s.spawn(|| {
                let mut rng = rand::thread_rng();
                let score = rng.gen_range(1..6);
                paper.reviews.push(Review { score });
            });
        }
    });

    print_decision(paper);
}
