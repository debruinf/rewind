use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
        match dir.metadata() {
            Ok(m) => println!("{:?}", m.modified()),
            Err(err) => println!("{:?}", err),
        }
    }
    Ok(())
}
