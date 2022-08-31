pub mod mall;
pub use crate::guard::Guard;
pub use crate::mall::floor;
pub use crate::mall::floor::store;
pub use crate::mall::Mall;
use mall::floor::store::employee::Employee;
use mall::floor::store::Store;
pub use mall::*;

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
    employees1.pop();
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
}
pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut all_empl = 0;

    for floor in mall.floors {
        for store in floor.stores {
            all_empl += store.employees.len();
        }
    }
    all_empl += mall.guards.len();

    all_empl
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
