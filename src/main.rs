use svg::parser::Event;

fn main() {
    let path = "svg/lichess_logo_black.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).expect("The file in the given path does not exist!")
    {
        if let Event::Tag(tag_name, _, ref hm) = event {
            if tag_name == "style" {
                println!("{:?}", event);
            }
            if let Some(value) = hm.get("fill") {
                println!("{:?} {:?}", tag_name, value);
            }
            if let Some(value) = hm.get("stroke") {
                println!("{:?} {:?}", tag_name, value);
            }
            if let Some(value) = hm.get("style") {
                println!("{:?} {:?}", tag_name, value);
            }
        }
    }
}
