fn main() {
  let content = std::fs::read_to_string("text.md").unwrap();
  print!("{}", content);
  match std::fs::write("text.md", "Willkommen beim Beispiel-006!") {
    Ok(_) => {}
    Err(e) => eprintln!("{}", e),
  }
}
