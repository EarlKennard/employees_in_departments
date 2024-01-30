use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world! This is a program that adds workers to departments in a company. You can create several companies with several departments and 
    several workers.");

    let mut company_vector: Vec<company> = Vec::new();
}

struct company {
    name: String,
    departments_and_workers: HashMap<String, Vec<String>>,
}

// DRY
fn user_input_string() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input
}

fn list_of_commands() {
    println!("\nHere are the commands:
    create_company: creates a new company. you have to name the company and at least create one department with one worker in it.
    delete_company: delete a company
    create_department: given an existing company, creates a new department and you have to put at least one worker in it.
    delete_department: given an existing company, you can delete an entire department
    hire_worker: given the name of a company and a department in it, you can hire a worker
    fire_worker: given the name of a company and a department in it, you can fire a worker
    companies: show the list of existing companies, sorted alphabetically
    company_details: given an existing company, show the departments listed alphabetically
    department_details: given an existing company and department, show the workers in it sorted alphabetically\n");
}

/* Notes
1. should i initialize some departments in the hashmap?
2. there are no types of workers because that seems too annoying for now

3. commands that the user should be able to do
a) create_company: creates a new company
b) delete_company: deletes a company (idk how to do this)
c) create_department: has to give the name of the company. create a department in that
d) delete_department: has to give the name of company. delete department
e) hire_worker: has to give name of department and company. push worker in vector
f) fire_worker: has to give name of department and company. pop worker from vector
g) companies: show companies sorted alphabetically. not too much information cuz the terminal would bloat, just show names
h) company_details: show details of a company. only name, and the departments sorted alphabetically
i) department_details: given a company, show the workers in department sorted alphabetically
*/
