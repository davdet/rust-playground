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
    let opt = opt.chars().nth(0).unwrap();

    match opt {
        '1' => add_eployee(),
        '2' => list_all_employees(),
        '3' => list_department_employees(),
        _ => panic!(),
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
