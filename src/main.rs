
// Think of enums like making multiple structs but not
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audio { title: String, artist: String },
    Podcast(u32), //You can do this 
    Placeholder, // or this
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
            Media::Podcast(episodes) => format!("Podcast with {} episodes", episodes),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    media: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { media: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.media.push(item);
    }
}

fn main() {
    // How to decide when to use a struct or enum
    // Use structs when you have some similar functions but same data also when you have a ton of fields
    // imagine they all have description but book have read, movies play, audio listen
    // Use enums when you have same functions but different data works better with less fields
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
    let podcast = Media::Podcast(100);
    let placeholder = Media::Placeholder;

    println!("{}", book.description());
    println!("{}", movie.description());
    println!("{}", audio.description());
    println!("{}", podcast.description());
    println!("{}", placeholder.description());

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
