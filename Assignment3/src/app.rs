use std::collections::HashMap;
use std::io::{self, Write};
use crate::errors::WalletError;
use crate::logger::{ConsoleLogger, Logger};
use crate::models::User;


pub struct WalletApp {
   users: HashMap<String, User>,
   logger: Box<dyn Logger>,
}


impl WalletApp {
   pub fn new() -> Self {
       let mut users = HashMap::new();
       users.insert("teja".to_string(),User::new("teja", "haha123"));
       users.insert( "jaya".to_string(), User::new("jaya", "123haha"));
       Self {
           users,
           logger: Box::new(ConsoleLogger),
       }
   }

   pub fn run(&mut self) {
    let username = self.read_input("Enter username: ");
    let password = self.read_input("Enter password: ");

    // Temporarily remove the user to avoid borrowing self
    if let Some(mut user) = self.users.remove(&username) {
        match user.verify_password(&password) {
            Ok(_) => {
                self.logger.log(&format!("{} logged in.", username));
                self.menu_loop(&mut user);
                // Put the user back after menu_loop
                self.users.insert(username, user);
            }
            Err(WalletError::IncorrectPassword) => {
                println!("Incorrect password.");
                self.users.insert(username, user); // Put back even on failure
            }
            Err(e) => {
                println!("Authentication error: {:?}", e);
                self.users.insert(username, user); // Put back even on error
            }
        }
    } else {
        println!("User not found.");
    }
}

//    pub fn run(&mut self) {
//        let username = self.read_input("Enter username: ");
//        let password = self.read_input("Enter password: ");
//        if let Some(user) = self.users.get_mut(&username) {
//            match user.verify_password(&password) {
//                Ok(_) => {
//                    self.logger.log(&format!("{} logged in.", username));
//                    self.menu_loop(user);
//                }
//                Err(WalletError::IncorrectPassword) => {
//                    println!("Incorrect password.");
//                }
//                Err(e) => {
//                    println!("Authentication error: {:?}", e);
//                }
//            }
//        } else {
//            println!("User not found.");
//        }
//    }
   fn menu_loop(&mut self, current_user: &mut User) {
       loop {
           println!("\n--- MENU ---");
           println!("1. Check Balance");
           println!("2. Deposit");
           println!("3. Transfer Funds");
           println!("4. Show Transactions");
           println!("5. Exit");
           let choice = self.read_input("Choose an option: ");
           match choice.as_str() {
               "1" => self.check_balance(current_user),
               "2" => self.deposit(current_user),
               "3" => self.transfer(current_user),
               "4" => self.show_history(current_user),
               "5" => {
                   println!("Goodbye!");
                   break;
               }
               _ => println!("Invalid option."),
           }
       }
   }
   fn check_balance(&self, user: &User) {
       println!("Balance: ${:.2}", user.balance);
   }
   fn deposit(&mut self, user: &mut User) {
       let input = self.read_input("Amount to deposit: ");
       if let Ok(amount) = input.parse::<f64>() {
           user.balance += amount;
           user.history.push(format!("Deposited ${}", amount));
           self.logger
               .log(&format!("{} deposited ${}", user.username, amount));
       } else {
           println!("Invalid number.");
       }
   }
   fn transfer(&mut self, from_user: &mut User) {
       let to_username = self.read_input("Send to username: ");
       let input = self.read_input("Amount to send: ");
       if let Some(to_user) = self.users.get_mut(&to_username) {
           if let Ok(amount) = input.parse::<f64>() {
               if from_user.balance < amount {
                   println!("Insufficient funds.");
                   return;
               }
               from_user.balance -= amount;
               to_user.balance += amount;
               from_user
                   .history
                   .push(format!("Sent ${} to {}", amount, to_user.username));
               to_user
                   .history
                   .push(format!("Received ${} from {}", amount, from_user.username));
               self.logger.log(&format!(
                   "{} sent ${} to {}",
                   from_user.username, amount, to_user.username
               ));
               println!("Transfer successful.");
           } else {
               println!("Invalid amount.");
           }
       } else {
           println!("Recipient not found.");
       }
   }
   fn show_history(&self, user: &User) {
       println!("Transaction history for {}:", user.username);
       for entry in &user.history {
           println!("- {}", entry);
       }
   }
   fn read_input(&self, prompt: &str) -> String {
       print!("{}", prompt);
       io::stdout().flush().unwrap();
       let mut s = String::new();
       io::stdin().read_line(&mut s).unwrap();
       s.trim().to_string()
   }
}