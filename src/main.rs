#[derive(Debug)]
struct Album {
    name: String,
    artist: String,
    genre: String,
    release_year: i32,
    price: f64,
}

fn main() {
    let name = String::from("Mr. Morale And The Big Steppers");
    let artist = String::from("Kendrick Lamar");
    let genre = String::from("Rap/Hiphop");
    let release_year = 2022;
    let price = 10.55;
    println!("{:#?}", create_album(name, artist, genre, release_year, price));
}

fn create_album(album_name: String, album_artist: String, album_genre: String, album_release_year: i32, album_price: f64) -> Album {
    let new_album = Album {
        name: album_name,
        artist: album_artist,
        genre: album_genre,
        release_year: album_release_year,
        price: album_price,
    };
    new_album
}
