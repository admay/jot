// this will takes args and a config file
pub fn handle(dose: i8, wake: String, mood: String) -> std::io::Result<()> {
    println!("{:?}", dose);
    println!("{:?}", wake);
    println!("{:?}", mood);
    Ok(())
}
