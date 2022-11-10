use svg::parser::Event;

fn main() {
    let path = "svg/rust_logo_black.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(tag_name, _,ref hm) => {
                if let Some(value) = hm.get("fill") {
                    println!("{:?} {:?}", tag_name, value);
                }
                if let Some(value) = hm.get("stroke") {
                    println!("{:?} {:?}", tag_name, value);
                }
            }
            _ => {},
        }
    }
}

