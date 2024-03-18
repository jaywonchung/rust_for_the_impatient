/// Step 2 of the Rust for the Impatient tutorial.
///
/// First, there can be only one mutable reference to a variable in a scope.
/// So, the `paper_ref` variable and the inplace `&mut paper` reference
/// cannot be alive at the same time.
///
/// Variables that are not `Copy` are moved when passed to a function.
/// This means that the variable is no longer available in the calling scope.
/// To avoid this, we can pass a reference to the variable instead.

struct Paper {
    id: u32,
    title: String,
    reviews: Vec<Review>,
}

struct Review {
    score: u8,
}

fn add_review(mut paper: Paper, score: u8) {
    paper.reviews.push(Review { score });
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

    // let paper_ref = &mut paper;
    // add_review(&mut paper, 4);
    // add_review(paper_ref, 2);
    add_review(paper, 2);
    add_review(paper, 4);

    // print_decision(paper);
}
