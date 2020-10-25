use crate::types::*;

pub fn deposit(accounts: &mut Accounts, transactions: &mut Disputes, record: Record) {
    let Record {
        client,
        amount,
        r#type,
        ..
    } = record.clone();
    transactions.insert(
        record.tx,
        Dispute {
            disputed: DisputeState::None,
            client,
            amount: amount.unwrap(),
            r#type,
        },
    );
    accounts
        .entry(record.client)
        .and_modify(|e| {
            if !e.frozen {
                e.amount += record.amount.unwrap()
            }
        })
        .or_insert(Account {
            amount: record.amount.unwrap(),
            held: 0.0,
            frozen: false,
        });
}

pub fn withdrawal(accounts: &mut Accounts, _transactions: &mut Disputes, record: Record) {
    // given withdrawals can not be disputed, we don't need to keep track of them
    accounts.entry(record.client).and_modify(|e| {
        if e.amount - record.amount.unwrap() > 0.0 && !e.frozen {
            e.amount -= record.amount.unwrap()
        };
    });
    // a client that only withdrawals will not be created
}
pub fn dispute(accounts: &mut Accounts, transactions: &mut Disputes, record: Record) {
    // should only a deposit be disputable?
    // "funds should decrease by the amount disputed" - makes only sense for withdrawal
    // should a client only be able to dispute its own transactions?
    if let Some(transaction) = transactions.get_mut(&record.tx) {
        if transaction.disputed == DisputeState::None {
            accounts.entry(transaction.client).and_modify(|e| {
                e.held += transaction.amount;
                e.amount -= transaction.amount;
            });
            transaction.disputed = DisputeState::Dispute;
        }
    };
}
pub fn resolve(accounts: &mut Accounts, transactions: &mut Disputes, record: Record) {
    if let Some(transaction) = transactions.get_mut(&record.tx) {
        if transaction.disputed == DisputeState::Dispute {
            accounts.entry(transaction.client).and_modify(|e| {
                e.held -= transaction.amount;
                e.amount += transaction.amount;
            });
            transaction.disputed = DisputeState::None;
        }
    };
}
pub fn chargeback(accounts: &mut Accounts, transactions: &mut Disputes, record: Record) {
    if let Some(transaction) = transactions.get_mut(&record.tx) {
        if transaction.disputed == DisputeState::Dispute {
            accounts.entry(transaction.client).and_modify(|e| {
                e.held -= transaction.amount;
                e.frozen = true;
            });
            transaction.disputed = DisputeState::ChargeBack;
        }
    };
}
