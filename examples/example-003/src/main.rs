fn main() {
  let args: Vec<String> = std::env::args().collect();
  std::process::exit(args.len().saturating_sub(1) as i32);
}
