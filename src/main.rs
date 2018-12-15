use reqwest;
use scraper::{Html, Selector};

fn main() {
    let res = reqwest::get("https://www.nicovideo.jp/ranking/fav/daily/vocaloid?matrix_menu");
    res.map(|mut body| {
        let music_videos = parse_html( body.text().unwrap() );
        for music_video in music_videos {
            println!("{:?}", music_video);
        }
    });
}

#[derive(Debug)]
struct MusicVideo {
    url: String,
    title: String,
}

impl MusicVideo {

    fn new(title: String, url: String) -> MusicVideo {
        MusicVideo {
            title: title,
            url: url,
        }
    }

}

fn parse_html(html: String) -> Vec<MusicVideo> {
    let document = Html::parse_document(&html);
    let selector = Selector::parse(".videoRanking").unwrap();
    let music_videos: Vec<MusicVideo> = document.select(&selector).map(|node| {
        let last_child = 
            node.children()
                .filter(|node| node.value().is_element())
                .last()
                .unwrap();
        let item_title =
            last_child.children()
                .filter(|node| node.value().is_element())
                .next()
                .unwrap();
        let movie_link =
            item_title.children()
                .filter(|node| node.value().is_element())
                .next()
                .unwrap()
                .value()
                .as_element()
                .unwrap();
        MusicVideo::new(
            movie_link.attr("title").unwrap().to_string(),
            movie_link.attr("href").unwrap().to_string()
        )
    }).collect();
    music_videos
}

/*

        let last_child = get_last_element(node);
            node.children()
                .filter(|node| node.value().is_element())
                .last()
                .unwrap()
                .value()
                .as_element()
                .unwrap();
                */

        // print!("{} ", last_child.is_document());
        // print!("{} ", last_child.is_fragment());
        // print!("{} ", last_child.is_doctype());
        // print!("{} ", last_child.is_comment());
        // print!("{} ", last_child.is_text());
        // print!("{} ", last_child.is_element());
        // println!("");
