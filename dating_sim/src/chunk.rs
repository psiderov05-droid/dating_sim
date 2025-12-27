pub struct Chunk_{
    text: String,
    name_of_character_who_says_it: String,
    is_a_choice: bool,
    choice_screen_name: String,
}

impl Chunk_{
    pub fn new(
        text: String,
        name_of_character_who_says_it: String,
        is_a_choice: bool,
        choice_screen_name: String,
    ) -> Self {
        Self {
            text,
            name_of_character_who_says_it,
            is_a_choice,
            choice_screen_name,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn name_of_character(&self) -> &str {
        &self.name_of_character_who_says_it
    }

    pub fn is_choice(&self) -> bool {
        self.is_a_choice
    }

    pub fn choice_screen_name(&self) -> &str {
        &self.choice_screen_name
    }
}
