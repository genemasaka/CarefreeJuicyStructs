#[derive(Debug)]
struct Album {
    name: String,
    artist: String,
    genre: String,
    year: usize,
    price: f64,
    sales: usize,
}

impl Album {
    fn new(name: String, artist: String, genre: String, year: usize, price: f64, sales: usize) -> Album{
        Self {name, artist, genre, year, price, sales}   
    }

    fn is_platinum(&self) -> bool {
        if self.sales >= 1000000 {
            true
        } else {
            false
        }
    }
}

fn main() {

    let album = Album::new(
        String::from("Mr. Morale and The Big Steppers"),
        String::from("Kendrick Lamar"),
        String::from("HipHop/Rap"),
        2022,
        10.55,
        1039445,
        );
    println!("{} is certified platinum: {}", album.name, album.is_platinum());
}