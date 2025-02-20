/// Step 2 of the Rust for the Impatient tutorial.
///
/// Let's start actually building MiniCRP, a stripped down version of HotCRP.
///
/// Why doesn't the second call to `add_review` compile?
/// It's because of Rust's ownership rule. Variables passed into functions
/// as is are "moved" into the function, and at the end of the function where
/// the variable goes out of scope, it is deallocated ("dropped" in Rust-speak).
/// Therefore, after the first call to `add_review` returns, the variable
/// `paper` is no longer accesible; accessing it is a compile error.
///
/// Then how are anyone supposed to write any useful code with this language?
/// That's why there are references. References are pointers to variables that
/// do not take ownership of the variable. They are denoted by the `&` symbol.
///
/// There are two types of references: immutable references (`&paper`) and
/// mutable references (`&mut paper`). The former allows you to *only* read the
/// value of the variable, while the latter allows you to modify the value.
///
/// In Rust, multiple immutable references can exist at the same time, but
/// *only one* mutable reference can exist at a time. We'll soon see how this
/// rule is used to prevent data races in multi-threaded programs.
///
/// So, how do we fix this? We pass a mutable reference to `add_review`:
/// ```rust
/// fn add_review(paper: &mut Paper, score: u8) {
///    paper.reviews.push(score);
/// }
///
/// fn main() {
///     // Instantiate `paper`.
///
///     let paper_ref = &mut paper;
///     add_review(paper_ref, 2);
///     add_review(paper_ref, 4);
///
///     print_decision(paper);
/// }
/// ```
///
/// Hey, but by the way, the following doesn't compile. Can you guess why?
/// ```rust
/// fn main() {
///     // Instantiate `paper`.
///
///     let paper_ref = &mut paper;
///     add_review(&mut paper, 2);
///     add_review(paper_ref, 4);
///
///     print_decision(paper);
/// }
/// ```

struct Paper {
    id: u32,
    title: String,
    reviews: Vec<u8>,
}

fn add_review(mut paper: Paper, score: u8) {
    paper.reviews.push(score);
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

    add_review(paper, 2);
    add_review(paper, 4);

    // print_decision(paper);
}
