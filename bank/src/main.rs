#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
    total_balance: f32,
}
impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: Vec::new(),
            total_balance: 0.0,
        }
    }
}
impl Account {
    fn new(id: u32, holder: String) -> Self {
       Account {
           id,
           holder,
           balance: 0
       }
    }
}
fn print_account(account: &Account) {
    println!("{:#?}", account);
}
fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("John"));
    // bank.accounts.push(account);
    println!("{:#?}", bank);
    print_account(&account);
}
