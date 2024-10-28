pub mod mall;
pub use mall::*;
pub use mall::floor::*;
pub use mall::floor::*;
pub use crate::store::*;
pub use crate::employee::Employee;
pub use mall::guard::*;

pub fn biggest_store(mall: Mall) -> Store {
    let mut biggest_store = &mall.floors[0].stores[0];
    for floor in &mall.floors {
        for store in &floor.stores {
            if store.square_meters > biggest_store.square_meters {
                biggest_store = store;
            }
        }
    }
    biggest_store.clone()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee>  {
    let mut highest_paid_employee = &mall.floors[0].stores[0].employees[0];
    let mut highest_paid_employees : Vec<Employee> = Vec::new();
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees{
                if employee.salary > highest_paid_employee.salary {
                    highest_paid_employee = employee;
                }
            }


           
        }
    }
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees{
                if employee.salary == highest_paid_employee.salary {
                    highest_paid_employees.push(highest_paid_employee.clone());
                }

            } 

           
        }
    }
    


    highest_paid_employees
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut number : usize =0;
    for floor in &mall.floors {
        for store in &floor.stores {
                number+= store.employees.len();
        }
    }
    number + mall.guards.len()
}

pub fn check_for_securities( mall: &mut Mall, add_guards: Vec<Guard>) {
    let mut total_floor_size : usize = 0;
    let mut total_guards : usize = 0;

    for floor in &mall.floors {
            total_floor_size += floor.size_limit as usize;
    }
    if total_floor_size %  200 == 0 {
        total_guards = total_floor_size /  200;
    }else{
        total_guards = total_floor_size /  200 + 1;
    }
    let number_gards_to_add = total_guards - mall.guards.len();

    if number_gards_to_add > 0 {
        for i in 0..number_gards_to_add{

            mall.guards.push(add_guards[i].clone());
        }
    }

}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees{
                if employee.working_hours.1 as f64 - employee.working_hours.0 as f64 >= 10.0{
                    employee.salary = employee.salary + employee.salary*0.1;
                }else{
                    employee.salary = employee.salary - employee.salary*0.1;
                }
            }
           
        }
    }
  
}
