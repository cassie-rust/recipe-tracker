use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
/// CreateRecipe represents the data required to create a new recipe record.
/// It contains the recipe name, an optional reference to a recipe book,
/// and an optional URL.
///
/// Note that a CreateRecipe is invalid if it doesn't contain at least one of
/// the optional fields
pub struct CreateRecipe {
    pub name: String,
    pub book: Option<CreateRecipeBook>,
    pub url: Option<CreateRecipeUrl>,
}

impl<'de> Deserialize<'de> for CreateRecipe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Book,
            Url,
        }

        struct CreateRecipeVisitor;

        impl<'de> Visitor<'de> for CreateRecipeVisitor {
            type Value = CreateRecipe;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CreateRecipe")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut book = None;
                let mut url = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Book => {
                            if book.is_some() {
                                return Err(de::Error::duplicate_field("book"));
                            }
                            book = Some(map.next_value()?);
                        }
                        Field::Url => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            url = Some(map.next_value()?);
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;

                if book.is_none() && url.is_none() {
                    Err(de::Error::custom("recipes must contain either book or url"))?
                }
                Ok(CreateRecipe { name, book, url })
            }
        }
        const FIELDS: &[&str] = &["name", "book", "url"];
        deserializer.deserialize_struct("CreateRecipe", FIELDS, CreateRecipeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct CreateRecipeBook {
    pub title: String,
    pub isbn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct CreateRecipeUrl {
    pub name: String,
    pub url: String,
}

#[cfg(test)]
mod fixtures {
    use super::*;

    impl CreateRecipe {
        pub fn fixture(
            name: &str,
            book: Option<CreateRecipeBook>,
            url: Option<CreateRecipeUrl>,
        ) -> CreateRecipe {
            let mut recipe = CreateRecipe {
                name: name.to_string(),
                book,
                url,
            };

            if recipe.book.is_some() || recipe.url.is_some() {
                return recipe;
            }

            if rand::random() {
                recipe.book = Some(CreateRecipeBook::fixture("Example Book", None));
            } else {
                recipe.url = Some(CreateRecipeUrl::fixture(
                    "Example Url",
                    "https://example.com/",
                ));
            }
            recipe
        }
    }

    impl CreateRecipeBook {
        pub fn fixture(title: &str, isbn: Option<String>) -> CreateRecipeBook {
            CreateRecipeBook {
                title: title.to_string(),
                isbn,
            }
        }
    }

    impl CreateRecipeUrl {
        pub fn fixture(name: &str, url: &str) -> CreateRecipeUrl {
            CreateRecipeUrl {
                name: name.to_string(),
                url: url.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{"name": "Cake", "book": {"title": "Baking 101"}}"#;
        let recipe: CreateRecipe = serde_json::from_str(json).unwrap();
        assert_eq!(recipe.name, "Cake");
        assert_eq!(recipe.book.unwrap().title, "Baking 101");
    }

    #[test]
    fn test_missing_book_and_url() {
        let json = r#"{"name": "Cookies"}"#;
        let result = serde_json::from_str::<CreateRecipe>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_error() {
        let json = r#"{"name": "Pasta"}"#;
        let result = serde_json::from_str::<CreateRecipe>(json);
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("recipes must contain either book or url"));
    }
}
