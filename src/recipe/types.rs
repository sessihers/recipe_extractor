use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Image {
    URL(String),
    ImageObject(std::collections::HashMap<String, serde_json::Value>)
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeInstruction {
    #[serde(rename = "@type")]
    kind: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum RecipeYield {
    Text(String),
    Number(u8),
    NumberList(Vec<u8>),
    TextList(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Kind {
    Text(String),
    TextList(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    #[serde(rename = "@type")]
    pub kind: Option<Kind>,
    #[serde(rename = "cookTime")]
    cook_time: Option<String>,
    #[serde(rename = "datePublished")]
    date_published: Option<String>,
    description: Option<String>,
    image: Option<Image>,
    #[serde(rename = "recipeIngredient")]
    recipe_ingredient: Option<Vec<String>>,
    name: Option<String>,
    #[serde(rename = "recipeInstructions")]
    recipe_instructions: Option<Vec<RecipeInstruction>>,
    #[serde(rename = "recipeYield")]
    recipe_yield: Option<RecipeYield>,
}
