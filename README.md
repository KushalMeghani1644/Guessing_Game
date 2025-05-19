# 🎯 Rust Number Guessing Game

Welcome to the **Number Guessing Game**, written in Rust!  
This fun and interactive CLI game challenges you to guess a randomly generated number within a user-defined number of attempts.

---

## 📦 Features

- 🔢 Random numbers based on the difficulty level the user chooses:
     1. Easy: (1 to 40)
     2. Medium (1 to 60)
     3. Hard(1 to 100)
- 🎮 User-defined number of attempts with the max limit being 50
- 📉 Feedback after each guess (`Too low`, `Too high`, `You guessed it!`)
- ❌ Game over message with correct answer
- 🔁 Validates non-numeric input gracefully

---

## 🛠 Built With

- [Rust](https://www.rust-lang.org/)
- [rand](https://crates.io/crates/rand) crate for random number generation
