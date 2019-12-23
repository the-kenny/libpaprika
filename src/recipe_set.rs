use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::path::Path;

use super::*;

#[derive(Debug)]
pub struct RecipeSet {
  pub recipes: HashMap<Uuid, Recipe>,
}

impl RecipeSet {
  pub fn new() -> RecipeSet {
    RecipeSet {
      recipes: HashMap::new(),
    }
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RecipeSet, Error> {
    let file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut recipes = HashMap::new();

    for i in 0..archive.len() {
      let file = archive.by_index(i).unwrap();
      let recipe = Recipe::from_reader(file)?;
      recipes.insert(recipe.uid.clone(), recipe);
    }

    Ok(RecipeSet { recipes: recipes })
  }

  pub fn to_writer<W: Write + Seek>(&self, writer: W) -> Result<(), Error> {
    let mut zip = zip::ZipWriter::new(writer);
    let options =
      zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (_, recipe) in &self.recipes {
      zip.start_file(&format!("{}.paprikarecipe", recipe.name), options)?;
      recipe.to_writer(&mut zip)?;
    }

    Ok(())
  }
}
