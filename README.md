# Overview

I wanted to learn the Rust language for awhile now and I chose to build a password manager!

I used a lot of new things to me with the Rust language. I used the AES-256-GCM for encyrpting. Then the program retrieves and decrypts the passwords. It uses a simple command-line interface (CLI) for interaction. Fianlly it stores the data in a json file on disk.

I thought it would be fun to build something a little more complex (at least it felt that way to me) for learning the Rust language. I felt like it  had its challenges and I also thought it had some common things that would be good to know/master when learning this language.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- Visual Studio Code
- Git/GitHub
- RUSTUP-INIT.EXE(64-BIT)
- AES-GCM
- rand

# Useful Websites

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

# Future Work

- Improve Security: Use proper key management system instead of stoeing keys in a file.
- GUI Interface: Use eframe  or Tauri for grapophical interface.
- Hash-based Encrypytion Secure passwords using  Argon2 for hashing.