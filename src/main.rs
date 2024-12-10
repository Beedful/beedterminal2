fn main() {
    println!("Welcome to BeedTerminal 2");
    println!("Copyright (c) Beedful 2024");
    let mut input: String = String::new();

    while input.trim() != "x" {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "x" {
            break;
        }

        println!("You wrote {}", input);
    }

    println!("Bye!");
}
