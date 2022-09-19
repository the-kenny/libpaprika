use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use super::Error;

pub type Url = String;
pub type Uuid = String;
pub type Base64 = String;
pub type Hash = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipePhoto {
    pub data: Base64,
    pub filename: String,
    pub name: String,
    pub hash: Hash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub created: String,
    pub difficulty: String,
    pub photo_large: Option<Base64>,
    pub uid: Uuid,
    pub photo: Option<String>,
    pub categories: Vec<String>,
    pub notes: String,
    pub prep_time: String,
    pub cook_time: String,
    pub source: String,
    pub source_url: String,
    pub photo_data: Option<Base64>,
    pub total_time: String,
    pub name: String,
    pub ingredients: String,
    pub photos: Vec<RecipePhoto>,
    pub rating: u8,
    pub description: String,
    pub photo_hash: Option<Hash>,
    pub image_url: Option<Url>,
    pub nutritional_info: String,
    pub directions: String,
    #[serde(skip_serializing)]
    pub hash: Hash,
    pub servings: String,
}

impl Recipe {
    pub fn from_reader<R: Read>(rdr: R) -> Result<Recipe, Error> {
        let gzip_decoder = libflate::gzip::Decoder::new(rdr)?;
        Ok(serde_json::from_reader(gzip_decoder)?)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Recipe, Error> {
        Ok(Self::from_reader(File::open(path)?)?)
    }

    pub fn to_writer<W: Write>(&self, writer: W) -> Result<(), Error> {
        let mut gzip_encoder = libflate::gzip::Encoder::new(writer)?;
        serde_json::to_writer(&mut gzip_encoder, self)?;
        gzip_encoder.finish().into_result()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialization() {
        let data = include_bytes!("test/Brownies.paprikarecipe.json");
        let result: Result<Recipe, _> = serde_json::from_slice(data);
        assert!(result.is_ok());

        let recipe = result.unwrap();
        assert_eq!(recipe.name, "Brownies");
    }

    #[test]
    fn from_file() {
        let data = include_bytes!("test/Brownies.paprikarecipe");
        let result = Recipe::from_reader(&data[..]);

        let recipe = result.unwrap();
        assert_eq!(recipe.name, "Brownies");
    }
}
