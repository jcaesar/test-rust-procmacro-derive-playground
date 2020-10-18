use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::fmt::Debug;

#[derive(HelloMacro)]
struct Pancakes {
    bananas: String,
    apples: u64,
}

fn main() {
    let pc = Pancakes {
        bananas: "Yellow".to_string(),
        apples: 4,
    };
    println!(
        "Memb: {:?}",
        pc.members()
            .into_iter()
            .map(
                |(name, value)| 
                if let Some(value) = value.downcast_ref::<String>() {
                    format!("{}: {}", name, value)
                } else if let Some(value) = value.downcast_ref::<&Debug>() {
                    format!("{}: {:?} - the sucky part is: this won't work", name, value)
                } else {
                    format!("{} is opaque", name)
                }
            )
            .collect::<Vec<String>>()
            .join(", ")
    );
}
