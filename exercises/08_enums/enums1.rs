#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize = 0,
    Move = 1,
    Echo = 3,
    ChangeColor = 7,
    Quit = 99,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
