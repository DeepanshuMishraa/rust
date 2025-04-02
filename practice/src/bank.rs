
enum Transaction{
  Deposit(f64),
  Withdraw(f64)
}

struct BankAccount{
  balance:f64,
  transactions:Vec<Transaction>
}

impl BankAccount{
  fn new()->BankAccount{
    BankAccount{
      balance:0.0,
      transactions:Vec::new()
    }
  }

  fn deposit(&mut self, amount:f64){
    self.balance += amount;
    self.transactions.push(Transaction::Deposit(amount));
  }

  fn withdraw(&mut self,amount:f64){
    if amount > self.balance{
      println!("Insufficient funds");
    }else{
      self.balance -= amount;

      self.transactions.push(Transaction::Withdraw(amount));
    }
  }
}


fn main(){

  let mut account = BankAccount::new();

  account.deposit(1000.0);
  account.withdraw(10000.0);

  println!("Current balance: {}", account.balance);

  for transaction in &account.transactions{
    match transaction{
      Transaction::Deposit(amount) => println!("Deposited: {}", amount),
      Transaction::Withdraw(amount) => println!("Withdrawn: {}", amount),
    }
  }

}
