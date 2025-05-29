use std::str::FromStr;

use chrono::{DateTime, Local};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use tabled::Tabled;

use crate::{db::Db, utils::random_hash};

use super::Transactions;

#[derive(Debug, Clone, Tabled)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub amount: i64,
    pub transaction_type: String,
    pub description: String,
    pub category_id: String,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
}

pub struct TransactionRequest {
    pub account_id: String,
    pub amount: i64,
    pub transaction_type: String,
    pub description: String,
    pub category_id: String,
}

impl Db {
    pub fn create_transaction(
        &self,
        transaction: &TransactionRequest,
    ) -> Result<Transaction, rusqlite::Error> {
        let id = random_hash(8);
        let time_now = Local::now().to_string();

        let query = Query::insert()
            .into_table(Transactions::Table)
            .columns([
                Transactions::Id,
                Transactions::AccountId,
                Transactions::Amount,
                Transactions::Type,
                Transactions::Description,
                Transactions::CategoryId,
                Transactions::CreatedAt,
                Transactions::UpdatedAt,
            ])
            .values_panic(vec![
                id.clone().into(),
                transaction.account_id.clone().into(),
                transaction.amount.into(),
                transaction.transaction_type.clone().into(),
                transaction.description.clone().into(),
                transaction.category_id.clone().into(),
                time_now.clone().into(),
                time_now.into(),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(Transaction {
            id,
            account_id: transaction.account_id.clone(),
            amount: transaction.amount,
            transaction_type: transaction.transaction_type.clone(),
            description: transaction.description.clone(),
            category_id: transaction.category_id.clone(),
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }

    pub fn get_transaction(&self, id: &str) -> Result<Transaction, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Transactions::Id,
                Transactions::AccountId,
                Transactions::Amount,
                Transactions::Type,
                Transactions::Description,
                Transactions::CategoryId,
                Transactions::CreatedAt,
                Transactions::UpdatedAt,
            ])
            .from(Transactions::Table)
            .and_where(Expr::col(Transactions::Id).eq(id))
            .limit(1)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        let transaction = stmt.query_row((), |row| {
            let created_at =
                DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
            let updated_at =
                DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

            Ok(Transaction {
                id: row.get(0)?,
                account_id: row.get(1)?,
                amount: row.get(2)?,
                transaction_type: row.get(3)?,
                description: row.get(4)?,
                category_id: row.get(5)?,
                created_at,
                updated_at,
            })
        })?;

        Ok(transaction)
    }

    pub fn get_all_transactions(&self) -> Result<Vec<Transaction>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Transactions::Id,
                Transactions::AccountId,
                Transactions::Amount,
                Transactions::Type,
                Transactions::Description,
                Transactions::CategoryId,
                Transactions::CreatedAt,
                Transactions::UpdatedAt,
            ])
            .from(Transactions::Table)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Transaction, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
                let updated_at =
                    DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

                Ok(Transaction {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    amount: row.get(2)?,
                    transaction_type: row.get(3)?,
                    description: row.get(4)?,
                    category_id: row.get(5)?,
                    created_at,
                    updated_at,
                })
            })?
            .map(|transaction| transaction.unwrap())
            .collect::<Vec<Transaction>>())
    }

    pub fn get_transactions_by_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<Transaction>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Transactions::Id,
                Transactions::AccountId,
                Transactions::Amount,
                Transactions::Type,
                Transactions::Description,
                Transactions::CategoryId,
                Transactions::CreatedAt,
                Transactions::UpdatedAt,
            ])
            .from(Transactions::Table)
            .and_where(Expr::col(Transactions::AccountId).eq(account_id))
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Transaction, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(6)?.as_str()).unwrap_or(Local::now());
                let updated_at =
                    DateTime::from_str(row.get::<_, String>(7)?.as_str()).unwrap_or(Local::now());

                Ok(Transaction {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    amount: row.get(2)?,
                    transaction_type: row.get(3)?,
                    description: row.get(4)?,
                    category_id: row.get(5)?,
                    created_at,
                    updated_at,
                })
            })?
            .map(|transaction| transaction.unwrap())
            .collect::<Vec<Transaction>>())
    }

    pub fn update_transaction(
        &self,
        id: &str,
        transaction: &Transaction,
    ) -> Result<(), rusqlite::Error> {
        let updated_at = Local::now().to_string();

        let query = Query::update()
            .table(Transactions::Table)
            .and_where(Expr::col(Transactions::Id).eq(id))
            .values(vec![
                (
                    Transactions::AccountId,
                    transaction.account_id.clone().into(),
                ),
                (Transactions::Amount, transaction.amount.into()),
                (
                    Transactions::Type,
                    transaction.transaction_type.clone().into(),
                ),
                (
                    Transactions::Description,
                    transaction.description.clone().into(),
                ),
                (
                    Transactions::CategoryId,
                    transaction.category_id.clone().into(),
                ),
                (Transactions::UpdatedAt, updated_at.into()),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }

    pub fn delete_transaction(&self, id: &str) -> Result<(), rusqlite::Error> {
        let query = Query::delete()
            .from_table(Transactions::Table)
            .and_where(Expr::col(Transactions::Id).eq(id))
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{account::AccountRequest, category::CategoryRequest, users::UserRequest},
        utils::get_test_db_path,
    };

    use super::*;

    #[test]
    fn test_transactions() {
        // Clean up test database
        let _ = std::fs::remove_file(get_test_db_path("transaction"));

        let db = Db::new(get_test_db_path("transaction").as_str()).unwrap();
        db.create_tables().unwrap();

        let user = UserRequest {
            name: "John Doe".to_string(),
        };
        let user = db.create_user(&user).unwrap();

        let account = AccountRequest {
            name: "John Doe".to_string(),
            bank: "Bank of America".to_string(),
            account_number: None,
            balance: 0.0,
            holder_id: user.id.clone(),
        };
        let account = db.create_account(&account).unwrap();

        let category = CategoryRequest {
            name: "Test category".to_string(),
            icon: "🍔".to_string(),
        };
        let category = db.create_category(&category).unwrap();

        // Create a test transaction
        let transaction_req = TransactionRequest {
            account_id: account.id.clone(),
            amount: 1000,
            transaction_type: "credit".to_string(),
            description: "Test transaction".to_string(),
            category_id: category.id.clone(),
        };

        let created_transaction = db.create_transaction(&transaction_req).unwrap();
        assert_eq!(created_transaction.amount, 1000);
        assert_eq!(created_transaction.description, "Test transaction");

        // Get transaction by ID
        let retrieved_transaction = db.get_transaction(&created_transaction.id).unwrap();
        assert_eq!(retrieved_transaction.id, created_transaction.id);
        assert_eq!(retrieved_transaction.amount, 1000);

        // Get all transactions
        let all_transactions = db.get_all_transactions().unwrap();
        assert_eq!(all_transactions.len(), 1);

        // Get transactions by account
        let account_transactions = db.get_transactions_by_account(&account.id).unwrap();
        assert_eq!(account_transactions.len(), 1);

        // Update transaction
        let mut updated_transaction = retrieved_transaction.clone();
        updated_transaction.amount = 2000;
        updated_transaction.description = "Updated transaction".to_string();

        let update_result = db.update_transaction(&created_transaction.id, &updated_transaction);
        assert!(update_result.is_ok());

        let updated_retrieved = db.get_transaction(&created_transaction.id).unwrap();
        assert_eq!(updated_retrieved.amount, 2000);
        assert_eq!(updated_retrieved.description, "Updated transaction");

        // Delete transaction
        let delete_result = db.delete_transaction(&created_transaction.id);
        assert!(delete_result.is_ok());

        let all_transactions_after_delete = db.get_all_transactions().unwrap();
        assert!(all_transactions_after_delete.is_empty());
    }
}
