use trimsec::args::get_cur_cmd;

fn main() {
    let eexit = |e| {
        eprintln!("{e}");
        std::process::exit(1);
    };

    match get_cur_cmd() {
        Ok(c) => {
            if let Err(e) = c.run() {
                eexit(e);
            }
        }
        Err(e) => {
            eexit(e);
        }
    }
}
