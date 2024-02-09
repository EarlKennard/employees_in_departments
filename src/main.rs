use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world! This is a text interface program where you can add employees to/delete from a department in a company.");
    list_of_commands();

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = user_input_str();
        let inputs: Vec<&str> = input.split_whitespace().collect(); // idk why i have to split these up to input and inputs

        match inputs[0] {
            "quit" => break,
            "commands" => list_of_commands(),
            "add" => add_worker(&mut company, inputs[3], inputs[1]),
            "delete" => fire_worker_interface(&mut company, inputs[3], inputs[1]),
            "workers" => {
                if inputs[2] == "company" {
                    sort_workers_in_company(&company);
                } else {
                    sort_workers_in_department(&company, inputs[2]);
                }
            },
            "departments" => sort_departments(&company),
            _ => println!("Wrong command. Please try again."),
        }
    }
}

fn user_input_str() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input = input.trim().to_lowercase().to_string();

    input
}

fn list_of_commands() {
    println!("For now, you can only add, delete, or retrieve a list of people in a department/company. Examples of available commands:
    Add <Worker> to <Department> -> adds a worker to a department. creates a new department if one doesn't already exist
        Example: Add Sally to Engineering
    Delete <Worker> from <Department> -> deletes worker from a department. the worker and the department both have to exist.
        Example: Delete Sally from Engineering
    Workers in <Department> -> alphabetically lists workers from a department, like Engineering. Replace <Department> with the appropriate department.
        Example: Workers in Engineering
    Workers in <Company> -> alphabetically lists workers in the company. For now there is only one company, so only the following command works:
        Example: Workers in company
    Departments in <Company> -> alphabetically lists the departments in a company. For now, there is only one company, so only the following command works:
        Example: Departments in company
    
    To see this list of commands again, type 'commands'.
    Type 'quit' to end the program.
    Please note that worker and department names should only be a single word each.");
}

fn sort_workers_in_department(company: &HashMap<String, Vec<String>>, user_department: &str) {
    if !company.contains_key(user_department) {
        println!("Sorry! That department doesn't exist!");
        return;
    }

    let mut names: Vec<String> = Vec::new();
    for (department, employees) in company {
        if department.to_string() == user_department.to_string() {
            for employee in employees {
                names.push(employee.to_string());
            }
        }
    }

    names.sort_by(|a, b| a.cmp(&b));
    println!("Here are the workers in {}: {:?}", user_department, names);
}

fn sort_workers_in_company(company: &HashMap<String, Vec<String>>) {
    let mut names: Vec<String> = Vec::new();
    for (_department, employees) in company {
        for employee in employees {
            names.push(employee.to_string());
        }
    }

    names.sort_by(|a, b| a.cmp(&b));
    println!("Here are the workers in the company: {:?}", names);
}

fn sort_departments(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<String> = Vec::new();
    for (department, _employees) in company {
        departments.push(department.to_string());
    }

    departments.sort_by(|a, b| a.cmp(&b));
    println!("Here are the departments in the company: {:?}", departments);
}

fn add_worker(company: &mut HashMap<String, Vec<String>>, department: &str, worker: &str) {
    company.entry(department.to_string()).or_insert(Vec::new());
    company.get_mut(department).unwrap().push(worker.to_string());

    println!("You just hired {} to work in {}!", worker, department);
}

fn fire_worker(company: &mut HashMap<String, Vec<String>>, department: &str, worker: &str) -> bool {
    if !company.contains_key(department) {
        return false;
    }

    // unwrap explicitly returns Some, but will actually panic if there is None, which is why it's discouraged. 
    // Good thing the previous if statement quits the fn if there actually is None
    if !company.get(department).unwrap().contains(&worker.to_string()) {
        return false;
    }

    // removes every instance
    company.get_mut(department).unwrap().retain(|x| *x != worker.to_string());

    true
}

// needed cuz the match statement in main only accepts () due to the other arms
// con (i think): linked dependency from fire_worker() to this. not that big of a con imo
fn fire_worker_interface(company: &mut HashMap<String, Vec<String>>, department: &str, worker: &str) {
    let checker = fire_worker(company, department, worker);

    if checker == false {
        println!("Sorry! {} doesn't work in {}!", worker, department);
    } else {
        println!("{} has been fired from {}.", worker, department);
    }
}

/* Notes
1. nah let's just make this simple and expand this if need be later because i'm really confused
2. to make things simpler, names are added in lowercase so i don't have to check for cases. this isn't ideal, but i'll add that functionality later

list of problems:
3. can only do very simple commands
a) thread main panics if the command starts with a valid word (aka doesn't satisfy the else case in the match statement) and the arm reaches for an index position that
is non-existent because the inputs vector in main isn't long enough (aka a command like 'workers inoaisdufoaisjdf' and "workers" arm reaches for inputs[2]).
index out of bounds basically, but i had to describe how it happens
b) names of workers and departments can only be one single word each, or else the program starts breaking down and gets confusing. it's a very big limitation. 
to fix this, i would have to really expand the user input interface
4. currently there is no way to just create a department with no workers. seems like a major and dumb oversight
*/
