pub use json::*;
pub struct Food {
    name: String,
    calories: [String; 2],
    proteins: f64,
    fats: f64,
    carbs: f64,
    nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for meal in foods {
        let cal_num: Vec<&str> = meal.calories[1].split("kcal").collect();
        let one_meal_cal: f64 = cal_num[0].parse().unwrap();
       cals += meal.nbr_of_portions * one_meal_cal;
       carbs += meal.nbr_of_portions * meal.carbs;
       proteins += meal.nbr_of_portions * meal.proteins;
       fats += meal.nbr_of_portions * meal.fats;

    }
    let mut data = json::JsonValue::new_object();

    data["cals"] = ((cals* 100.0).round() / 100.0).into();
    data["carbs"] = ((carbs* 100.0).round() / 100.0).into();
    data["proteins"] = ((proteins* 100.0).round() / 100.0).into();
    data["fats"] = ((fats* 100.0).round() / 100.0).into();
  
    return data;
}


