use std::io::{Error, ErrorKind};

use colored::Colorize;
use tabled::Tabled;

use crate::{
    db::transaction::{Transaction, TransactionRequest},
    handler::Handler,
    utils::print_table,
};

#[derive(Debug, Clone, Tabled)]
pub struct TransactionResponse {
    pub id: String,
    pub account_id: String,
    pub amount: i64,
    pub transaction_type: String,
    pub description: String,
    pub category_id: String,
}

impl Handler {
    pub fn add_transaction(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let account_id = args.get_one::<String>("account_id").cloned().unwrap_or_else(|| {
            let account = self.select_account().unwrap();
            account.id
        });

        let amount = args.get_one::<i64>("amount").cloned().unwrap_or_else(|| {
            let amount = inquire::Text::new("Amount")
                .with_help_message("Enter the amount (in cents)")
                .prompt()
                .unwrap();

            amount.parse::<i64>().unwrap()
        });

        let transaction_type = args.get_one::<String>("type").cloned().unwrap_or_else(|| {
            let options = vec!["credit".to_string(), "debit".to_string()];
            inquire::Select::new("Transaction Type", options)
                .prompt()
                .unwrap()
        });

        let description = args.get_one::<String>("description").cloned().unwrap_or_else(|| {
            inquire::Text::new("Description")
                .with_help_message("Enter the description of the transaction")
                .prompt()
                .unwrap()
        });

        let category_id = args.get_one::<String>("category_id").cloned().unwrap_or_else(|| {
            let category = self.select_category().unwrap();
            category.id
        });

        let transaction_request = TransactionRequest {
            account_id,
            amount,
            transaction_type,
            description,
            category_id,
        };

        let transaction = self
            .db
            .create_transaction(&transaction_request)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully created transaction with id".green(),
            transaction.id.green()
        );

        Ok(())
    }

    pub fn list_transactions(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let account_id = args.get_one::<String>("account_id").cloned();

        let transactions = if let Some(account_id) = account_id {
            self.db
                .get_transactions_by_account(&account_id)
                .map_err(|e| Error::new(ErrorKind::Other, e))?
        } else {
            self.db
                .get_all_transactions()
                .map_err(|e| Error::new(ErrorKind::Other, e))?
        };

        let transaction_responses = transactions
            .into_iter()
            .map(|transaction| TransactionResponse {
                id: transaction.id,
                account_id: transaction.account_id,
                amount: transaction.amount,
                transaction_type: transaction.transaction_type,
                description: transaction.description,
                category_id: transaction.category_id,
            })
            .collect::<Vec<TransactionResponse>>();

        print_table(transaction_responses, "Transactions");

        Ok(())
    }

    pub fn select_transaction(&self) -> Result<Transaction, Error> {
        let transactions = self
            .db
            .get_all_transactions()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let options = transactions
            .iter()
            .map(|transaction| format!("{} - {}", transaction.description, transaction.amount))
            .collect::<Vec<String>>();
        let option = inquire::Select::new("Select a transaction", options.clone())
            .prompt()
            .unwrap();

        // Find the transaction by matching the description and amount
        let selected_index = options.iter().position(|x| x == &option).unwrap();
        Ok(transactions[selected_index].clone())
    }

    pub fn get_transaction_by_id(&self, args: &clap::ArgMatches) -> Result<Transaction, Error> {
        let id = args.get_one::<String>("id").cloned();

        if let Some(id) = id {
            let transaction = self
                .db
                .get_transaction(&id)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            Ok(transaction)
        } else {
            let transaction = self.select_transaction()?;
            Ok(transaction)
        }
    }

    pub fn get_transaction(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let transaction = self.get_transaction_by_id(args)?;

        let transaction_response = TransactionResponse {
            id: transaction.id,
            account_id: transaction.account_id,
            amount: transaction.amount,
            transaction_type: transaction.transaction_type,
            description: transaction.description,
            category_id: transaction.category_id,
        };

        print_table(vec![transaction_response], "Transaction");
        Ok(())
    }

    pub fn update_transaction(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let mut transaction = self.get_transaction_by_id(args)?;

        let new_amount = inquire::Text::new("New Amount")
            .with_help_message("Enter the new amount (in cents)")
            .with_default(&transaction.amount.to_string())
            .prompt()
            .unwrap();

        let new_amount = new_amount.parse::<i64>().unwrap();

        let new_type = inquire::Select::new("New Transaction Type", vec!["credit", "debit"])
            .prompt()
            .unwrap();

        let new_description = inquire::Text::new("New Description")
            .with_help_message("Enter the new description of the transaction")
            .with_default(&transaction.description)
            .prompt()
            .unwrap();

        transaction.amount = new_amount;
        transaction.transaction_type = new_type.to_string();
        transaction.description = new_description;

        self.db
            .update_transaction(&transaction.id, &transaction)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully updated transaction".green(),
            transaction.id.green()
        );

        Ok(())
    }

    pub fn delete_transaction(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let transaction = self.get_transaction_by_id(args)?;

        self.db
            .delete_transaction(&transaction.id)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully deleted transaction".green(),
            transaction.id.green()
        );

        Ok(())
    }
}
