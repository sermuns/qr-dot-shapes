use qrcode::QrCode;
use std::path::Path;
use svg::{Document, Node, node::element::Rectangle};

const QRCODE_DARK_COLOR: char = '█';

fn create_qrcode_string(data: &[u8]) -> String {
    let code = QrCode::new(data).unwrap();

    code.render::<char>()
        .dark_color(QRCODE_DARK_COLOR)
        .quiet_zone(false)
        // .module_dimensions(2, 1)
        .build()
}

fn render_qrcode_char_string_to_svg(inp: &str, path: impl AsRef<Path>) {
    const RECT_SIZE: usize = 10;

    let viewbox_width = inp.lines().next().unwrap().chars().count() * RECT_SIZE;
    let viewbox_height = inp.lines().count() * RECT_SIZE;

    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    for (y, line) in inp.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != QRCODE_DARK_COLOR {
                continue;
            }
            let rect = Rectangle::new()
                .set("y", y * RECT_SIZE)
                .set("x", x * RECT_SIZE)
                .set("width", RECT_SIZE)
                .set("height", RECT_SIZE);

            document.append(rect);
        }
    }

    svg::save(path, &document).unwrap();
}

fn main() {
    let string = create_qrcode_string(b"Hello");
    render_qrcode_char_string_to_svg(&string, "qr.svg");
}
