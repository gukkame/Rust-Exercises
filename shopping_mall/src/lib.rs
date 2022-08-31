use crate::mall::floor::{self};
use crate::mall::guard::Guard;
use crate::{employee::Employee, mall::floor::store};
use mall::{
    floor::store::{employee, Store},
    Mall,
};
mod mall;

pub fn biggest_store(mall: Mall) -> Store {
    let mut name = "".to_string();
    let mut area = 0;
    let mut employees1 = vec![Employee::new("", 0, 0, 0, 0.0)];
    for floor in mall.floors {
        for store in floor.stores {
            if area < store.square_meters {
                name = store.name;
                area = store.square_meters;
                employees1 = store.employees;
            }
        }
    }

    Store {
        name: name,
        square_meters: area,
        employees: employees1,
    }
}
pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut salary = 0.0;
    let mut employees1 = vec![Employee::new("", 0, 0, 0, 0.0)];
    for floor in mall.floors {
        for store in floor.stores {
            for empl in store.employees {
                if salary < empl.salary {
                    salary = empl.salary;
                    employees1.pop();
                    employees1.push(Employee::new(
                        &empl.name,
                        empl.age,
                        empl.working_hours.0,
                        empl.working_hours.1,
                        empl.salary,
                    ));
                }
            }
        }
    }
    employees1
    // for empl in store.employees {

    // }
}
pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut all_Empl = 0;

    for floor in mall.floors {
        for store in floor.stores {
            all_Empl += store.employees.len();
        }
    }
    all_Empl += mall.guards.len();

    all_Empl
}
pub fn fire_old_securities(mall: &mut Mall) -> Mall {
    let mut mall1 = mall.clone();
    let mut i = 1;
    let len = mall1.guards.len().clone();
    let mut k = 1;
    while i <= len {
        if mall1.guards[mall1.guards.len() - k].age > 49 {
            mall1.guards.pop();
        } else {
            k += 1;
        }
        i += 1;
    }
    mall1
}
pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) -> Mall {
    let mut mall_edit = mall.clone();
    //get mall area size / 200 and get number of how many guard should be there and acourdingly get extra if needed!
    let mut area = 0;

    for floor in mall_edit.floors.clone() {
        for store in floor.stores {
            area += store.square_meters;
        }
    }
    let guards_rn: u64 = mall_edit.guards.len().try_into().unwrap();
    let guards_needed = area / 200;
    if guards_rn < guards_needed {
        let mut numb = guards_needed - guards_rn;
        for one_guard in guards {
            if numb > 0 {
                mall_edit.guards.push(one_guard);
                numb -= 1;
            }
        }
    }

    mall_edit
}
pub fn cut_or_raise(mall: &mut Mall) -> Mall {
    let mut mall_edit = mall.clone();
    for (i, floor) in mall_edit.floors.clone().iter().enumerate() {
        for (j, store) in floor.stores.iter().enumerate() {
            for (k, empl) in store.employees.iter().enumerate() {
                if empl.working_hours.1 - empl.working_hours.0 > 10 {
                    mall_edit.floors[i].stores[j].employees[k].salary = empl.salary * 1.1;
                } else {
                    mall_edit.floors[i].stores[j].employees[k].salary = empl.salary * 0.9;
                }
            }
        }
    }
    mall_edit
}

fn main() {
    let secs = vec![
        mall::guard::Guard::new("John Oliver", 34, 7),
        mall::guard::Guard::new("Logan West", 23, 2),
        mall::guard::Guard::new("Bob Schumacher", 53, 15),
    ];

    let footzo_emp = vec![
        mall::floor::store::employee::Employee::new("Finbar Haines", 36, 9, 14, 650.88),
        mall::floor::store::employee::Employee::new("Roksanna Rocha", 45, 13, 22, 772.00),
        mall::floor::store::employee::Employee::new("Sienna-Rose Penn", 26, 9, 22, 1000.43),
    ];
    let swashion_emp = vec![
        mall::floor::store::employee::Employee::new("Abdallah Stafford", 54, 8, 22, 1234.21),
        mall::floor::store::employee::Employee::new("Marian Snyder", 21, 8, 14, 831.90),
        mall::floor::store::employee::Employee::new("Amanda Mclean", 29, 13, 22, 1222.12),
        mall::floor::store::employee::Employee::new("Faizaan Castro", 32, 11, 18, 1106.43),
    ];
    let pizbite_emp = vec![
        mall::floor::store::employee::Employee::new("Juniper Cannon", 21, 16, 23, 804.35),
        mall::floor::store::employee::Employee::new("Alena Simon", 28, 9, 15, 973.54),
        mall::floor::store::employee::Employee::new("Yasemin Collins", 29, 9, 19, 986.33),
        mall::floor::store::employee::Employee::new("Areeb Roberson", 54, 9, 22, 957.82),
        mall::floor::store::employee::Employee::new("Rocco Amin", 44, 13, 23, 689.21),
    ];
    let grill_emp = vec![
        mall::floor::store::employee::Employee::new("Rhian Crowther", 45, 9, 15, 841.18),
        mall::floor::store::employee::Employee::new("Nikkita Steadman", 52, 14, 22, 858.61),
        mall::floor::store::employee::Employee::new("Reginald Poole", 32, 9, 22, 1197.64),
        mall::floor::store::employee::Employee::new("Minnie Bull", 54, 14, 22, 1229.73),
    ];
    let sumo_emp = vec![
        mall::floor::store::employee::Employee::new("Chantelle Barajas", 20, 8, 22, 969.22),
        mall::floor::store::employee::Employee::new("Hywel Rudd", 49, 12, 22, 695.74),
        mall::floor::store::employee::Employee::new("Marianne Beasley", 55, 8, 14, 767.83),
    ];
    let supermaket_emp = vec![
        mall::floor::store::employee::Employee::new("Amara Schaefer", 23, 9, 14, 796.21),
        mall::floor::store::employee::Employee::new("Yara Wickens", 39, 9, 14, 853.42),
        mall::floor::store::employee::Employee::new("Tomi Boyer", 64, 9, 14, 881.83),
        mall::floor::store::employee::Employee::new("Greta Dickson", 42, 9, 14, 775.10),
        mall::floor::store::employee::Employee::new("Caroline Finnegan", 41, 9, 14, 702.92),
        mall::floor::store::employee::Employee::new("Indiana Baxter", 33, 13, 20, 991.71),
        mall::floor::store::employee::Employee::new("Jadine Page", 48, 13, 20, 743.21),
        mall::floor::store::employee::Employee::new("Husna Ryan", 43, 13, 20, 655.75),
        mall::floor::store::employee::Employee::new("Tyler Hunt", 63, 13, 20, 668.25),
        mall::floor::store::employee::Employee::new("Dahlia Caldwell", 56, 13, 20, 781.38),
        mall::floor::store::employee::Employee::new("Chandler Mansell", 20, 19, 24, 656.75),
        mall::floor::store::employee::Employee::new("Mohsin Mcgee", 30, 19, 24, 703.83),
        mall::floor::store::employee::Employee::new("Antoine Goulding", 45, 19, 24, 697.12),
        mall::floor::store::employee::Employee::new("Mark Barnard", 53, 19, 24, 788.81),
    ];

    let ground_stores = vec![
        store::Store::new("Footzo", 50, footzo_emp),
        store::Store::new("Swashion", 43, swashion_emp),
    ];
    let food_stores = vec![
        store::Store::new("PizBite", 60, pizbite_emp),
        store::Store::new("Chillout Grill", 50, grill_emp),
        store::Store::new("Sumo Food", 30, sumo_emp),
    ];
    let supermarket = vec![store::Store::new("Pretail", 950, supermaket_emp)];

    let floors = vec![
        floor::Floor::new("Ground Floor", ground_stores, 300),
        floor::Floor::new("Food Floor", food_stores, 500),
        floor::Floor::new("Supermarket", supermarket, 1000),
    ];

    let mut mall_la_vie = mall::Mall::new("La Vie Funchal", secs, floors);

    // println!("{:?}", &mall_la_vie);

    //returns the biggest store
    // println!("{:?}", biggest_store(mall_la_vie.clone()));

    // //returns the list with the highest paid employees
    // println!("{:?}", highest_paid_employee(mall_la_vie.clone()));

    // //returns the number of employees
    // println!("{:?}", nbr_of_employees(mall_la_vie.clone()));

    // //fires old securities
    // fire_old_securities(&mut mall_la_vie);
    // println!("{:?}", fire_old_securities(&mut mall_la_vie).guards);

    // //checks if it is needed to add securities
    // check_for_securities(
    //     &mut mall_la_vie,
    //     vec![
    //         mall::guard::Guard::new("Peter Solomons", 45, 20),
    //         mall::guard::Guard::new("William Charles", 32, 10),
    //         mall::guard::Guard::new("Leonardo Changretta", 23, 0),
    //         mall::guard::Guard::new("Vlad Levi", 38, 8),
    //         mall::guard::Guard::new("Faruk Berkai", 40, 15),
    //         mall::guard::Guard::new("Chritopher Smith", 35, 9),
    //         mall::guard::Guard::new("Jason Mackie", 26, 2),
    //         mall::guard::Guard::new("Kenzie Mair", 34, 8),
    //         mall::guard::Guard::new("Bentley Larson", 33, 10),
    //         mall::guard::Guard::new("Ray Storey", 37, 12),
    //     ],
    // );
    // println!("{:?}", check_for_securities(
    //     &mut mall_la_vie,
    //     vec![
    //         mall::guard::Guard::new("Peter Solomons", 45, 20),
    //         mall::guard::Guard::new("William Charles", 32, 10),
    //         mall::guard::Guard::new("Leonardo Changretta", 23, 0),
    //         mall::guard::Guard::new("Vlad Levi", 38, 8),
    //         mall::guard::Guard::new("Faruk Berkai", 40, 15),
    //         mall::guard::Guard::new("Chritopher Smith", 35, 9),
    //         mall::guard::Guard::new("Jason Mackie", 26, 2),
    //         mall::guard::Guard::new("Kenzie Mair", 34, 8),
    //         mall::guard::Guard::new("Bentley Larson", 33, 10),
    //         mall::guard::Guard::new("Ray Storey", 37, 12),
    //     ],
    // ));
    // //raises or cuts the salary  of every employee
    // cut_or_raise(&mut mall_la_vie);

    // println!("{:?}", cut_or_raise(&mut mall_la_vie));
}
