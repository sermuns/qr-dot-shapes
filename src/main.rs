use qrcode::QrCode;
use std::path::Path;
use svg::{
    Document, Node,
    node::element::{Circle, Rectangle},
};

const QRCODE_DARK_COLOR: char = '█';
const RECT_SIZE_PX: usize = 10;
const LOCATION_ELEMENT_SIZE_DOTS: usize = 7;

fn create_qrcode_string(data: &[u8]) -> String {
    let code = QrCode::new(data).unwrap();

    code.render::<char>()
        .dark_color(QRCODE_DARK_COLOR)
        .quiet_zone(false)
        // .module_dimensions(2, 1)
        .build()
}

fn append_location_element(cx: f32, cy: f32, document: &mut Document) {
    let inner = Circle::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("r", 1.4 * RECT_SIZE_PX as f32);
    document.append(inner);

    let outer = Circle::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("r", 3.0 * RECT_SIZE_PX as f32)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1.0 * RECT_SIZE_PX as f32);
    document.append(outer);
}

fn render_qrcode_char_string_to_svg(inp: &str, path: impl AsRef<Path>) {
    let num_rows_dots = inp.lines().count();
    let num_columns_dots = num_rows_dots;

    let viewbox_height = num_rows_dots * RECT_SIZE_PX;
    let viewbox_width = num_columns_dots * RECT_SIZE_PX;

    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    // top left
    append_location_element(
        (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
        (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
        &mut document,
    );

    // top right
    append_location_element(
        (num_columns_dots as f32 - LOCATION_ELEMENT_SIZE_DOTS as f32 / 2.) * RECT_SIZE_PX as f32,
        (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
        &mut document,
    );

    // bottom right
    append_location_element(
        (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
        (num_rows_dots as f32 - LOCATION_ELEMENT_SIZE_DOTS as f32 / 2.) * RECT_SIZE_PX as f32,
        &mut document,
    );

    for (y, line) in inp.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            // top left + top right
            if y < LOCATION_ELEMENT_SIZE_DOTS
                && (x < LOCATION_ELEMENT_SIZE_DOTS
                    || x > num_columns_dots - LOCATION_ELEMENT_SIZE_DOTS - 1)
            {
                continue;
            }
            // bottom left
            if y > num_rows_dots - LOCATION_ELEMENT_SIZE_DOTS - 1 && x < LOCATION_ELEMENT_SIZE_DOTS
            {
                continue;
            }
            if character != QRCODE_DARK_COLOR {
                continue;
            }
            let rect = Rectangle::new()
                .set("y", y * RECT_SIZE_PX)
                .set("x", x * RECT_SIZE_PX)
                .set("width", RECT_SIZE_PX)
                .set("height", RECT_SIZE_PX);

            document.append(rect);
        }
    }

    svg::save(path, &document).unwrap();
}

fn main() {
    let string = create_qrcode_string(b"Hello");
    println!("{}", &string);
    render_qrcode_char_string_to_svg(&string, "qr.svg");
}
