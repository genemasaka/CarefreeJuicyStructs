# Album Information System

This is a simple Rust program that stores information about albums, including the album name, artist, genre, release year, price, and sales. The program uses a struct and an implementation block to define an `Album` type, and includes methods to create new albums and check if an album is certified platinum.

## Getting Started

1. Clone this repository to your local machine

$ git clone https://github.com/[username]/album-info.git

2. Change into the project directory

$ cd album-info

3. Run the program

$ cargo run


## Prerequisites

- Rust programming language

## Using the Program

The program defines a struct `Album` with the following fields:

- `name`: album name (String)
- `artist`: album artist (String)
- `genre`: album genre (String)
- `year`: album release year (usize)
- `price`: album price (f64)
- `sales`: album sales (usize)

The `Album` type also includes two methods:

- `new()`: creates a new instance of the `Album` type with the specified field values
- `is_platinum()`: returns a `bool` indicating whether an album has sold more than 1 million units

The `main` function creates a new instance of the `Album` type, and prints whether it is certified platinum or not.

## Built With

- Rust programming language

## Contributing

Please feel free to contribute to this project by submitting a pull request.

## Author

genemasaka

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.





