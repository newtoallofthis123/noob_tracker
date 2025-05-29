use std::io::{Error, ErrorKind};

use colored::Colorize;

use crate::{
    db::account::{Account, AccountRequest, AccountResponse},
    handler::Handler,
    utils::print_table,
};

impl Handler {
    pub fn add_account(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let name = args.get_one::<String>("name").cloned().unwrap_or_else(|| {
            let name = inquire::Text::new("Name")
                .with_help_message("Enter the name of the account")
                .prompt()
                .unwrap();

            name
        });

        let balance = args.get_one::<f64>("balance").cloned().unwrap_or_else(|| {
            let balance = inquire::Text::new("Balance")
                .with_help_message("Enter the balance of the account")
                .prompt()
                .unwrap();

            balance.parse::<f64>().unwrap()
        });

        let bank = args.get_one::<String>("bank").cloned().unwrap_or_else(|| {
            let bank = inquire::Text::new("Bank")
                .with_help_message("Enter the bank of the account")
                .prompt()
                .unwrap();

            bank
        });

        let holder_id = self.select_user()?.id;

        let account_number = args.get_one::<String>("account_number").cloned();

        let account_request = AccountRequest {
            name,
            balance,
            bank,
            account_number,
            holder_id,
        };

        let account = self
            .db
            .create_account(&account_request)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully created account".green(),
            account.name.green()
        );

        Ok(())
    }

    pub fn list_accounts(&self, _args: &clap::ArgMatches) -> Result<(), Error> {
        let accounts = self
            .db
            .get_all_accounts()
            .map_err(|e| Error::new(ErrorKind::Other, e))?
            .into_iter()
            .map(|account| AccountResponse {
                id: account.id,
                name: account.name,
                bank: account.bank,
                balance: account.balance,
                holder_id: account.holder_id,
            })
            .collect::<Vec<AccountResponse>>();

        print_table(accounts, "Accounts");

        Ok(())
    }

    pub fn select_account(&self) -> Result<Account, Error> {
        let accounts = self
            .db
            .get_all_accounts()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let options = accounts
            .iter()
            .map(|account| account.name.clone())
            .collect::<Vec<String>>();
        let option = inquire::Select::new("Select an account", options)
            .prompt()
            .unwrap();

        Ok(accounts
            .iter()
            .find(|account| account.name == option)
            .unwrap()
            .clone())
    }

    pub fn get_account_by_id_or_name(&self, args: &clap::ArgMatches) -> Result<Account, Error> {
        let id = args.get_one::<String>("id").cloned();
        let name = args.get_one::<String>("name").cloned();

        if let Some(id1) = id {
            let account = self
                .db
                .get_account(&id1)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            Ok(account)
        } else if let Some(name1) = name {
            let account = self
                .db
                .search_accounts_by_name(&name1)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if account.len() == 1 {
                Ok(account[0].clone())
            } else {
                let options = account
                    .iter()
                    .map(|account| account.name.clone())
                    .collect::<Vec<String>>();

                let option = inquire::Select::new("Select an account", options)
                    .prompt()
                    .unwrap();

                Ok(account
                    .iter()
                    .find(|account| account.name == option)
                    .unwrap()
                    .clone())
            }
        } else {
            let account = self.select_account()?;
            Ok(account)
        }
    }

    pub fn get_account(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let account = self.get_account_by_id_or_name(args)?;

        let account_response = AccountResponse {
            id: account.id,
            name: account.name,
            bank: account.bank,
            balance: account.balance,
            holder_id: account.holder_id,
        };

        print_table(vec![account_response], "Account");
        Ok(())
    }

    pub fn update_account(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let mut account = self.get_account_by_id_or_name(args)?;

        let new_name = inquire::Text::new("New Name")
            .with_help_message("Enter the new name of the account")
            .prompt()
            .unwrap();

        let new_balance = inquire::Text::new("New Balance")
            .with_help_message("Enter the new balance of the account")
            .prompt()
            .unwrap();

        let new_balance = new_balance.parse::<f64>().unwrap();

        account.name = new_name;
        account.balance = new_balance;

        self.db
            .update_account(&account.id, &account)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully updated account".green(),
            account.name.green()
        );

        Ok(())
    }

    pub fn delete_account(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let account = self.get_account_by_id_or_name(args)?;

        self.db
            .delete_account(&account.id)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully deleted account".green(),
            account.name.green()
        );

        Ok(())
    }
}
