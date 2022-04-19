use core::time;
use std::{
    collections::HashMap,
    io::{self, Write},
    thread,
};

fn main() {
    println!("Company Directory");
    let mut dir: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Input a command");
        println!("Add: Add Employee to a Department");
        println!("Get: Retrieve Employee List");
        println!("Quit: Quit the Program");

        let mut cmd = String::new();
        print!("Input: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Error: Unable to read input");

        match cmd.as_str().trim() {
            "Add" => loop {
                println!("Input: Add <Employee> to <Department>");
                println!("Back: Go Back");
                let mut cmd = String::new();
                print!("Input: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("Error: Unable to read input");

                if cmd.trim() == "Back" {
                    break;
                }

                let cmd: Vec<&str> = cmd.trim().split_whitespace().collect();
                if cmd.len() != 4 {
                    println!("Error: Please Correctly Enter the command\n");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                } else if cmd[0] != "Add" {
                    println!("Error: Please Correctly Enter the command\n");
                    thread::sleep(time::Duration::from_secs(1));
                    println!("Error: You Forgot 'Add'");
                    continue;
                } else if cmd[2] != "to" {
                    println!("Error: Please Correctly Enter the command\n");
                    println!("Error: You Forgot 'to'\n");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                }

                let emp_name = cmd[1];
                let dept_name = cmd[3];

                if dir.contains_key(dept_name) {
                    match dir
                        .get(dept_name)
                        .unwrap()
                        .iter()
                        .find(|e| e.to_string() == *emp_name.to_string())
                    {
                        Some(_) => println!("Employee Already Exists.\n"),
                        None => {
                            dir.entry(dept_name.to_string())
                                .or_insert(Vec::new())
                                .push(emp_name.to_string());
                            println!("{} added to {} Department\n", emp_name, dept_name);
                            thread::sleep(time::Duration::from_secs(1));
                        }
                    };
                } else {
                    dir.entry(dept_name.to_string())
                        .or_insert(Vec::new())
                        .push(emp_name.to_string());
                    println!("{} added to {} Department\n", emp_name, dept_name);
                    thread::sleep(time::Duration::from_secs(1));
                }
            },
            "Get" => loop {
                println!("Get Employee List");
                println!("Input: <Department> or All");
                println!("Back: Go Back");

                let mut cmd = String::new();
                print!("Input: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("Error: Unable to read input");

                if cmd.trim() == "Back" {
                    break;
                }

                match cmd.as_str().trim() {
                    "All" => {
                        for (dept, emp) in dir.iter_mut() {
                            emp.sort();
                            println!("\n{} Department:\n{}", dept, emp.join("\n"))
                        }

                        println!();
                        thread::sleep(time::Duration::from_secs(1));
                    }
                    dept => {
                        if dir.contains_key(dept) {
                            println!("{} Department:", dept);
                            for emp in dir.get(dept).unwrap() {
                                println!("{}", emp);
                            }
                            println!();
                            thread::sleep(time::Duration::from_secs(1));
                        } else {
                            println!("{} Department Not Found!\n", dept);
                            thread::sleep(time::Duration::from_secs(1));
                            continue;
                        }
                    }
                }
            },
            "Quit" => break,
            _ => {
                println!("Invalid Command\n");
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}
