use libpaprika::RecipeSet;
use std::fs::File;

pub fn main() {
  let arg = match std::env::args().skip(1).next() {
    Some(arg) => arg,
    None => {
      println!("Usage: roundtrip PAPRIKARECIPE_FILE");
      return;
    }
  };

  let set = RecipeSet::from_file(&arg).unwrap();
  std::fs::remove_file(&arg).expect("Failed to delete file");
  set.to_writer(File::create(&arg).unwrap()).unwrap();
  println!("Read file {} into memory and wrote it back.", arg);
}
