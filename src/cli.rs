use clap::{Arg, ArgAction, Command, command};

fn general_user_queries() -> Vec<Arg> {
    vec![
        Arg::new("id")
            .short('i')
            .long("id")
            .action(ArgAction::Set)
            .help("The id of the user"),
        Arg::new("name")
            .short('n')
            .long("name")
            .action(ArgAction::Set)
            .help("The name of the user"),
    ]
}

fn general_account_options() -> Vec<Arg> {
    vec![
        Arg::new("id")
            .short('i')
            .long("id")
            .action(ArgAction::Set)
            .help("The id of the account"),
        Arg::new("name")
            .short('n')
            .long("name")
            .action(ArgAction::Set)
            .help("The name of the account"),
        Arg::new("bank")
            .short('b')
            .long("bank")
            .action(ArgAction::Set)
            .help("The bank of the account"),
        Arg::new("account_number")
            .short('a')
            .long("account-number")
            .action(ArgAction::Set)
            .help("The account number of the account"),
        Arg::new("balance")
            .short('m')
            .long("balance")
            .action(ArgAction::Set)
            .help("The balance of the account"),
    ]
}

pub fn setup_cli() -> Command {
    command!()
        .author("NoobScience <noobscience@duck.com>")
        .about("Advanced CLI for tracking your finances")
        .subcommand(
            Command::new("user")
                .about("Manage Users")
                .subcommand(
                    Command::new("create").about("Create a new user").arg(
                        Arg::new("name")
                            .short('n')
                            .long("name")
                            .action(ArgAction::Set)
                            .help("The name of the user"),
                    ),
                )
                .subcommand(
                    Command::new("list").about("List all users").arg(
                        Arg::new("id")
                            .short('i')
                            .long("id")
                            .action(ArgAction::Set)
                            .help("The id of the user"),
                    ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get a user")
                        .args(general_user_queries()),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a user")
                        .args(general_user_queries()),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update a user")
                        .args(general_user_queries()),
                ),
        )
        .subcommand(
            Command::new("account")
                .about("Manage Accounts")
                .subcommand(
                    Command::new("create")
                        .about("Create a new account")
                        .args(general_account_options()),
                )
                .subcommand(Command::new("list").about("List all accounts"))
                .subcommand(
                    Command::new("get")
                        .about("Get an account")
                        .args(general_account_options()),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete an account")
                        .args(general_account_options()),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update an account")
                        .args(general_account_options()),
                ),
        )
}
