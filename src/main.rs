mod cmd;

use cmd::Command;
use qrcode::QrCode;

fn main() {
    let mut root_command = Command::new("generate qr code", "qr TEXT", |text, flags| {
        let data = text.unwrap();
        let code = QrCode::new(data).unwrap();
        let qr_string = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();
        println!("{}", qr_string);
    });
    root_command.execute();
}
