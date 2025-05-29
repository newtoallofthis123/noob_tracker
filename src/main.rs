use db::Db;
use handler::Handler;

mod cli;
mod db;
mod handler;
mod utils;

fn main() {
    let cli = cli::setup_cli();
    let matches = cli.get_matches();

    let db = Db::new("tmp/test.db").unwrap();
    db.create_tables().unwrap();

    let handler = Handler::new(db);

    match matches.subcommand() {
        Some(("user", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => {
                handler.add_user(sub_matches).unwrap();
            }
            Some(("list", sub_matches)) => {
                handler.list_users(sub_matches).unwrap();
            }
            Some(("get", sub_matches)) => {
                handler.get_user(sub_matches).unwrap();
            }
            Some(("delete", sub_matches)) => {
                handler.delete_user(sub_matches).unwrap();
            }
            Some(("update", sub_matches)) => {
                handler.update_user(sub_matches).unwrap();
            }
            _ => {}
        },

        Some(("account", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => {
                handler.add_account(sub_matches).unwrap();
            }

            Some(("list", sub_matches)) => {
                handler.list_accounts(sub_matches).unwrap();
            }
            Some(("get", sub_matches)) => {
                handler.get_account(sub_matches).unwrap();
            }
            Some(("delete", sub_matches)) => {
                handler.delete_account(sub_matches).unwrap();
            }
            Some(("update", sub_matches)) => {
                handler.update_account(sub_matches).unwrap();
            }
            _ => {}
        },

        _ => {}
    }
}
