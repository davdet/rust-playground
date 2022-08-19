pub fn hash_maps() {
    println!("WELCOME!");
    println!("1. Add employee to department");
    println!("2. List all employees");
    println!("3. List department employees");
    println!(">");

    let mut opt = String::new();
    std::io::stdin()
        .read_line(&mut opt)
        .expect("Failed to read line");

    match opt {
        String::from("1") => add_eployee(),
        String::from("2") => list_all_employees(),
        String::from("3") => list_department_employees(),
    }

    println!("{}", opt);
}

fn add_eployee() {
    println!("Add employee");
}

fn list_all_employees() {
    println!("List all employees");
}

fn list_department_employees() {
    println!("List department employees");
}
