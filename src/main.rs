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
    let args: Vec<String> = std::env::args().collect();
    let name = args.get(1).cloned().unwrap_or_else(|| "world".to_string());
    let lang = args.get(2).cloned().unwrap_or_else(|| "en".to_string());

    let greeter = Greeter { name, language: lang };
    println!("{}", greeter.format_greeting());
}
