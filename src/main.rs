struct Greeter {
    name: String,
    language: String,
}

impl Greeter {
    fn format_greeting(&self) -> String {
        match self.language.as_str() {
            "es" => format!("¡Hola, {}!", self.user_name),
            "fr" => format!("Bonjour, {}!", self.user_name),
            _    => format!("Hello, {}!", self.user_name),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
