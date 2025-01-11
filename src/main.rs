use reqwest::blocking;
use reqwest::header::{HeaderMap,HeaderValue, ACCEPT, USER_AGENT};
use scraper::Selector;
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
enum Kind {
    Text(String),
    List(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum RecipeScript {
    JSONObject(Recipe),
    JSONArray(Vec<Recipe>)
}

type DeserializedRecipe = Result<RecipeScript, String>;

#[derive(Serialize, Deserialize, Debug)]
struct RecipeValidator {
    #[serde(rename = "@context")]
    context: Option<String>,
    #[serde(rename = "@type")]
    kind: Option<Kind>,
}

impl Recipe {
    fn validate(&self) -> bool {
        let expected_context = String::from("https://schema.org");
        let expect_content_with_s = String::from("http://schema.org");
        let expected_type = String::from("Recipe");
        
        if let (Some(context), Some(kind)) = (self.context.clone(), self.kind.clone()) {
            let context_bool = context == expected_context || context == expect_content_with_s;
            let kind_bool = match kind {
                Kind::Text(s) => s == expected_type,
                Kind::List(l) => l.contains(&expected_type)
            };
            return context_bool && kind_bool;
        }
        false
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    #[serde(rename = "@context")]
    context: Option<String>,
    #[serde(rename = "@type")]
    kind: Option<Kind>,
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

fn main() {
    let url = "https://www.allrecipes.com/beef-birria-ramen-recipe-8749389";

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_str("text/html").unwrap());
    // You are a browser, no one will know
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36").unwrap());

    let client = blocking::Client::new();
    let response = client.get(url).headers(headers).send();
    
    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();

    let mut ld_scripts = document.select(&selector);

    
    
    let recipe = ld_scripts.find_map(|script| {
        let html = script.inner_html();
            match serde_json::from_str(&html) {
            Ok(recipe_script) => {
                match recipe_script {
                   RecipeScript::JSONArray(recipe_scripts) =>  {
                        for recipe_script in recipe_scripts {
                            if recipe_script.validate() {
                                return Some(recipe_script);
                            }
                        }
                        return None;
                    }
                    RecipeScript::JSONObject(recipe_script) => {
                        if recipe_script.validate() { 
                            return Some(recipe_script);
                        }
                        return None;
                    }
                }
            }
            Err(error) => {
                println!("Our: {error}");
                return None;
            }
        }
    });
    

    // let recipe: Recipe = serde_json::from_str(&recipe_string).expect("cant deserialise {recipe_string}");

    println!("recipe {:#?}", recipe);



}
