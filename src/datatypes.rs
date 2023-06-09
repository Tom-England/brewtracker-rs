pub mod datatypes {
    use std::{fs::File, io::Read};
    use serde::{Deserialize, Serialize};

    use tui::widgets::ListState;

    pub struct StatefulList<T> {
        pub state: ListState,
        pub items: Vec<T>,
    }

    
    #[derive(Serialize, Deserialize)]
    pub struct Brews{
        pub brews: Vec<Brew>
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct Brew{
        pub name: String
    }
    
    impl<T> StatefulList<T> {
        pub fn with_items(items: Vec<T>) -> StatefulList<T> {
            StatefulList {
                state: ListState::default(),
                items,
            }
        }
    
        pub fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
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
                        self.items.len() - 1
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
    
    impl Brews {
        pub fn load_brews_from_file() -> Brews{
            let mut file = File::open("data.json").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let brews: Brews =
                serde_json::from_str(&data).expect("JSON was not well-formatted");
            
            return brews;
        }
    }
    
}