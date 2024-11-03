//CRUD Application
use std::io;
fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!(">>>>>>>Please try again");
    };
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    } else {
        Some(input)
    }

}

enum Manager {
    AddStudent,
    ViewStudent,
    UpdateStudent,
    DeleteStudent
}
impl Manager {
    fn show(){
        println!("*****---------****");
        println!("");
        println!("--Manager UI----");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("---------");
        println!("=> Please enter your choices: ");
        println!("*****---------****");
    }
    fn choice(input:&str) ->  Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::UpdateStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    loop {
        Manager::show();
        let input = get_input().expect(">>>>>>>Please enter correct data input");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => (),
            Some(Manager::ViewStudent) => (),
            Some(Manager::UpdateStudent) => (),
            Some(Manager::DeleteStudent) => (),
            None => return,
        }
    }
}
