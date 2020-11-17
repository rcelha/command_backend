use backend_derive::*;
use std::vec::Vec;

pub trait FromResp {
    fn from_resp(v: &Vec<&str>) -> Option<Self>
    where
        Self: Sized;
}

pub struct Hello;

impl FromResp for Hello {
    fn from_resp(v: &Vec<&str>) -> Option<Self> {
        Some(Hello {})
    }
}

pub struct Exit;

impl FromResp for Exit {
    fn from_resp(v: &Vec<&str>) -> Option<Self> {
        Some(Exit {})
    }
}

pub trait Backend<T: FromResp> {
    fn execute(self: &Self, cmd: &T) -> Option<String>;
}

#[derive(CommandRegistry)]
pub struct InMemoryBackend;

impl Backend<Hello> for InMemoryBackend {
    fn execute(self: &Self, cmd: &Hello) -> Option<String> {
        Some("HELLO:OK".to_string())
    }
}

impl Backend<Exit> for InMemoryBackend {
    fn execute(self: &Self, cmd: &Exit) -> Option<String> {
        Some("EXIT:OK".to_string())
    }
}

#[derive(CommandRegistry)]
pub struct FileBackend;

impl Backend<Hello> for FileBackend {
    fn execute(self: &Self, cmd: &Hello) -> Option<String> {
        Some("F:HELLO:OK".to_string())
    }
}

impl Backend<Exit> for FileBackend {
    fn execute(self: &Self, cmd: &Exit) -> Option<String> {
        Some("F:EXIT:OK".to_string())
    }
}

pub enum CommandRegistry {
    Hello(Hello),
    Exit(Exit),
}

impl FromResp for CommandRegistry {
    fn from_resp(v: &Vec<&str>) -> Option<CommandRegistry> {
        match v.as_slice() {
            ["hello", ..] => Some(CommandRegistry::Hello(Hello::from_resp(v)?)),
            ["exit", ..] => Some(CommandRegistry::Exit(Exit::from_resp(v)?)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backend_manager() {
        let backend = InMemoryBackend {};
        let file_backend = FileBackend {};
        let command = CommandRegistry::from_resp(&vec!["hello"]).unwrap();
        assert_eq!(backend.execute(&command), Some("HELLO:OK".to_string()));
        assert_eq!(
            file_backend.execute(&command),
            Some("F:HELLO:OK".to_string())
        );
        let command = CommandRegistry::from_resp(&vec!["exit"]).unwrap();
        assert_eq!(backend.execute(&command), Some("EXIT:OK".to_string()));
        assert_eq!(
            file_backend.execute(&command),
            Some("F:EXIT:OK".to_string())
        );
    }

    #[test]
    fn it_works() {
        let backend = InMemoryBackend {};
        let command = Hello::from_resp(&vec!["hello"]).unwrap();
        assert_eq!(backend.execute(&command), Some("HELLO:OK".to_string()));
        let command = Exit::from_resp(&vec!["exit"]).unwrap();
        assert_eq!(backend.execute(&command), Some("EXIT:OK".to_string()));
    }

    #[test]
    fn it_works_file_backend() {
        let backend = FileBackend {};
        let command = Hello::from_resp(&vec!["hello"]).unwrap();
        assert_eq!(backend.execute(&command), Some("F:HELLO:OK".to_string()));
        let command = Exit::from_resp(&vec!["exit"]).unwrap();
        assert_eq!(backend.execute(&command), Some("F:EXIT:OK".to_string()));
    }
}
