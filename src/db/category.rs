use std::str::FromStr;

use chrono::{DateTime, Local};
use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::{db::Db, utils::random_hash};

use super::Categories;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub created_at: chrono::DateTime<Local>,
}

pub struct CategoryRequest {
    pub name: String,
    pub icon: String,
}

impl Db {
    pub fn create_category(&self, category: &CategoryRequest) -> Result<Category, rusqlite::Error> {
        let id = random_hash(8);
        let time_now = Local::now().to_string();

        let query = Query::insert()
            .into_table(Categories::Table)
            .columns([
                Categories::Id,
                Categories::Name,
                Categories::Icon,
                Categories::CreatedAt,
            ])
            .values_panic(vec![
                id.clone().into(),
                category.name.clone().into(),
                category.icon.clone().into(),
                time_now.into(),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(Category {
            id,
            name: category.name.clone(),
            icon: category.icon.clone(),
            created_at: Local::now(),
        })
    }

    pub fn get_category(&self, id: &str) -> Result<Category, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Categories::Id,
                Categories::Name,
                Categories::Icon,
                Categories::CreatedAt,
            ])
            .from(Categories::Table)
            .and_where(Expr::col(Categories::Id).eq(id))
            .limit(1)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        let category = stmt.query_row((), |row| {
            let created_at =
                DateTime::from_str(row.get::<_, String>(3)?.as_str()).unwrap_or(Local::now());

            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                created_at,
            })
        })?;

        Ok(category)
    }

    pub fn get_all_categories(&self) -> Result<Vec<Category>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Categories::Id,
                Categories::Name,
                Categories::Icon,
                Categories::CreatedAt,
            ])
            .from(Categories::Table)
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Category, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(3)?.as_str()).unwrap_or(Local::now());

                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    icon: row.get(2)?,
                    created_at,
                })
            })?
            .map(|category| category.unwrap())
            .collect::<Vec<Category>>())
    }

    pub fn get_categories_by_name(&self, name: &str) -> Result<Vec<Category>, rusqlite::Error> {
        let query = Query::select()
            .columns([
                Categories::Id,
                Categories::Name,
                Categories::Icon,
                Categories::CreatedAt,
            ])
            .from(Categories::Table)
            .and_where(Expr::col(Categories::Name).like(format!("%{}%", name)))
            .to_string(SqliteQueryBuilder);

        let mut stmt = self.conn.prepare(&query)?;

        Ok(stmt
            .query_map((), |row| -> Result<Category, rusqlite::Error> {
                let created_at =
                    DateTime::from_str(row.get::<_, String>(3)?.as_str()).unwrap_or(Local::now());

                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    icon: row.get(2)?,
                    created_at,
                })
            })?
            .map(|category| category.unwrap())
            .collect::<Vec<Category>>())
    }

    pub fn update_category(&self, id: &str, category: &Category) -> Result<(), rusqlite::Error> {
        let query = Query::update()
            .table(Categories::Table)
            .and_where(Expr::col(Categories::Id).eq(id))
            .values(vec![
                (Categories::Name, category.name.clone().into()),
                (Categories::Icon, category.icon.clone().into()),
            ])
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }

    pub fn delete_category(&self, id: &str) -> Result<(), rusqlite::Error> {
        let query = Query::delete()
            .from_table(Categories::Table)
            .and_where(Expr::col(Categories::Id).eq(id))
            .to_string(SqliteQueryBuilder);

        self.conn.execute(&query, ())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::get_test_db_path;

    use super::*;

    #[test]
    fn test_categories() {
        // Clean up test database
        let _ = std::fs::remove_file(get_test_db_path("category"));

        let db = Db::new(get_test_db_path("category").as_str()).unwrap();
        db.create_tables().unwrap();

        // Create a test category
        let category_req = CategoryRequest {
            name: "Food & Dining".to_string(),
            icon: "🍽️".to_string(),
        };

        let created_category = db.create_category(&category_req).unwrap();
        assert_eq!(created_category.name, "Food & Dining");
        assert_eq!(created_category.icon, "🍽️");

        // Get category by ID
        let retrieved_category = db.get_category(&created_category.id).unwrap();
        assert_eq!(retrieved_category.id, created_category.id);
        assert_eq!(retrieved_category.name, "Food & Dining");
        assert_eq!(retrieved_category.icon, "🍽️");

        // Create another category for testing multiple results
        let category_req2 = CategoryRequest {
            name: "Transportation".to_string(),
            icon: "🚗".to_string(),
        };
        let _created_category2 = db.create_category(&category_req2).unwrap();

        // Get all categories
        let all_categories = db.get_all_categories().unwrap();
        assert_eq!(all_categories.len(), 2);

        // Get categories by name (search functionality)
        let food_categories = db.get_categories_by_name("Food").unwrap();
        assert_eq!(food_categories.len(), 1);
        assert_eq!(food_categories[0].name, "Food & Dining");

        let transport_categories = db.get_categories_by_name("Transport").unwrap();
        assert_eq!(transport_categories.len(), 1);
        assert_eq!(transport_categories[0].name, "Transportation");

        // Search should be case insensitive and partial
        let dining_categories = db.get_categories_by_name("dining").unwrap();
        assert_eq!(dining_categories.len(), 1);

        // Update category
        let mut updated_category = retrieved_category.clone();
        updated_category.name = "Restaurants & Dining".to_string();
        updated_category.icon = "🍴".to_string();

        let update_result = db.update_category(&created_category.id, &updated_category);
        assert!(update_result.is_ok());

        let updated_retrieved = db.get_category(&created_category.id).unwrap();
        assert_eq!(updated_retrieved.name, "Restaurants & Dining");
        assert_eq!(updated_retrieved.icon, "🍴");

        // Delete category
        let delete_result = db.delete_category(&created_category.id);
        assert!(delete_result.is_ok());

        // Verify deletion
        let get_deleted_result = db.get_category(&created_category.id);
        assert!(get_deleted_result.is_err());

        // Should only have one category left
        let remaining_categories = db.get_all_categories().unwrap();
        assert_eq!(remaining_categories.len(), 1);
        assert_eq!(remaining_categories[0].name, "Transportation");
    }

    #[test]
    fn test_category_search_edge_cases() {
        // Clean up test database
        let _ = std::fs::remove_file("test_category.db");

        let db = Db::new("test_category.db").unwrap();
        db.create_tables().unwrap();

        // Create categories with similar names
        let categories = vec![
            CategoryRequest {
                name: "Food".to_string(),
                icon: "🍎".to_string(),
            },
            CategoryRequest {
                name: "Fast Food".to_string(),
                icon: "🍟".to_string(),
            },
            CategoryRequest {
                name: "Food Delivery".to_string(),
                icon: "🚚".to_string(),
            },
            CategoryRequest {
                name: "Pet Food".to_string(),
                icon: "🐕".to_string(),
            },
        ];

        for category in categories {
            db.create_category(&category).unwrap();
        }

        // Search for "Food" should return all 4 categories
        let food_results = db.get_categories_by_name("Food").unwrap();
        assert_eq!(food_results.len(), 4);

        // Search for "Fast" should return 1 category
        let fast_results = db.get_categories_by_name("Fast").unwrap();
        assert_eq!(fast_results.len(), 1);
        assert_eq!(fast_results[0].name, "Fast Food");

        // Search for "Pet" should return 1 category
        let pet_results = db.get_categories_by_name("Pet").unwrap();
        assert_eq!(pet_results.len(), 1);
        assert_eq!(pet_results[0].name, "Pet Food");

        // Search for non-existent category should return empty
        let empty_results = db.get_categories_by_name("NonExistent").unwrap();
        assert_eq!(empty_results.len(), 0);

        // Cleanup
        let _ = std::fs::remove_file("test_search.db");
    }
}
