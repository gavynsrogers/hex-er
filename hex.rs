use std::env;

fn main() {
    print!("0x");
    for arg in env::args() {
        let arg = arg.as_bytes();
        if arg[0] == '/' as u8 || arg[0] == '.' as u8 { continue; }
        else {
            for ch in arg {
                print!("{:>0len$x}", *ch, len = 2);
            }
        }
    } println!("");
}
