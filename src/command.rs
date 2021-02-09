#[derive(Debug, Clone)]
pub struct Command {
    pub verb: Option<String>,
    pub obj: Option<String>,
    pub prep: Option<String>,
    pub obj_prep: Option<String>,
}

pub struct CommandResult {
    pub message: String,
}

impl CommandResult {
    pub fn didnt_understand(word: String) -> Self {
        CommandResult {
            message: format!("I don't know what {} means.", word),
        }
    }

    pub fn no_object(name: String) -> Self {
        CommandResult {
            message: format!("There is no {} here.", name),
        }
    }

    pub fn cant_do_that(verb: String) -> Self {
        CommandResult {
            message: format!("You can't {} that.", verb),
        }
    }
    pub fn doesnt_have_that(character_name: String) -> Self {
        CommandResult {
            message: format!("The {} doesn't have that.", character_name),
        }
    }
}
