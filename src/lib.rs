//! # Rust Beginner Projects
//! 
//! ## Rundown
//! 
//! This a list of solutions to [beginner-projects](https://github.com/jorgegonzalez/beginner-projects) written in Rust
//! 
//! ### How to Run
//! 
//! To run a solution: compile the crate, and run the corresponding example
//! 
//! For example to run `magic_8_ball.rs`, run
//! ```bash
//! $ cargo build
//! $ cargo run --example magic_8_ball
//! ```
//! 
//! Examples can be found in the `examples` directory
//! 
//! ### Documentation
//! 
//! To view documentation, run `cargo doc --open`
//! 
//! ## Status of Solutions
//! 
//! - [x] 99 Bottles
//! - [x] Magic 8 Ball
//!   - [ ] Bonus
//! - [x] Pythagorean Triples Checker
//! - [x] Rock Paper Scissors Game
//! - [x] Coin Estimator By Weight
//! - [x] Mad Libs Story Maker
//! - [ ] Change Calculator
//! - [ ] Mean, Median, and Mode
//! - [ ] Higher Lower Guessing Game
//! - [ ] Multiplication Table
//! - [ ] Fibonacci Sequence
//! - [ ] Base Jumper
//! - [ ] Hangman Game
//! - [ ] Menu Calculator
//! - [ ] Dice Rolling Simulator
//!   - [ ] Bonus
//! - [ ] Count and Fix Green Eggs and Ham
//! - [ ] What's My Number?
//! - [ ] Factors of a Number
//!   - [ ] Bonus
//! - [ ] Countdown Clock
//! - [ ] Turn Based Pokemon Style Game
//! - [ ] A Variation of 21
//! - [ ] Compare Recent Reddit Karma
//! - [ ] Watch for New TIL Facts
//! - [ ] Random Wikipedia Article
//! - [ ] What's the Weather?
//! - [ ] Sierpinski Triangle
//! - [ ] Two Numbers
//! - [ ] Chickens and Rabbits
//! - [ ] FLAMES Game
//! - [ ] Pomodoro Timer
//! - [ ] Scarne's Dice

extern crate rand;

/// 99 Bottles Solution
pub mod bottles_99;

/// Magic 8 Ball Solution
pub mod magic_8_ball;

/// Pythagorean Triples Checker Solution
pub mod pythagorean_triples_checker;

/// Rock Paper Scissors Solution
pub mod rock_paper_scissors;

/// Coin Estimator Solution
pub mod coin_estimator_by_weight;

/// Mad Libs Story Maker Solution
pub mod mad_libs;

/// Watcher for TIL
pub mod today_i_learnt;
