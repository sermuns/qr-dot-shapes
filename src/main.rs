use qrcode::QrCode;
use std::path::Path;
use svg::{
    Document, Node,
    node::element::{Circle, Path as SvgPath, Rectangle},
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

fn append_circular_location_element(cx: f32, cy: f32, document: &mut Document) {
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

    // // top left
    // append_circular_location_element(
    //     (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
    //     (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
    //     &mut document,
    // );
    //
    // // top right
    // append_circular_location_element(
    //     (num_columns_dots as f32 - LOCATION_ELEMENT_SIZE_DOTS as f32 / 2.) * RECT_SIZE_PX as f32,
    //     (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
    //     &mut document,
    // );
    //
    // // bottom right
    // append_circular_location_element(
    //     (LOCATION_ELEMENT_SIZE_DOTS * RECT_SIZE_PX) as f32 / 2.,
    //     (num_rows_dots as f32 - LOCATION_ELEMENT_SIZE_DOTS as f32 / 2.) * RECT_SIZE_PX as f32,
    //     &mut document,
    // );

    for (y, line) in inp.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != QRCODE_DARK_COLOR {
                continue;
            }

            // top left + top right
            if y < LOCATION_ELEMENT_SIZE_DOTS
                && (x < LOCATION_ELEMENT_SIZE_DOTS
                    || x > num_columns_dots - LOCATION_ELEMENT_SIZE_DOTS - 1)
            {
                let rect = Rectangle::new()
                    .set("y", y * RECT_SIZE_PX)
                    .set("x", x * RECT_SIZE_PX)
                    .set("width", RECT_SIZE_PX)
                    .set("height", RECT_SIZE_PX);

                document.append(rect);
            }
            // bottom left
            if y > num_rows_dots - LOCATION_ELEMENT_SIZE_DOTS - 1 && x < LOCATION_ELEMENT_SIZE_DOTS
            {
                let rect = Rectangle::new()
                    .set("y", y * RECT_SIZE_PX)
                    .set("x", x * RECT_SIZE_PX)
                    .set("width", RECT_SIZE_PX)
                    .set("height", RECT_SIZE_PX);

                document.append(rect);
            }

            if false {
                let rect = Rectangle::new()
                    .set("y", y * RECT_SIZE_PX)
                    .set("x", x * RECT_SIZE_PX)
                    .set("width", RECT_SIZE_PX)
                    .set("height", RECT_SIZE_PX);

                document.append(rect);
            } else {
                let path = SvgPath::new().set("d", "M9.00001 0H7.00001L5.51292 4.57681L0.700554 4.57682L0.0825195 6.47893L3.97581 9.30756L2.48873 13.8843L4.10677 15.0599L8.00002 12.2313L11.8933 15.0599L13.5113 13.8843L12.0242 9.30754L15.9175 6.47892L15.2994 4.57681L10.4871 4.57681L9.00001 0Z").set("transform", format!("translate({} {}) scale(0.6)",
            x* RECT_SIZE_PX, y*RECT_SIZE_PX
                    ));
                document.append(path);
            }
        }
    }

    svg::save(path, &document).unwrap();
}

fn main() {
    let string = create_qrcode_string(b"Hello");
    println!("{}", &string);
    render_qrcode_char_string_to_svg(&string, "qr.svg");
}
