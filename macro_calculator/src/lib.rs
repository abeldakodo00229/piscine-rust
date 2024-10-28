
#[derive(Debug, PartialEq)]
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
  

    for food in foods {
        let number_str =food.calories[1].trim_end_matches("kcal");
        let number_f64: f64 = number_str.parse().expect("Error");

        total_cals +=  number_f64 * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }
    let mut data = json::JsonValue::new_object();
    data["cals"] = convert(total_cals).into();
    data["carbs"] = convert(total_carbs).into();
    data["proteins"] = convert(total_proteins).into();
    data["fats"] = convert(total_fats).into();
    // println!("{}", convert(total_cals));
          data
}

        
            fn convert(f: f64) -> f64 {
                let  r : f64 = (f * 100.0).round() / 100.0;
                if r % 1.0 == 0.0 {
                 return   (f * 10.0).round() / 10.0;
                }
                r
            }
        


