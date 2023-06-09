pub mod datatypes {
    use std::{fs::File, io::Read};
    use serde::{Deserialize, Serialize};

    use tui::widgets::ListState;

    pub struct Brews{
        pub state: ListState,
        pub brews: Vec<Brew>
    }

    #[derive(Serialize, Deserialize)]
    struct BrewList {
        pub brews: Vec<Brew>
    }

    #[derive(Serialize, Deserialize)]
    pub struct Brew{
        pub name: String,
        pub rating: u8
    }
    
    impl Brews {
        pub fn load_brews_from_file() -> Brews{
            let mut file = File::open("data.json").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let brew_vec: BrewList =
                serde_json::from_str(&data).expect("JSON was not well-formatted");
            
            let brews: Brews = Brews {
                state: ListState::default(),
                brews: brew_vec.brews
            };

            return brews;
        }

        pub fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.brews.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    
        pub fn previous(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.brews.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    
        pub fn unselect(&mut self) {
            self.state.select(None);
        }
    }
    
}