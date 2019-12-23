use libpaprika::RecipeSet;

pub fn main() {
  let arg = match std::env::args().skip(1).next() {
    Some(arg) => arg,
    None => {
      println!("Usage: list PAPRIKARECIPE_FILE");
      return;
    }
  };

  let set = RecipeSet::from_file(arg).unwrap();
  for recipe in set.recipes.values() {
    println!("{}", recipe.name)
  }
}
