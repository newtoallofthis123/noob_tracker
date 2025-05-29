use rusqlite::{Connection, Result};
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SqliteQueryBuilder, Table};

pub mod account;
pub mod category;
pub mod transaction;
pub mod users;

pub struct Db {
    conn: Connection,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
    Name,
    Bank,
    AccountNumber,
    Balance,
    HolderId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Name,
    Icon,
    CreatedAt,
}

#[derive(Iden)]
enum Transactions {
    Table,
    Id,
    AccountId,
    Amount,
    Type,
    Description,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn create_tables(&self) -> Result<()> {
        let create_users = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(ColumnDef::new(Users::Id).text().not_null().primary_key())
            .col(ColumnDef::new(Users::Name).text().not_null())
            .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
            .build(SqliteQueryBuilder);

        let create_accounts = Table::create()
            .table(Accounts::Table)
            .if_not_exists()
            .col(ColumnDef::new(Accounts::Id).text().not_null().primary_key())
            .col(ColumnDef::new(Accounts::Name).text().not_null())
            .col(ColumnDef::new(Accounts::Bank).text().not_null())
            .col(ColumnDef::new(Accounts::AccountNumber).text())
            .col(ColumnDef::new(Accounts::Balance).integer().not_null())
            .col(ColumnDef::new(Accounts::HolderId).text().not_null())
            .col(ColumnDef::new(Accounts::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(Accounts::UpdatedAt).timestamp().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_accounts_users")
                    .from(Accounts::Table, Accounts::HolderId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder);

        let create_categories = Table::create()
            .table(Categories::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Categories::Id)
                    .text()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Categories::Name).text().not_null())
            .col(ColumnDef::new(Categories::Icon).text().not_null())
            .col(ColumnDef::new(Categories::CreatedAt).timestamp().not_null())
            .build(SqliteQueryBuilder);

        let create_transactions = Table::create()
            .table(Transactions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Transactions::Id)
                    .text()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Transactions::AccountId).text().not_null())
            .col(ColumnDef::new(Transactions::Amount).integer().not_null())
            .col(ColumnDef::new(Transactions::Type).text().not_null())
            .col(ColumnDef::new(Transactions::Description).text().not_null())
            .col(ColumnDef::new(Transactions::CategoryId).text().not_null())
            .col(
                ColumnDef::new(Transactions::CreatedAt)
                    .timestamp()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Transactions::UpdatedAt)
                    .timestamp()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_transactions_accounts")
                    .from(Transactions::Table, Transactions::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_transactions_categories")
                    .from(Transactions::Table, Transactions::CategoryId)
                    .to(Categories::Table, Categories::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder);

        let create_tables = vec![
            create_users,
            create_accounts,
            create_categories,
            create_transactions,
        ];
        for table in create_tables {
            self.conn.execute(&table, ())?;
        }

        Ok(())
    }
}
