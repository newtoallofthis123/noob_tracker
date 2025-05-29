use std::io::{Error, ErrorKind};

use colored::Colorize;

use crate::{
    db::users::{User, UserRequest},
    handler::Handler,
    utils::print_table,
};

impl Handler {
    pub fn add_user(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let name = args.get_one::<String>("name").cloned().unwrap_or_else(|| {
            let name = inquire::Text::new("Name")
                .with_help_message("Enter the name of the user")
                .prompt()
                .unwrap();

            name
        });

        let user_request = UserRequest { name };

        let user = self
            .db
            .create_user(&user_request)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully created user with id".green(),
            user.id.green()
        );

        Ok(())
    }

    pub fn list_users(&self, _args: &clap::ArgMatches) -> Result<(), Error> {
        let users = self
            .db
            .get_all_users()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        print_table(users, "Users");

        Ok(())
    }

    pub fn select_user(&self) -> Result<User, Error> {
        let users = self
            .db
            .get_all_users()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let options = users
            .iter()
            .map(|user| user.name.clone())
            .collect::<Vec<String>>();
        let option = inquire::Select::new("Select a user", options)
            .prompt()
            .unwrap();

        Ok(users
            .iter()
            .find(|user| user.name == option)
            .unwrap()
            .clone())
    }

    pub fn get_user_by_id_or_name(&self, args: &clap::ArgMatches) -> Result<User, Error> {
        // check if id is present with the --id flag, if not check for --name flag or finally ask the user for a name

        let id = args.get_one::<String>("id").cloned();
        let name = args.get_one::<String>("name").cloned();

        if id.is_some() {
            let user = self
                .db
                .get_user(&id.unwrap())
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            Ok(user)
        } else if name.is_some() {
            let users = self
                .db
                .search_users_by_name(&name.unwrap())
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if users.len() == 1 {
                return Ok(users[0].clone());
            } else {
                let options = users
                    .iter()
                    .map(|user| user.name.clone())
                    .collect::<Vec<String>>();
                let option = inquire::Select::new("Select a user", options)
                    .prompt()
                    .unwrap();

                Ok(users
                    .iter()
                    .find(|user| user.name == option)
                    .unwrap()
                    .clone())
            }
        } else {
            let name = inquire::Text::new("Name")
                .with_help_message("Enter the name of the user")
                .prompt()
                .unwrap();

            let users = self
                .db
                .search_users_by_name(&name)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if users.len() == 1 {
                return Ok(users[0].clone());
            } else {
                let options = users
                    .iter()
                    .map(|user| user.name.clone())
                    .collect::<Vec<String>>();
                let option = inquire::Select::new("Select a user", options)
                    .prompt()
                    .unwrap();

                Ok(users
                    .iter()
                    .find(|user| user.name == option)
                    .unwrap()
                    .clone())
            }
        }
    }

    pub fn get_user(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let user = self.get_user_by_id_or_name(args)?;

        print_table(vec![user], "User");
        Ok(())
    }

    pub fn update_user(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let mut user = self.get_user_by_id_or_name(args)?;

        let new_name = inquire::Text::new("New Name")
            .with_help_message("Enter the new name of the user")
            .prompt()
            .unwrap();

        user.name = new_name;

        self.db
            .update_user(&user.id, &user)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully updated user".green(),
            user.name.green()
        );

        Ok(())
    }

    pub fn delete_user(&self, args: &clap::ArgMatches) -> Result<(), Error> {
        let user = self.get_user_by_id_or_name(args)?;

        self.db
            .delete_user(&user.id)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        println!(
            "{} {}",
            "Successfully deleted user".green(),
            user.name.green()
        );

        Ok(())
    }
}
