
// Think of enums like making multiple structs but not
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audio { title: String, artist: String },
}

// Implementing a method for the enum allows you to run it on books, movies, and audio
fn print_media(media: Media) {
    println!("{:#?}", media);
}
fn main() {
    let book = Media::Book {
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string()
    };
    let movie = Media::Movie {
        title: "The Lord of the Rings".to_string(),
        director: "Peter Jackson".to_string()
    };
    let audio = Media::Audio {
        title: "The Beatles".to_string(),
        artist: "The Beatles".to_string()
    };

    // behold the power of enums
    print_media(book);
    print_media(movie);
    print_media(audio);
}
