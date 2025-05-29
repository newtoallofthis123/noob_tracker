use std::io::{Error, ErrorKind};

use colored::Colorize;

use crate::{
    db::category::{Category, CategoryRequest},
    handler::Handler,
    utils::print_table,
};

impl Handler {
    pub fn add_category(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let name = args.get_one::<String>("name").cloned().unwrap_or_else(|| {
            let name = inquire::Text::new("Name")
                .with_help_message("Enter the name of the category")
                .prompt()
                .unwrap();

            name
        });

        let icon = args.get_one::<String>("icon").cloned().unwrap_or_else(|| {
            let icon = inquire::Text::new("Icon")
                .with_help_message("Enter the icon of the category (emoji)")
                .prompt()
                .unwrap();

            icon
        });

        let category_request = CategoryRequest { name, icon };

        let category = self
            .db
            .create_category(&category_request)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully created category with id".green(),
            category.id.green()
        );

        Ok(())
    }

    pub fn list_categories(&self, _args: &clap::ArgMatches) -> Result<(), Error> {
        let categories = self
            .db
            .get_all_categories()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        print_table(categories, "Categories");

        Ok(())
    }

    pub fn select_category(&self) -> Result<Category, Error> {
        let categories = self
            .db
            .get_all_categories()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let options = categories
            .iter()
            .map(|category| format!("{} {}", category.icon, category.name))
            .collect::<Vec<String>>();
        let option = inquire::Select::new("Select a category", options)
            .prompt()
            .unwrap();

        // Extract the name from the selected option (remove icon and space)
        let selected_name = option.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");

        Ok(categories
            .iter()
            .find(|category| category.name == selected_name)
            .unwrap()
            .clone())
    }

    pub fn get_category_by_id_or_name(&self, args: &clap::ArgMatches) -> Result<Category, Error> {
        let id = args.get_one::<String>("id").cloned();
        let name = args.get_one::<String>("name").cloned();

        if let Some(id) = id {
            let category = self
                .db
                .get_category(&id)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            Ok(category)
        } else if let Some(name) = name {
            let categories = self
                .db
                .get_categories_by_name(&name)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if categories.len() == 1 {
                return Ok(categories[0].clone());
            } else if categories.is_empty() {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("No categories found with name '{}'", name),
                ));
            } else {
                let options = categories
                    .iter()
                    .map(|category| format!("{} {}", category.icon, category.name))
                    .collect::<Vec<String>>();
                let option = inquire::Select::new("Select a category", options)
                    .prompt()
                    .unwrap();

                // Extract the name from the selected option (remove icon and space)
                let selected_name = option.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");

                Ok(categories
                    .iter()
                    .find(|category| category.name == selected_name)
                    .unwrap()
                    .clone())
            }
        } else {
            let name = inquire::Text::new("Name")
                .with_help_message("Enter the name of the category")
                .prompt()
                .unwrap();

            let categories = self
                .db
                .get_categories_by_name(&name)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if categories.len() == 1 {
                return Ok(categories[0].clone());
            } else if categories.is_empty() {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("No categories found with name '{}'", name),
                ));
            } else {
                let options = categories
                    .iter()
                    .map(|category| format!("{} {}", category.icon, category.name))
                    .collect::<Vec<String>>();
                let option = inquire::Select::new("Select a category", options)
                    .prompt()
                    .unwrap();

                // Extract the name from the selected option (remove icon and space)
                let selected_name = option.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");

                Ok(categories
                    .iter()
                    .find(|category| category.name == selected_name)
                    .unwrap()
                    .clone())
            }
        }
    }

    pub fn get_category(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let category = self.get_category_by_id_or_name(args)?;

        print_table(vec![category], "Category");
        Ok(())
    }

    pub fn update_category(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let mut category = self.get_category_by_id_or_name(args)?;

        let new_name = inquire::Text::new("New Name")
            .with_help_message("Enter the new name of the category")
            .with_default(&category.name)
            .prompt()
            .unwrap();

        let new_icon = inquire::Text::new("New Icon")
            .with_help_message("Enter the new icon of the category (emoji)")
            .with_default(&category.icon)
            .prompt()
            .unwrap();

        category.name = new_name;
        category.icon = new_icon;

        self.db
            .update_category(&category.id, &category)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {} {}",
            "Successfully updated category".green(),
            category.icon.green(),
            category.name.green()
        );

        Ok(())
    }

    pub fn delete_category(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let category = self.get_category_by_id_or_name(args)?;

        self.db
            .delete_category(&category.id)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {} {}",
            "Successfully deleted category".green(),
            category.icon.green(),
            category.name.green()
        );

        Ok(())
    }
}
