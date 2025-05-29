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

fn general_category_options() -> Vec<Arg> {
    vec![
        Arg::new("id")
            .short('i')
            .long("id")
            .action(ArgAction::Set)
            .help("The id of the category"),
        Arg::new("name")
            .short('n')
            .long("name")
            .action(ArgAction::Set)
            .help("The name of the category"),
        Arg::new("icon")
            .short('c')
            .long("icon")
            .action(ArgAction::Set)
            .help("The icon of the category"),
    ]
}

fn general_transaction_options() -> Vec<Arg> {
    vec![
        Arg::new("id")
            .short('i')
            .long("id")
            .action(ArgAction::Set)
            .help("The id of the transaction"),
        Arg::new("account_id")
            .short('a')
            .long("account-id")
            .action(ArgAction::Set)
            .help("The account id of the transaction"),
        Arg::new("amount")
            .short('m')
            .long("amount")
            .action(ArgAction::Set)
            .help("The amount of the transaction (in cents)"),
        Arg::new("type")
            .short('t')
            .long("type")
            .action(ArgAction::Set)
            .help("The type of the transaction (credit/debit)"),
        Arg::new("description")
            .short('d')
            .long("description")
            .action(ArgAction::Set)
            .help("The description of the transaction"),
        Arg::new("category_id")
            .short('c')
            .long("category-id")
            .action(ArgAction::Set)
            .help("The category id of the transaction"),
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
                        .args(&general_account_options()[1..]),
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
        .subcommand(
            Command::new("category")
                .about("Manage Categories")
                .subcommand(
                    Command::new("create")
                        .about("Create a new category")
                        .args(&general_category_options()[1..]),
                )
                .subcommand(Command::new("list").about("List all categories"))
                .subcommand(
                    Command::new("get")
                        .about("Get a category")
                        .args(general_category_options()),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a category")
                        .args(general_category_options()),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update a category")
                        .args(general_category_options()),
                ),
        )
        .subcommand(
            Command::new("transaction")
                .about("Manage Transactions")
                .subcommand(
                    Command::new("create")
                        .about("Create a new transaction")
                        .args(&general_transaction_options()[1..]),
                )
                .subcommand(
                    Command::new("list")
                        .about("List all transactions")
                        .arg(
                            Arg::new("account_id")
                                .short('a')
                                .long("account-id")
                                .action(ArgAction::Set)
                                .help("Filter by account id"),
                        ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get a transaction")
                        .arg(
                            Arg::new("id")
                                .short('i')
                                .long("id")
                                .action(ArgAction::Set)
                                .help("The id of the transaction"),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a transaction")
                        .arg(
                            Arg::new("id")
                                .short('i')
                                .long("id")
                                .action(ArgAction::Set)
                                .help("The id of the transaction"),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update a transaction")
                        .arg(
                            Arg::new("id")
                                .short('i')
                                .long("id")
                                .action(ArgAction::Set)
                                .help("The id of the transaction"),
                        ),
                ),
        )
}
