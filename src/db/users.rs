use std::str::FromStr;

use chrono::{DateTime, Local};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use tabled::Tabled;

use crate::{db::Db, utils::random_hash};

use super::Users;

#[derive(Debug, Clone, Tabled)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: chrono::DateTime<Local>,
}

pub struct UserRequest {
    pub name: String,
}

impl Db {
    pub fn create_user(&self, user: &UserRequest) -> Result<User, rusqlite::Error> {
        let id = random_hash(8);
        let time_now = Local::now().to_string();

        let query = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Id, Users::Name, Users::CreatedAt])
            .values_panic(vec![
                id.clone().into(),
                user.name.clone().into(),
                time_now.into(),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(User {
            id,
            name: user.name.clone(),
            created_at: Local::now(),
        })
    }

    pub fn get_user(&self, id: &str) -> Result<User, rusqlite::Error> {
        let query = Query::select()
            .columns([Users::Id, Users::Name, Users::CreatedAt])
            .from(Users::Table)
            .and_where(Expr::col(Users::Id).eq(id))
            .limit(1)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        let user = stmt.query_row((), |row| {
            let created_at =
                DateTime::from_str(row.get::<_, String>(2)?.as_str()).unwrap_or(Local::now());

            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at,
            })
        })?;

        Ok(user)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, rusqlite::Error> {
        let query = Query::select()
            .columns([Users::Id, Users::Name, Users::CreatedAt])
            .from(Users::Table)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<User, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(2)?.as_str()).unwrap_or(Local::now());

                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at,
                })
            })?
            .map(|user| user.unwrap())
            .collect::<Vec<User>>())
    }

    pub fn search_users_by_name(&self, name: &str) -> Result<Vec<User>, rusqlite::Error> {
        let query = Query::select()
            .columns([Users::Id, Users::Name, Users::CreatedAt])
            .from(Users::Table)
            .and_where(Expr::col(Users::Name).like(format!("%{}%", name).as_str()))
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<User, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(2)?.as_str()).unwrap_or(Local::now());

                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at,
                })
            })?
            .map(|user| user.unwrap())
            .collect::<Vec<User>>())
    }

    pub fn update_user(&self, id: &str, user: &User) -> Result<(), rusqlite::Error> {
        let query = Query::update()
            .table(Users::Table)
            .and_where(Expr::col(Users::Id).eq(id))
            .values(vec![
                (Users::Name, user.name.clone().into()),
                (Users::CreatedAt, user.created_at.to_string().into()),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }

    pub fn delete_user(&self, id: &str) -> Result<(), rusqlite::Error> {
        let query = Query::delete()
            .from_table(Users::Table)
            .and_where(Expr::col(Users::Id).eq(id))
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }
}

mod tests {
    use crate::utils::get_test_db_path;

    use super::*;

    #[test]
    fn test_users() {
        // delete test.db
        let _ = std::fs::remove_file(get_test_db_path("users"));

        let db = Db::new(get_test_db_path("users").as_str()).unwrap();
        db.create_tables().unwrap();
        let user = UserRequest {
            name: "John Doe".to_string(),
        };

        let create_res = db.create_user(&user);
        assert!(create_res.is_ok());

        let users = db.get_all_users().unwrap();
        assert!(!users.is_empty());

        let user = db.get_user(&create_res.unwrap().id).unwrap();

        assert_eq!(user.clone().name, "John Doe");

        let update_res = db.update_user(
            &user.id,
            &User {
                id: user.id.clone(),
                name: "Jane Doe".to_string(),
                created_at: user.created_at,
            },
        );
        assert!(update_res.is_ok());

        let user = db.get_user(&user.id).unwrap();
        assert_eq!(user.clone().name, "Jane Doe");

        let delete_res = db.delete_user(&user.id);
        assert!(delete_res.is_ok());
    }
}
