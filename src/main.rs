mod model;
use model::media::Media;
use model::catalog::Catalog;

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
    

}
