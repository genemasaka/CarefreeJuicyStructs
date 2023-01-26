use std::io;

struct Album {
    name: String,
    artist: String,
    genre: String,
    release_year: i32,
    price: f64,
}

fn main() {
    create_album();
}

fn create_album() {
    println!("Enter Album name:\n");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Dang! Failed to read input");

    println!("Enter Artist name:\n");

    let mut artist = String::new();
    io::stdin()
        .read_line(&mut artist)
        .expect("Dang! Failed to read input");

    println!("Enter music genre:\n");

    let mut genre = String::new();
    io::stdin()
        .read_line(&mut genre)
        .expect("Dang! Failed to read input");

    println!("Enter Release Year:\n");

    let mut release_year = String::new();
    io::stdin()
        .read_line(&mut release_year)
        .expect("Dang! Failed to read input");
    let year: i32 = release_year.parse().unwrap();

    println!("Enter price:\n");

    let mut price = String::new();
    io::stdin()
        .read_line(&mut price)
        .expect("Dang! Failed to read input");
    let price_val: f64 = price.parse().unwrap();

    let album = Album {
        name: name,
        artist: artist,
        genre: genre,
        release_year: year,
        price: price_val,
    };
    println!(
        "Album name: {}\n Artist: {}\n Genre: {}\n Release Year: {}\n Price: {}\n",
        album.name, album.artist, album.genre, album.release_year, album.price
    );
}
