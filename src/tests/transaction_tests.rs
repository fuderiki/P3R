use crate::modules::transaction::Transaction;

#[test]
fn test_transaction_creation() {
    let from = String::from("sender");
    let to = String::from("recipient");
    let amount = 10.0;

    let transaction = Transaction::new(from.clone(), to.clone(), amount);

    assert_eq!(transaction.from, from);
    assert_eq!(transaction.to, to);
    assert_eq!(transaction.amount, amount);
    // Add assertions to test the timestamp and other fields of the Transaction struct
}

