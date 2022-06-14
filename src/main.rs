use keybow;

fn handle_back(state: bool) {}

fn handle_forward(state: bool) {}

fn handle_toggle(state: bool) {}

fn main() -> Result<!, std::io::Error> {
    let mut k = match keybow::Keybow::new_mini() {
        Ok(k) => k,
        Err(e) => std::io::Error::new(std::io::ErrorKind::Other, e)
    };
    k.add_key(0, handle_back);
    k.add_key(1, handle_toggle);
    k.add_key(2, handle_forward);
    Ok(k.run())
}
