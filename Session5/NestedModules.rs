pub mod movies {
    pub mod english {
        pub mod comedy {
            pub fn play_c1(name: String) {  //snake case name
                println!("Playing comedy movie {}", name);
            }
            fn play_c2(name: String) {
                println!("Playing comedy movie {}", name);
            }
        }

        mod action {
            pub fn play_a1(name: String) {
                println!("Playing action movie {}", name);
            }
            fn play_a2(name: String) {
                println!("Playing action movie {}", name);
            }
        }
    }
}

use movies::english::comedy::play_c1;

fn test_module(movie_name: String) {
    // movies::english::comedy::play_c1("testMovieC1".to_string());
    play_c1(movie_name);
}

fn main() {

    movies::english::comedy::play_c1("MovieC1".to_string()); //.to_string() -> reason: expected struct `String`, found `&str`
    // movies::english::comedy::play_c2("MovieC2".to_string()); //error: private function

    // movies::english::action::play_a1("MovieA1".to_string()); //error: private module
    // movies::english::action::play_a2("MovieA2".to_string()); //error: private module

    test_module("anyMovie".to_string());

}