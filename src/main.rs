use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

fn main() {
    let path = "svg/rust-logo-blk.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        if let Event::Tag(Path, _, attributes) = event {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    Command::Move(..) => println!("Move!"),
                    Command::Line(..) => println!("Line!"),
                    Command::HorizontalLine(..) => println!("Horizontal Line!"),
                    Command::VerticalLine(..) => println!("Vertical Line!"),
                    Command::QuadraticCurve(..) => println!("Quadratic Curve!"),
                    Command::SmoothQuadraticCurve(..) => println!("Smooth Quadratic Curve!"),
                    Command::CubicCurve(..) => println!("Cubic Curve!"),
                    Command::SmoothCubicCurve(..) => println!("Smooth Cubic Curve!"),
                    Command::EllipticalArc(..) => println!("Elliptical Arc!"),
                    Command::Close => println!("Close!"),
                }
            }
        }
    }
}
