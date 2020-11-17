use backend::*;
use backend_derive::*;

#[derive(CommandRegistry)]
pub struct HibrydBackend;

impl Backend<Hello> for HibrydBackend {
    fn execute(self: &Self, _: &Hello) -> Option<String> {
        Some("HELLO:OK + F:HELLO:OK".to_string())
    }
}

impl Backend<Exit> for HibrydBackend {
    fn execute(self: &Self, _: &Exit) -> Option<String> {
        Some("EXIT:OK + F:EXIT:OK".to_string())
    }
}

fn main() {
    println!("Hello, world!");

    let backend = InMemoryBackend {};
    let file_backend = FileBackend {};
    let h_backend = HibrydBackend {};

    let command = CommandRegistry::from_resp(&vec!["hello"]).unwrap();

    println!("{}", backend.execute(&command).unwrap());
    println!("{}", file_backend.execute(&command).unwrap());
    println!("{}", h_backend.execute(&command).unwrap());

    let command = CommandRegistry::from_resp(&vec!["exit"]).unwrap();

    println!("{}", backend.execute(&command).unwrap());
    println!("{}", file_backend.execute(&command).unwrap());
    println!("{}", h_backend.execute(&command).unwrap());
}
