# Simple Password Generator

## Description
This is a simple CLI program that generates a random, secure password, which can be easily remembered by humans. It accomplishes this by using a list of words to create the password. **It is inspired by [xkcdpass](https://github.com/redacted/XKCD-password-generator), which is more advanced and feature-rich.**

## Usage
```bash
$ cargo run

Secret Words: Word1|Word2|Word4|Word3
New Password: r1Wdo-2orWd-4orWd-ro3dW^&$242
```
You can ammend your own magic words in the `txt` files or create your own. If you add new `txt` file, you need to update `main.rs` to read the new file. Will plan to make it smarter in the future.