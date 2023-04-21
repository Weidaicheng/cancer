use qrcode::QrCode;

fn main() {
    let code = QrCode::new("Hello, world!").unwrap();
    let qr_string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", qr_string);
}
