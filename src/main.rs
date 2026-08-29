use std::io;

fn food (food_list: &mut Vec<String>) {
    
    let mut food_rs = 0.0;
    
    println!("========== Food ==========");
    
    loop {
        
            println!(r#"
            Enter Input:
            [1] View Expense
            [2] Add Expense
            [3] Exit
            "#);
                
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on food:{}rs",food_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                food_rs += rs;
                println!("New Ammount:{}rs",food_rs);
                let date = "'date'"; // will work on importing date later
                food_list.push(format!("You spent Rs{} on food on {}", food_rs, date))
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn utility () {
    
    let mut utility_rs = 0.0;

    println!("========== Utility ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on utilities:{}rs",utility_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                utility_rs += rs;
                println!("New Ammount:{}rs",utility_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn transport () {
    
    let mut transport_rs = 0.0;

    println!("========== Transport ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on transport:{}rs",transport_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                transport_rs += rs;
                println!("New Ammount:{}rs",transport_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn shopping () {
    
    let mut shopping_rs = 0.0;

    println!("========== Shopping ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on shopping:{}rs",shopping_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                shopping_rs += rs;
                println!("New Ammount:{}rs",shopping_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn education () {
    
    let mut education_rs = 0.0;

    println!("========== Education ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on education:{}rs",education_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                education_rs += rs;
                println!("New Ammount:{}rs",education_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn entertainment () {
    
    let mut entertainment_rs = 0.0;

    println!("========== Entertainment ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on entertainment:{}rs",entertainment_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                entertainment_rs += rs;
                println!("New Ammount:{}rs",entertainment_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn bpac () {
    
    let mut bpac_rs = 0.0;
    
    println!("==========Beauty and Personal Care ==========");
    
    loop {
        
        println!(r#"
        Enter Input:
        [1] View Expense
        [2] Add Expense
        [3] Exit
        "#);
        
    let mut input = String::new();
    println!("Enter Input");
    io::stdin().read_line(&mut input).unwrap();
    
        match input.trim() {
            "1" => {
                println!("Current Expense on Beauty and Peronal Care:{}rs",bpac_rs);
            } "2" => {
                let mut rs = String::new();
                println!("Enter Ammount:");
                io::stdin().read_line(&mut rs).unwrap();
                let rs: f64 = rs.trim().parse().unwrap();
                bpac_rs += rs;
                println!("New Ammount:{}rs",bpac_rs);
            } "3" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn add_expense(food_list: &mut Vec<String>) {
    
    // categories: food
    //             utility
    //             transport
    //             shopping
    //             education
    //             Enertainment
    //             Beauty and Peronal Care (bapc)
    
    loop {
        
        println!(r#"
        Select Expense Type
        [1] Food
        [2] Utility
        [3] Trnasport
        [4] Shopping
        [5] Education
        [6] Entertainment
        [7] Beauty and Personal Care
        [8] Exit
        "#);
        
        let mut input = String::new();
        println!("Enter Input");
        io::stdin().read_line(&mut input).unwrap();
            match input.trim(){
            "1" => {
                food(food_list);
            } "2" => {
                utility();
            } "3" => {
                transport();
            } "4" => {
                shopping();
            } "5" => {
                education();
            } "6" => {
                entertainment();
            } "7" => {
                bpac();
            } "8" => {
                println!("Exiting...");
                break;
            } _ => {
                println!("Invalid Input")
            }
        }
    }

}

fn view_expense(food_list: &Vec<String>) {
    
    loop {

        println!(r#"
         Select Expense Type to view
         [1] Food
         [2] Utility
         [3] Trnasport
         [4] Shopping
         [5] Education
         [6] Entertainment
         [7] Beauty and Personal Care
         [8] Exit
         "#); 
    
        let mut input = String::new();
        println!("Enter Input");
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                for (index, foods) in food_list.iter().enumerate() {
                    println!("{}-{}",index+1, foods)
                }
            }"8" => {
                println!("Exiting...");
                break;
            }_ => {
                println!("Invalid Input")
            } 
        }
    }
}

fn total_spending() {}

fn search_expense() {}

fn main() {

    // arrays
    let mut food_list: Vec<String> = Vec::new();
    let mut utility_list: Vec<String> = Vec::new();
    let mut transport_list: Vec<String> = Vec::new();
    let mut shopping_list: Vec<String> = Vec::new();
    let mut education_list: Vec<String> = Vec::new();
    let mut entertainment_list: Vec<String> = Vec::new();
    let mut bpac_list: Vec<String> = Vec::new();

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
        ║   [5]  Exit                                                                          ║
        ║                                                                                      ║
        ╠══════════════════════════════════════════════════════════════════════════════════════╣
        ║                                                                                      ║
        ║   Select an option:                                                                  ║
        ║                                                                                      ║
        ╚══════════════════════════════════════════════════════════════════════════════════════╝
        "#
    );
    let mut input = String::new();
    println!("Select Option:");
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            add_expense(&mut food_list);
        } "2" => {
            view_expense(&food_list);
        } "3" => {
            total_spending();
        } "4" => {
            search_expense();
        } "5" => {
            println!("Exiting...");
            break;
        } _ => {
            println!("Invalid Input");
        }
    }
    }
}

