mod cmd;

use cmd::Command;
use qrcode::QrCode;

fn main() {
    let mut root_command = Command::new(
        String::from("generate qr code"),
        String::from("qr TEXT"),
        |args| {
            let data = &args[0];
            let code = QrCode::new(data).unwrap();
            let qr_string = code
                .render::<char>()
                .quiet_zone(false)
                .module_dimensions(2, 1)
                .build();
            println!("{}", qr_string);
        },
    );
    root_command.execute();
}
