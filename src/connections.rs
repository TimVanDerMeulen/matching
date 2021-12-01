struct Element {
    id: String,
    connections: Vec<Connection>,
}

struct Connection {
    to_element: Element,
    score: u8,
    locked: bool
}