struct Cash{
    amount: f64,
}


impl Cash {
// new method implemented for declaring initial amount in the atm
    fn new(amount: f64) -> Self {
        println!(" ATM amount: {}", amount);
        println!("Cash stored at memory address: {:p}", &amount);
        Cash {
            amount,
        }
    }

    fn withdraw(&mut self, amount: f64){
        println!("withdraw amount : {}", amount);
        if amount <= self.amount {
            self.amount -= amount;
            println!("Withdrawal successful, Balance: {}", self.amount);
        } else {
            // println!("Insufficient funds. Withdrawal failed.");
            panic!("withdraw failed");
        }
    }


}

fn shutdown() {
    println!("ATM shutting down...");
}



fn main() {
    println!("Hello, world!");

    let mut atm = Cash::new(10000.0);
    atm.withdraw(6000.0);
    atm.withdraw(400.0); 
    atm.withdraw(4000.0); // This will panic 
    shutdown();
}
