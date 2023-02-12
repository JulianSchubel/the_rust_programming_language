/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all
people in the company by department, sorted alphabetically. */

use std::io::{ self, Write };
use std::collections::HashMap;

fn get_input() -> u32 {
    println!("1) Add an employee to a department.");
    println!("2) Retrieve all employees in a department.");
    println!("3) Retreive all employees.");
    println!("4) Exit.");
    print!("Select an option from the list [1,2,3,4]: ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_)    => (),
        Err(_)   => {
            println!("Failed to receive input");
            return 4;
        }
    }
    let mut input: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Expected an integer.");
            get_input()
        }
    };
    if input < 1 || input > 4 {
        println!("Invalid input. Expected an integer between [1, 4]");
        input = get_input();
    }
    input
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let x = get_input();
        if x == 1 {
            /* Add an employee to the company */
            /* Get the employee name */
            print!("Enter the employee name: ");
            io::stdout().flush().unwrap();
            let mut employee_name: String = String::new();
            match io::stdin().read_line(& mut employee_name) {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to receive input");
                    return;
                }
            }
            employee_name = employee_name.trim().parse().unwrap();

            /* Get the department name */
            print!("Enter the department name: ");
            io::stdout().flush().unwrap();
            let mut department_name: String = String::new();
            match io::stdin().read_line(& mut department_name) {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to receive input");
                    return;
                }
            }
            department_name = department_name.trim().parse().unwrap();

            if company.contains_key(&department_name) {
                match company.get_mut(&department_name) {
                    Some(x) => x.push(employee_name),
                    _ => (),
                }
            } else {
                let mut employees: Vec<String> = Vec::new();
                employees.push(employee_name);
                company.insert(department_name, employees);
            }
        } else if x == 2 {
            /* List all employees in a department */
            print!("Enter the department name: ");
            io::stdout().flush().unwrap();
            let mut department_name: String = String::new();
            /* Don't want to keep on writing out the full match expression; using expect */
            io::stdin().read_line(& mut department_name).expect("Failed to receive input");
            department_name = department_name.trim().parse().unwrap();
            if company.contains_key(&department_name) {
                /* Retrieve a reference to the vector and print its contents */
                println!("--------------------------------------------------");
                println!("Employees in {department_name}:");
                if let Some(employees) = company.get(&department_name) {
                    for employee in employees {
                        println!("\t{employee}");
                    }
                    println!("--------------------------------------------------\n");
                }
            }
        } else if x == 3 {
            /* List all employess in the company */
            println!("--------------------------------------------------");
            println!("All persons employed by department name:");
            for (department, employees) in &company {
                println!("Employees in {department}:");
                for employee in employees {
                    println!("\t{employee}");
                }
                println!("");
            }
            println!("--------------------------------------------------\n");
        } else if x == 4{
            return;
        }    
    }
}
