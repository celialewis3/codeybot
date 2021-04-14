use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Film {
    id: String,
    title: String,
    original_title: String,
    original_title_romanised: String,
    description: String,
    director: String,
    producer: String,
    release_date: String,
    running_time: String,
    rt_score: String,
}

pub fn movie_info(movie_name: String, films: &Vec<Film>) -> String {
    let film = films.iter().find(|f| f.title == movie_name);

    match film {
        Some(f) => {
            let orig_title = (f.original_title).clone();
            let release_date = (f.release_date).clone();
            let description = (f.description).clone();
            format!(
                "{}\n{}\n{}\n{}",
                movie_name, release_date, orig_title, description
            )
        }
        None => {
            return "That film could not be found".to_string();
        }
    }
}
