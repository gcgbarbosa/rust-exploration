struct Message {
    message_id: u32,
    role: String,
    content: String,
}

struct GeographycCoordinate(i32, i32);

fn build_message(message_id: u32, role: &str, content: &str) -> Message {
    Message {
        message_id,
        role: role.to_string(),
        content: content.to_string(),
    }
}

fn main() {
    let message1 = Message {
        message_id: 1,
        role: String::from("assistant"),
        content: String::from("Quem descobriu o brasil?"),
    };

    println!("message: {}, {}", message1.role, message1.content);

    let mut message2 = Message {
        message_id: 1,
        role: String::from("assistant"),
        content: String::from("Quem descobriu o brasil?"),
    };

    message2.role = String::from("user");
    message2.content = String::from("Pedro Álvares Cabral");

    println!("message: {}, {}", message2.role, message2.content);

    let message3 = build_message(1, "assistant", "Quem descobriu o brasil?");

    println!("message: {}, {}", message3.role, message3.content);

    let message4 = Message {
        message_id: 10,
        ..message3
    };

    println!("message: {}, {}", message4.role, message4.content);

    let coord = GeographycCoordinate(10, 20);

    println!("<{},{}>", coord.0, coord.1)
}
