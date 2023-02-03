// This line is a derive macro that will generate the Debug trait implementation
// for the Album struct
#[derive(Debug)]

// Defines a struct named Album with the following fields:
// name: String
// artist: String
// genre: String
// year: usize
// price: f64
// sales: usize
struct Album {
    name: String,
    artist: String,
    genre: String,
    year: usize,
    price: f64,
    sales: usize,
}

// Implementation of the Album struct
impl Album {
    // Defines a method named new that takes the following parameters:
    // name: String
    // artist: String
    // genre: String
    // year: usize
    // price: f64
    // sales: usize
    // and returns a value of type Album
    fn new(name: String, artist: String, genre: String, year: usize, price: f64, sales: usize) -> Album{
        // Returns a new instance of the Album struct with the specified fields
        Self {name, artist, genre, year, price, sales}   
    }

    // Defines a method named is_platinum that takes a reference to self and returns a bool
    fn is_platinum(&self) -> bool {
        // If the sales field is greater than or equal to 1000000
        if self.sales >= 1000000 {
            // return true
            true
        } else {
            // else return false
            false
        }
    }
}

// Main function
fn main() {

    // Creates a new instance of Album using the new method
    let album = Album::new(
        // Initializes the name field with the string "Mr. Morale and The Big Steppers"
        String::from("Mr. Morale and The Big Steppers"),
        // Initializes the artist field with the string "Kendrick Lamar"
        String::from("Kendrick Lamar"),
        // Initializes the genre field with the string "HipHop/Rap"
        String::from("HipHop/Rap"),
        // Initializes the year field with 2022
        2022,
        // Initializes the price field with 10.55
        10.55,
        // Initializes the sales field with 1039445
        1039445,
        );

    // Prints the name of the album and whether it is certified platinum or not
    println!("{} is certified platinum: {}", album.name, album.is_platinum());
}
