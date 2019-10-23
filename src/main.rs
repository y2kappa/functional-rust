
fn flat_map() {

}

// ideas
// - [x] filter on is vegan
// - [x] flat map on ingredients (things to buy)
// monads for recipe execution
// option?
// flatten
// option_map option_default
// fold left
// pattern match
// templated classes
// Show the nonconcurrent race condition in a for loop

#[derive(PartialEq)]
#[derive(Debug)]
enum Food {
    Pizza,
    Pasta,
    Gelato
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Ingredient {
    Salami,
    Wheat,
    Milk,
    Tomatoes,
    Rice,
    Parmigiano
}

fn ingredients(food: &Food) -> Vec<Ingredient> {
    match food {
        Food::Pizza => vec![
            Ingredient::Salami,
            Ingredient::Tomatoes,
            Ingredient::Wheat],

        Food::Pasta => vec![
            Ingredient::Rice,
            Ingredient::Tomatoes,
            Ingredient::Parmigiano],

        Food::Gelato => vec![Ingredient::Milk]
    }
}

fn flatten<T>(v: &Vec<Vec<T>>) -> {
    v
        .iter_mut()
        .fold()
}

fn main() {
    let foods = vec![Food::Pizza, Food::Pasta, Food::Gelato];
    println!("Foods on the menu: {:?}", foods);

    for food in &foods {
        println!("{:?} contains {:?}",
            food,
            ingredients(&food));
    }

    let already_bought = vec![
        Ingredient::Parmigiano,
        Ingredient::Tomatoes];

    let is_vegan = |x: &Ingredient| match x {
        Ingredient::Rice
        | Ingredient::Tomatoes
        | Ingredient::Wheat => true,
        | _ => false
    };

    let ingredients_to_buy : Vec<Ingredient> =
        (&foods)
            .iter()
            .flat_map(|ref food| ingredients(&food))
            .filter(|ref ingr| !already_bought.contains(ingr))
            .filter(|ref ingr| is_vegan(ingr) )
            .collect();

    println!("All ingredients to buy {:?}",
        ingredients_to_buy);
}
