# Simple Password Generator

A simple CLI program that generates a random, secure password, which can be easily remembered by humans. Maybe? **For more advanced and feature-rich password generator, you can use [xkcdpass](https://github.com/redacted/XKCD-password-generator)**.

## How it works

It accomplishes this by using a list of words to create the password. 

## Usage

```bash
$ cargo run

Secret Words: Word1|Word2|Word4|Word3
New Password: r1Wdo-2orWd-4orWd-ro3dW^&$242
```
You can ammend your own magic words in the `txt` files or create your own. If you add new `txt` file, you need to update `main.rs` to read the new file. Will plan to make it smarter in the future.

## Disclaimer

This project is built for learning purpose on Rust programming language. Use this project at your own discretion.
