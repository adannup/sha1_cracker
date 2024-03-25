# SHA-1 Password Hash Finder

This is a simple program written in Rust that searches for a matching SHA-1 hash in a plaintext password wordlist.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/yourprogram.git
   ```

2. Navigate to the project directory:

   ```bash
   cd yourprogram
   ```

3. Compile the program using Cargo:

   ```bash
   cargo build --release
   ```

## Usage

To run the program, use the following command:

```bash
cargo run --release -- <path_to_wordlist> <sha1_hash_to_search>
```

For example:

```bash
cargo run --release -- wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
```

The program will search for the specified SHA-1 hash in the provided wordlist. If a match is found, it will display the corresponding password.

### Wordlist Example

The wordlist should be a plaintext file containing one password per line. Here is an example of how a wordlist file might look:

```
password1
securepassword
123456
passw0rd
```

## Contribution

Contributions are welcome! If you find a bug or have any improvement suggestions, feel free to create a pull request or open an issue on this repository.

## License

This project is licensed under the MIT License.
