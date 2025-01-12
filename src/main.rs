use recipe_extractor::recipe::http::{fetch};
fn main() {
    let recipe = fetch("https://www.bbcgoodfood.com/recipes/ultimate-spaghetti-carbonara-recipe");

    println!("{:#?}", recipe);
}