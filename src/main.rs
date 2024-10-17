
// Think of enums like making multiple structs but not
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audio { title: String, artist: String },
}

impl Media {
    fn description(&self) -> String {
        // This is how you would do it with if let
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} by {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} by {}", title, director)
        // } else if let Media::Audio { title, artist } = self {
        //     format!("Audio: {} by {}", title, artist)
        // } else {
        //     panic!("This should never happen")
        // }

        // match is how to tell what self is
        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
            Media::Audio { title, artist } => format!("Audio: {} by {}", title, artist),
        }
    }
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
    // print_media(book);
    // print_media(movie);
    // print_media(audio);

    println!("{}", book.description());
    println!("{}", movie.description());
    println!("{}", audio.description());
}
