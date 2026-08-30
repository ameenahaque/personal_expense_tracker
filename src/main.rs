use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    input.trim().to_string()
}


fn food(food_list: &mut Vec<String>, food_rs: &mut f64) {
    
    println!(r#"
    ╔═════════════════════════════════════════════════════════╗
    ║                          FOOD                           ║
    ╚═════════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
             ╔══════════════════════════════╗
             ║        SELECT OPTION         ║
             ╠══════════════════════════════╣
             ║ [1] View Expense             ║
             ║ [2] Add Expense              ║
             ║ [3] Exit                     ║
             ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!("Current Expense on Food: Rs{:.2}", *food_rs);
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *food_rs += amount;

                        let date = "Date will be added later";

                        food_list.push(format!(
                            "You spent Rs{:.2} on Food on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *food_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn utility(utility_list: &mut Vec<String>, utility_rs: &mut f64) {
    
    println!(r#"
    ╔════════════════════════════════════════════════════════╗
    ║                       UTILITY                          ║
    ╚════════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
             ╔══════════════════════════════╗
             ║        SELECT OPTION         ║
             ╠══════════════════════════════╣
             ║ [1] View Expense             ║
             ║ [2] Add Expense              ║
             ║ [3] Exit                     ║
             ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!("Current Expense on Utility: Rs{:.2}", *utility_rs);
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *utility_rs += amount;

                        let date = "Date will be added later";

                        utility_list.push(format!(
                            "You spent Rs{:.2} on Utility on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *utility_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn transport(transport_list: &mut Vec<String>, transport_rs: &mut f64) {
    
    println!(r#"
    ╔═════════════════════════════════════════════════════════╗
    ║                        TRANSPORT                        ║
    ╚═════════════════════════════════════════════════════════╝

    "#);
    
    loop {
        println!(r#"
           ╔══════════════════════════════╗
           ║        SELECT OPTION         ║
           ╠══════════════════════════════╣
           ║ [1] View Expense             ║
           ║ [2] Add Expense              ║
           ║ [3] Exit                     ║
           ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!("Current Expense on Transport: Rs{:.2}", *transport_rs);
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *transport_rs += amount;

                        let date = "Date will be added later";

                        transport_list.push(format!(
                            "You spent Rs{:.2} on Transport on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *transport_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn shopping(shopping_list: &mut Vec<String>, shopping_rs: &mut f64) {
    
    println!(r#"
    ╔═════════════════════════════════════════════════════════╗
    ║                         SHOPPING                        ║
    ╚═════════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
           ╔══════════════════════════════╗
           ║        SELECT OPTION         ║
           ╠══════════════════════════════╣
           ║ [1] View Expense             ║
           ║ [2] Add Expense              ║
           ║ [3] Exit                     ║
           ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!("Current Expense on Shopping: Rs{:.2}", *shopping_rs);
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *shopping_rs += amount;

                        let date = "Date will be added later";

                        shopping_list.push(format!(
                            "You spent Rs{:.2} on Shopping on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *shopping_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn education(education_list: &mut Vec<String>, education_rs: &mut f64) {
    
    println!(r#"
    ╔═══════════════════════════════════════════════════════╗
    ║                      EDUCATION                        ║
    ╚═══════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
           ╔══════════════════════════════╗
           ║        SELECT OPTION         ║
           ╠══════════════════════════════╣
           ║ [1] View Expense             ║
           ║ [2] Add Expense              ║
           ║ [3] Exit                     ║
           ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!("Current Expense on Education: Rs{:.2}", *education_rs);
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *education_rs += amount;

                        let date = "Date will be added later";

                        education_list.push(format!(
                            "You spent Rs{:.2} on Education on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *education_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn entertainment(entertainment_list: &mut Vec<String>,entertainment_rs: &mut f64) {
    
    println!(r#"
    ╔══════════════════════════════════════════════════════╗
    ║                    ENTERTAINMENT                     ║
    ╚══════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
           ╔══════════════════════════════╗
           ║        SELECT OPTION         ║
           ╠══════════════════════════════╣
           ║ [1] View Expense             ║
           ║ [2] Add Expense              ║
           ║ [3] Exit                     ║
           ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!(
                    "Current Expense on Entertainment: Rs{:.2}",
                    *entertainment_rs
                );
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *entertainment_rs += amount;

                        let date = "Date will be added later";

                        entertainment_list.push(format!(
                            "You spent Rs{:.2} on Entertainment on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *entertainment_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn bapc(bapc_list: &mut Vec<String>,bapc_rs: &mut f64) {
    
    println!(r#"
    ╔════════════════════════════════════════════════════════════╗
    ║                  BEAUTY AND PERSONAL CARE                  ║
    ╚════════════════════════════════════════════════════════════╝
    "#);
    
    loop {
        println!(r#"
           ╔══════════════════════════════╗
           ║        SELECT OPTION         ║
           ╠══════════════════════════════╣
           ║ [1] View Expense             ║
           ║ [2] Add Expense              ║
           ║ [3] Exit                     ║
           ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                println!(
                    "Current Expense on Beauty and Personal Care: Rs{:.2}",
                    *bapc_rs
                );
            }

            "2" => {
                println!("Enter Amount:");
                let rs = read_input();

                match rs.parse::<f64>() {
                    Ok(amount) => {
                        if amount <= 0.0 {
                            println!("Please enter an amount greater than 0.");
                            continue;
                        }

                        *bapc_rs += amount;

                        let date = "Date will be added later";

                        bapc_list.push(format!(
                            "You spent Rs{:.2} on Beauty and Personal Care on {}",
                            amount, date
                        ));

                        println!("New Amount: Rs{:.2}", *bapc_rs);
                    }

                    Err(_) => {
                        println!("Invalid Amount. Please enter a number.");
                    }
                }
            }

            "3" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn add_expense(
    food_list: &mut Vec<String>,
    food_rs: &mut f64,
    utility_list: &mut Vec<String>,
    utility_rs: &mut f64,
    transport_list: &mut Vec<String>,
    transport_rs: &mut f64,
    shopping_list: &mut Vec<String>,
    shopping_rs: &mut f64,
    education_list: &mut Vec<String>,
    education_rs: &mut f64,
    entertainment_list: &mut Vec<String>,
    entertainment_rs: &mut f64,
    bapc_list: &mut Vec<String>,
    bapc_rs: &mut f64,
) {
    
    loop {
        println!(r#"
        ╔═══════════════════════════════════════════════════╗
        ║              SELECT EXPENSE TYPE                  ║
        ║              -------------------                  ║
        ╠═══════════════════════════════════════════════════╣
        ║ [1] Food                                          ║
        ║ [2] Utility                                       ║
        ║ [3] Transport                                     ║
        ║ [4] Shopping                                      ║
        ║ [5] Education                                     ║
        ║ [6] Entertainment                                 ║
        ║ [7] Beauty and Personal Care                      ║
        ║ [8] Exit                                          ║
        ╚═══════════════════════════════════════════════════╝
        "#);

        println!(r#"
        ╔══════════════════════════════╗
        ║        SELECT OPTION         ║
        ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => food(food_list, food_rs),

            "2" => utility(utility_list, utility_rs),

            "3" => transport(transport_list, transport_rs),

            "4" => shopping(shopping_list, shopping_rs),

            "5" => education(education_list, education_rs),

            "6" => entertainment(entertainment_list, entertainment_rs),

            "7" => bapc(bapc_list, bapc_rs),

            "8" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn show_category(category_name: &str, expense_list: &Vec<String>) {
    
    match category_name {
        "FOOD" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║        FOOD EXPENSES         ║
            ╚══════════════════════════════╝
            "#);
        }

        "UTILITY" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║       UTILITY EXPENSES       ║
            ╚══════════════════════════════╝
            "#);
        }

        "TRANSPORT" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      TRANSPORT EXPENSES      ║
            ╚══════════════════════════════╝
            "#);
        }

        "SHOPPING" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      SHOPPING EXPENSES       ║
            ╚══════════════════════════════╝
            "#);
        }

        "EDUCATION" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      EDUCATION EXPENSES      ║
            ╚══════════════════════════════╝
            "#);
        }

        "ENTERTAINMENT" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║    ENTERTAINMENT EXPENSES    ║
            ╚══════════════════════════════╝
            "#);
        }

        "BAPC" => {
            println!(r#"
            ╔══════════════════════════════════════╗
            ║       BEAUTY AND PERSONAL CARE       ║
            ╚══════════════════════════════════════╝
            "#);
        }

        _ => {
            println!(r#"
            ╔══════════════════════════════╗
            ║           EXPENSES           ║
            ╚══════════════════════════════╝
            "#);
        }
    }

    if expense_list.is_empty() {
        println!("No expenses found.");
    } else {
        for (index, expense) in expense_list.iter().enumerate() {
            println!("{}. {}", index + 1, expense);
        }
    }
}


fn view_expense(
    food_list: &Vec<String>,
    utility_list: &Vec<String>,
    transport_list: &Vec<String>,
    shopping_list: &Vec<String>,
    education_list: &Vec<String>,
    entertainment_list: &Vec<String>,
    bapc_list: &Vec<String>,
) {
    
    loop {
        println!(r#"
        ╔══════════════════════════════════════════════════════════════╗
        ║                         VIEW EXPENSES                        ║
        ╚══════════════════════════════════════════════════════════════╝
        "#);

        println!(r#"
        ╔═══════════════════════════════════════╗
        ║      SELECT EXPENSE TYPE TO VIEW      ║
        ╠═══════════════════════════════════════╣
        ║ [1] Food                              ║
        ║ [2] Utility                           ║
        ║ [3] Transport                         ║
        ║ [4] Shopping                          ║
        ║ [5] Education                         ║
        ║ [6] Entertainment                     ║
        ║ [7] Beauty and Personal Care          ║
        ║ [8] Exit                              ║
        ╚═══════════════════════════════════════╝
        "#);

        println!(r#"
        ╔══════════════════════════════╗
        ║        SELECT OPTION         ║
        ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                show_category("FOOD", food_list);
            }

            "2" => {
                show_category("UTILITY", utility_list);
            }

            "3" => {
                show_category("TRANSPORT", transport_list);
            }

            "4" => {
                show_category("SHOPPING", shopping_list);
            }

            "5" => {
                show_category("EDUCATION", education_list);
            }

            "6" => {
                show_category("ENTERTAINMENT", entertainment_list);
            }

            "7" => {
                show_category("BAPC", bapc_list);
            }

            "8" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn total_spending(
    food_rs: f64,
    utility_rs: f64,
    transport_rs: f64,
    shopping_rs: f64,
    education_rs: f64,
    entertainment_rs: f64,
    bapc_rs: f64,
) {
    
    let total =
        food_rs
        + utility_rs
        + transport_rs
        + shopping_rs
        + education_rs
        + entertainment_rs
        + bapc_rs;

    println!(r#"
    ╔══════════════════════════════════════╗
    ║           TOTAL SPENDING             ║
    ╚══════════════════════════════════════╝
    "#);

    println!("Food: Rs{:.2}", food_rs);
    println!("Utility: Rs{:.2}", utility_rs);
    println!("Transport: Rs{:.2}", transport_rs);
    println!("Shopping: Rs{:.2}", shopping_rs);
    println!("Education: Rs{:.2}", education_rs);
    println!("Entertainment: Rs{:.2}", entertainment_rs);
    println!("(BAPC): Rs{:.2}", bapc_rs);

    println!("-----------------------------------");
    println!("TOTAL SPENDING: Rs{:.2}", total);
}


fn list_edit(category_name: &str,expense_list: &mut Vec<String>,expense_rs: &mut f64) {
    
    match category_name {
        "FOOD" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║        FOOD EXPENSES         ║
            ╚══════════════════════════════╝
            "#);
        }

        "UTILITY" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║       UTILITY EXPENSES       ║
            ╚══════════════════════════════╝
            "#);
        }

        "TRANSPORT" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      TRANSPORT EXPENSES      ║
            ╚══════════════════════════════╝
            "#);
        }

        "SHOPPING" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      SHOPPING EXPENSES       ║
            ╚══════════════════════════════╝
            "#);
        }

        "EDUCATION" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║      EDUCATION EXPENSES      ║
            ╚══════════════════════════════╝
            "#);
        }

        "ENTERTAINMENT" => {
            println!(r#"
            ╔══════════════════════════════╗
            ║    ENTERTAINMENT EXPENSES    ║
            ╚══════════════════════════════╝
            "#);
        }

        "BAPC" => {
            println!(r#"
            ╔══════════════════════════════════════╗
            ║       BEAUTY AND PERSONAL CARE       ║
            ╚══════════════════════════════════════╝
            "#);
        }

        _ => {
            println!(r#"
            ╔══════════════════════════════╗
            ║           EXPENSES           ║
            ╚══════════════════════════════╝
            "#);
        }
    }

    if expense_list.is_empty() {
        println!("No Expenses added yet")
    } else {
            println!("Select Expense Index to Delete");
        for (index, expense) in expense_list.iter().enumerate() {
            println!("{}. {}", index + 1, expense);
        }
        
        let length = expense_list.len();
        let x= read_input();
        let x: usize = x.trim().parse().unwrap();
        
        if x == 0 || x > length {
            println!("Invalid Index");
        } 
            else {
            let expense = &expense_list[x-1];
            let part = expense.split(" ").nth(2).unwrap();
            let part = part.trim_start_matches("Rs");
            let part: f64 = part.trim().parse().unwrap();
            println!("{}", part);
            *expense_rs -= part;
            expense_list.remove(x - 1);
            println!("Expense removed successfully");
        }
    }
}


fn edit_expense(
    food_list: &mut Vec<String>,
    food_rs: &mut f64,
    utility_list: &mut Vec<String>,
    utility_rs: &mut f64,
    transport_list: &mut Vec<String>,
    transport_rs: &mut f64,
    shopping_list: &mut Vec<String>,
    shopping_rs: &mut f64,
    education_list: &mut Vec<String>,
    education_rs: &mut f64,
    entertainment_list: &mut Vec<String>,
    entertainment_rs: &mut f64,
    bapc_list: &mut Vec<String>,
    bapc_rs: &mut f64,
) {
    
    loop {
        println!(r#"
        ╔══════════════════════════════════════════════════════════════╗
        ║                         EDIT EXPENSES                        ║
        ╚══════════════════════════════════════════════════════════════╝
        "#);

        println!(r#"
        ╔═══════════════════════════════════════╗
        ║      SELECT EXPENSE TYPE TO EDIT      ║
        ╠═══════════════════════════════════════╣
        ║ [1] Food                              ║
        ║ [2] Utility                           ║
        ║ [3] Transport                         ║
        ║ [4] Shopping                          ║
        ║ [5] Education                         ║
        ║ [6] Entertainment                     ║
        ║ [7] Beauty and Personal Care          ║
        ║ [8] Exit                              ║
        ╚═══════════════════════════════════════╝
        "#);

        println!(r#"
        ╔══════════════════════════════╗
        ║        SELECT OPTION         ║
        ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                list_edit("FOOD", food_list, food_rs);
            }

            "2" => {
                list_edit("UTILITY", utility_list, utility_rs);
            }

            "3" => {
                list_edit("TRANSPORT", transport_list, transport_rs);
            }

            "4" => {
                list_edit("SHOPPING", shopping_list, shopping_rs);
            }

            "5" => {
                list_edit("EDUCATION", education_list, education_rs);
            }

            "6" => {
                list_edit("ENTERTAINMENT", entertainment_list, entertainment_rs);
            }

            "7" => {
                list_edit("BAPC", bapc_list, bapc_rs);
            }

            "8" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid Input");
            }
        }
    }
}


fn search_expense() {
    println!("Search Expense feature will be added next.");
}

fn main() {

    // arrays
    let mut food_list: Vec<String> = Vec::new();
    let mut utility_list: Vec<String> = Vec::new();
    let mut transport_list: Vec<String> = Vec::new();
    let mut shopping_list: Vec<String> = Vec::new();
    let mut education_list: Vec<String> = Vec::new();
    let mut entertainment_list: Vec<String> = Vec::new();
    let mut bapc_list: Vec<String> = Vec::new();

    // category totals
    let mut food_rs: f64 = 0.0;
    let mut utility_rs: f64 = 0.0;
    let mut transport_rs: f64 = 0.0;
    let mut shopping_rs: f64 = 0.0;
    let mut education_rs: f64 = 0.0;
    let mut entertainment_rs: f64 = 0.0;
    let mut bapc_rs: f64 = 0.0;

    loop {
        println!(r#"
        ╔══════════════════════════════════════════════════════════════════════════════════════╗
        ║                                                                                      ║
        ║                            PERSONAL EXPENSE TRACKER                                  ║
        ║                                                                                      ║
        ╠══════════════════════════════════════════════════════════════════════════════════════╣
        ║                                                                                      ║
        ║   [1]  Add Expense                                                                   ║
        ║   [2]  View Expenses                                                                 ║
        ║   [3]  Total Spending                                                                ║
        ║   [4]  Search Expenses                                                               ║
        ║   [5]  Edit Expense                                                                  ║
        ║   [6]  Exit                                                                          ║
        ║                                                                                      ║
        ╚══════════════════════════════════════════════════════════════════════════════════════╝
        "#);

        println!(r#"
        ╔══════════════════════════════╗
        ║        SELECT OPTION         ║
        ╚══════════════════════════════╝
        "#);

        let input = read_input();

        match input.as_str() {
            "1" => {
                add_expense(
                    &mut food_list,
                    &mut food_rs,
                    &mut utility_list,
                    &mut utility_rs,
                    &mut transport_list,
                    &mut transport_rs,
                    &mut shopping_list,
                    &mut shopping_rs,
                    &mut education_list,
                    &mut education_rs,
                    &mut entertainment_list,
                    &mut entertainment_rs,
                    &mut bapc_list,
                    &mut bapc_rs,
                );
            }

            "2" => {
                view_expense(
                    &food_list,
                    &utility_list,
                    &transport_list,
                    &shopping_list,
                    &education_list,
                    &entertainment_list,
                    &bapc_list,
                );
            }

            "3" => {
                total_spending(
                    food_rs,
                    utility_rs,
                    transport_rs,
                    shopping_rs,
                    education_rs,
                    entertainment_rs,
                    bapc_rs,
                );
            }

            "4" => {
                search_expense();
            }

            "5" => {
                edit_expense(
                    &mut food_list,
                    &mut food_rs,
                    &mut utility_list,
                    &mut utility_rs,
                    &mut transport_list,
                    &mut transport_rs,
                    &mut shopping_list,
                    &mut shopping_rs,
                    &mut education_list,
                    &mut education_rs,
                    &mut entertainment_list,
                    &mut entertainment_rs,
                    &mut bapc_list,
                    &mut bapc_rs,
                );
            }

            "6" => {
                println!("Thank you for using our Personal Expense Tracker!");
                break;
            }

            _ => {
                println!("Invalid Input. Please try again.");
            }
        }
    }
}
