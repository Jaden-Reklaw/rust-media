
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
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_at_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        } 
    }
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
    let podcast = Media::Podcast(100);
    let placeholder = Media::Placeholder;



    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio);
    catalog.add(podcast);
    catalog.add(placeholder);

    //Other ways to deal with options
    //Using unwrap not used very often
    println!("{:#?}", catalog.get_at_index(0).unwrap().description());
    //println!("{:#?}", catalog.get_at_index(13).unwrap().description()); //This will panic 

    //Using expect good to use for enviornment variables that you expect but should crash if they are not there
    println!("{:#?}", catalog.get_at_index(0).expect("Index out of bounds").description());
    //println!("{:#?}", catalog.get_at_index(13).expect("Index out of bounds").description()); //This will panic

    //Using unwrap_or_else
    println!("{:#?}", catalog.get_at_index(0).unwrap_or_else(|| &Media::Placeholder).description());
    println!("{:#?}", catalog.get_at_index(13).unwrap_or_else(|| &Media::Placeholder).description());

}
