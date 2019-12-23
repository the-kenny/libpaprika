# libpaprika

[![Latest Version](https://img.shields.io/crates/v/libpaprika.svg)](https://crates.io/crates/libpaprika)

A library to read and generate [Paprika](http://www.paprikaapp.com/) recipe files and recipe collections.

## Usage

```rust
use libpaprika::RecipeSet;

pub fn main() {
  let set = RecipeSet::from_file("My Recipes.paprikarecipes").unwrap();
  for recipe in set.recipes.values() {
    println!("{}", recipe.name)
  }
}

```
