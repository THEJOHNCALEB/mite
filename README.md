# Mite - "Look" (見て)

Mite is a command-line file reader that allows you to search for a specific word within a file and prints the line containing that word. Interestingly, "Mite" is a Japanese word meaning "Look" (見て).

## Features

- Search for a word in a specified file
- Display the line containing the searched word

## Installation

To install Mite, you can use Cargo:

```sh
cargo install mite
```

Or you can clone the repository and build the project using Cargo:

```sh
cargo install --path .
```

You can also find Mite on [crates.io](https://crates.io/crates/mite).

## Usage

To use Mite, run the following command:

```sh
mite <word> <file_path>
```

- `<word>`: The word you want to search for in the file.
- `<file_path>`: The path to the file you want to search.

Example:

```sh
mite example /path/to/file.txt
```

This command will print the line containing the word "example" from the specified file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
