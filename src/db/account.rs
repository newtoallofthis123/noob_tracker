use std::str::FromStr;

use chrono::{DateTime, Local};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use tabled::Tabled;

use crate::{db::Db, utils::random_hash};

use super::Accounts;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub bank: String,
    pub account_number: Option<String>,
    pub balance: f64,
    pub holder_id: String,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, Tabled)]
pub struct AccountResponse {
    pub id: String,
    pub name: String,
    pub bank: String,
    pub balance: f64,
    pub holder_id: String,
}

pub struct AccountRequest {
    pub name: String,
    pub bank: String,
    pub account_number: Option<String>,
    pub balance: f64,
    pub holder_id: String,
}

impl Db {
    pub fn create_account(&self, account: &AccountRequest) -> Result<Account, rusqlite::Error> {
        let id = random_hash(8);
        let time_now = Local::now().to_string();

        let query = Query::insert()
            .into_table(Accounts::Table)
            .columns([
                Accounts::Id,
                Accounts::Name,
                Accounts::Bank,
                Accounts::AccountNumber,
                Accounts::Balance,
                Accounts::HolderId,
                Accounts::CreatedAt,
                Accounts::UpdatedAt,
            ])
            .values_panic(vec![
                id.clone().into(),
                account.name.clone().into(),
                account.bank.clone().into(),
                account.account_number.clone().into(),
                account.balance.into(),
                account.holder_id.clone().into(),
                time_now.clone().into(),
                time_now.into(),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(Account {
            id,
            name: account.name.clone(),
            bank: account.bank.clone(),
            account_number: account.account_number.clone(),
            balance: account.balance,
            holder_id: account.holder_id.clone(),
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }

    pub fn get_account(&self, id: &str) -> Result<Account, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Accounts::Id,
                Accounts::Name,
                Accounts::Bank,
                Accounts::AccountNumber,
                Accounts::Balance,
                Accounts::HolderId,
                Accounts::CreatedAt,
                Accounts::UpdatedAt,
            ])
            .from(Accounts::Table)
            .and_where(Expr::col(Accounts::Id).eq(id))
            .limit(1)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        let account = stmt.query_row((), |row| {
            let created_at =
                DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
            let updated_at =
                DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                bank: row.get(2)?,
                account_number: row.get(3)?,
                balance: row.get(4)?,
                holder_id: row.get(5)?,
                created_at,
                updated_at,
            })
        })?;

        Ok(account)
    }

    pub fn search_accounts_by_name(&self, name: &str) -> Result<Vec<Account>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Accounts::Id,
                Accounts::Name,
                Accounts::Bank,
                Accounts::AccountNumber,
                Accounts::Balance,
                Accounts::HolderId,
                Accounts::CreatedAt,
                Accounts::UpdatedAt,
            ])
            .from(Accounts::Table)
            .and_where(Expr::col(Accounts::Name).like(format!("%{}%", name).as_str()))
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        let account = stmt.query_map((), |row| {
            let created_at =
                DateTime::from_str(row.get::<_, String>(5)?.as_str()).unwrap_or(Local::now());
            let updated_at =
                DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());

            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                bank: row.get(2)?,
                account_number: row.get(3)?,
                balance: row.get(4)?,
                holder_id: row.get(5)?,
                created_at,
                updated_at,
            })
        })?;

        Ok(account
            .map(|account| account.unwrap())
            .collect::<Vec<Account>>())
    }

    pub fn get_all_accounts(&self) -> Result<Vec<Account>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Accounts::Id,
                Accounts::Name,
                Accounts::Bank,
                Accounts::AccountNumber,
                Accounts::Balance,
                Accounts::HolderId,
                Accounts::CreatedAt,
                Accounts::UpdatedAt,
            ])
            .from(Accounts::Table)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Account, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
                let updated_at =
                    DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

                Ok(Account {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    bank: row.get(2)?,
                    account_number: row.get(3)?,
                    balance: row.get(4)?,
                    holder_id: row.get(5)?,
                    created_at,
                    updated_at,
                })
            })?
            .map(|account| account.unwrap())
            .collect::<Vec<Account>>())
    }

    pub fn get_accounts_by_holder(&self, holder_id: &str) -> Result<Vec<Account>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Accounts::Id,
                Accounts::Name,
                Accounts::Bank,
                Accounts::AccountNumber,
                Accounts::Balance,
                Accounts::HolderId,
                Accounts::CreatedAt,
                Accounts::UpdatedAt,
            ])
            .from(Accounts::Table)
            .and_where(Expr::col(Accounts::HolderId).eq(holder_id))
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Account, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
                let updated_at =
                    DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

                Ok(Account {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    bank: row.get(2)?,
                    account_number: row.get(3)?,
                    balance: row.get(4)?,
                    holder_id: row.get(5)?,
                    created_at,
                    updated_at,
                })
            })?
            .map(|account| account.unwrap())
            .collect::<Vec<Account>>())
    }

    pub fn update_account(&self, id: &str, account: &Account) -> Result<(), rusqlite::Error> {
        let updated_at = Local::now().to_string();

        let query = Query::update()
            .table(Accounts::Table)
            .and_where(Expr::col(Accounts::Id).eq(id))
            .values(vec![
                (Accounts::Name, account.name.clone().into()),
                (Accounts::Bank, account.bank.clone().into()),
                (
                    Accounts::AccountNumber,
                    account.account_number.clone().into(),
                ),
                (Accounts::Balance, account.balance.into()),
                (Accounts::HolderId, account.holder_id.clone().into()),
                (Accounts::UpdatedAt, updated_at.into()),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }

    pub fn delete_account(&self, id: &str) -> Result<(), rusqlite::Error> {
        let query = Query::delete()
            .from_table(Accounts::Table)
            .and_where(Expr::col(Accounts::Id).eq(id))
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::users::UserRequest, utils::get_test_db_path};

    use super::*;

    #[test]
    fn test_accounts() {
        // Clean up test database
        let _ = std::fs::remove_file(get_test_db_path("account"));

        let db = Db::new(get_test_db_path("account").as_str()).unwrap();
        db.create_tables().unwrap();

        // Create a user first (required for foreign key)
        let user_req = UserRequest {
            name: "John Doe".to_string(),
        };
        let user = db.create_user(&user_req).unwrap();

        // Create a test account
        let account_req = AccountRequest {
            name: "Checking Account".to_string(),
            bank: "Test Bank".to_string(),
            account_number: Some("123456789".to_string()),
            balance: 1000.50,
            holder_id: user.id.clone(),
        };

        let created_account = db.create_account(&account_req).unwrap();
        assert_eq!(created_account.name, "Checking Account");
        assert_eq!(created_account.bank, "Test Bank");
        assert_eq!(created_account.balance, 1000.50);
        assert_eq!(created_account.holder_id, user.id);

        // Get account by ID
        let retrieved_account = db.get_account(&created_account.id).unwrap();
        assert_eq!(retrieved_account.id, created_account.id);
        assert_eq!(retrieved_account.name, "Checking Account");

        // Get all accounts
        let all_accounts = db.get_all_accounts().unwrap();
        assert!(!all_accounts.is_empty());

        // Get accounts by holder
        let holder_accounts = db.get_accounts_by_holder(&user.id).unwrap();
        assert_eq!(holder_accounts.len(), 1);
        assert_eq!(holder_accounts[0].holder_id, user.id);

        // Update account
        let mut updated_account = retrieved_account.clone();
        updated_account.name = "Updated Checking Account".to_string();
        updated_account.balance = 2000.75;

        let update_result = db.update_account(&created_account.id, &updated_account);
        assert!(update_result.is_ok());

        let updated_retrieved = db.get_account(&created_account.id).unwrap();
        assert_eq!(updated_retrieved.name, "Updated Checking Account");
        assert_eq!(updated_retrieved.balance, 2000.75);

        // Delete account
        let delete_result = db.delete_account(&created_account.id);
        assert!(delete_result.is_ok());
    }
}
