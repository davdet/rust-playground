use std::collections::HashMap;

pub fn hash_maps() {
    let mut empl_dept = HashMap::new();

    loop {
        println!("WELCOME!");
        println!("1. Add employee to department");
        println!("2. List all employees");
        println!("3. List department employees");
        println!("4. Quit");
        println!(">");

        let mut opt = String::new();
        std::io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");
        let opt = opt.chars().nth(0).unwrap();

        match opt {
            '1' => add_eployee(&mut empl_dept),
            '2' => list_all_employees(&mut empl_dept),
            '3' => list_department_employees(&mut empl_dept),
            '4' => {
                println!("Quitting...");
                break;
            }
            _ => println!("Select an option from 1-4."),
        }
    }
}

fn add_eployee(empl_dept: &mut HashMap<String, String>) {
    println!("Add employee");

    println!("Enter employee's name: ");
    let mut employee_name = String::new();
    std::io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");
    let employee_name = employee_name.trim().to_string();

    println!("Enter employee's department: ");
    let mut employee_dept = String::new();
    std::io::stdin()
        .read_line(&mut employee_dept)
        .expect("Failed to read line");
    let employee_dept = employee_dept.trim().to_string();

    empl_dept.entry(employee_name).or_insert(employee_dept);
    println!("{:?}", empl_dept);
}

fn list_all_employees(empl_dept: &mut HashMap<String, String>) {
    println!("List all employees");
}

fn list_department_employees(empl_dept: &mut HashMap<String, String>) {
    println!("List department employees");
}
