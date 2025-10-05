struct Message {
    message_id: u32,
    role: String,
    content: String,
}

impl Message {
    fn display(&self) {
        println!("Message ID: {}", self.message_id);
        println!("Role: {}", self.role);
        println!("Content: {}", self.content);
    }

    fn build_message(message_id: u32, role: &str, content: &str) -> Self {
        Self {
            message_id,
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

fn main() {
    let message3 = Message::build_message(1, "assistant", "Quem descobriu o brasil?");

    message3.display()
}
