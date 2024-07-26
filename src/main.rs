mod api;
mod consts;
mod utils;
mod providers;
mod config;

use api::user::User;
use config::{write_config, BlacketConfig};
use cursive::{view::Nameable, views::{self, SelectView}, Cursive, CursiveExt};

fn main() {
    let mut siv = Cursive::new();
    siv.load_toml(include_str!("tui-theme.toml")).unwrap();

    let config = config::parse_config(None);
    let config: &'static mut BlacketConfig = Box::leak(Box::new(config));
    let dialog = views::Dialog::around(SelectView::new()
        .with_all_str(config.accounts.accounts.iter().map(|a| a.username.clone()).collect::<Vec<String>>())
        .with_all_str(["+ New"].to_vec())
        .with_name("account_selector")
    )
        .title("Select an account")
        .button("Select", move |s| {
            let account_selector = s.call_on_name("account_selector", |v: &mut SelectView| v.selection());
            let selected = account_selector.unwrap().unwrap().to_string();
            let config = config.clone();
            if selected == "+ New" {
                let dialog = views::Dialog::new()
                    .title("Add an account")
                    .content(views::LinearLayout::vertical()
                        .child(views::TextView::new("Username:"))
                        .child(views::EditView::new().with_name("username"))
                        .child(views::TextView::new("Password:"))
                        .child(views::EditView::new().secret().with_name("password"))
                    )
                    .button("Add", move |s| {
                        let username = s.call_on_name("username", |v: &mut views::EditView| v.get_content()).unwrap().to_string();
                        let password = s.call_on_name("password", |v: &mut views::EditView| v.get_content()).unwrap().to_string();
                        let account = config::Account {
                            username,
                            password,
                            default: false
                        };
                        s.pop_layer();
                        s.call_on_name("account_selector", |v: &mut SelectView| v.insert_item_str(v.len() - 1, account.username.clone()));
                        s.call_on_name("account_selector", |v: &mut SelectView| v.set_selection(v.len() - 2));
                        let mut config = config.clone();
                        config.accounts.accounts.push(account);
                        write_config(&config, None);
                    })
                    .button("Back", |s| {
                        s.pop_layer();
                    });
                s.add_layer(dialog);
            } else {
                let account = &config.accounts.accounts.iter().find(|a| a.username == selected).unwrap();
                let mut user = User::new_credentials(&account.username, &account.password);
                let result = user.login(None, None);
                if result.is_err() {
                    s.pop_layer();
                    let dialog = views::Dialog::new()
                        .title("Error")
                        .content(views::TextView::new("An error occurred while logging in."))
                        .button("Ok", |s| {
                            s.pop_layer();
                        });
                    s.add_layer(dialog);
                } else {
                    s.pop_layer();
                    let dialog = views::Dialog::new()
                        .title("Success")
                        .content(views::TextView::new("Successfully logged in."))
                        .button("Ok", |s| {
                            s.pop_layer();
                        });
                    s.add_layer(dialog);
                }

                s.pop_layer();
                let dialog = views::Dialog::new()
                    .title("Select an option")
                    .content(views::LinearLayout::vertical()
                        .child(views::SelectView::new()
                            .item_str("Pack Opener")
                            .item_str("Logout")
                            .with_name("select_action")
                        )
                        .child(views::Button::new("Select", move |s| {
                            let action_selector = s.call_on_name("select_action", |v: &mut views::SelectView| v.selection()).unwrap().unwrap().to_string();
                            match action_selector.as_str() {
                                "Pack Opener" => {
                                    providers::pack_opener::handler(s);
                                }
                                "Logout" => {
                                    s.pop_layer();
                                    let user = user.clone();
                                    let dialog = views::Dialog::new()
                                        .title("Logout")
                                        .content(views::TextView::new("Are you sure you want to logout?"))
                                        .button("Yes", move |s| {
                                            user.logout().unwrap();
                                            s.quit();
                                        })
                                        .button("No", |s| {
                                            s.pop_layer();
                                        });
                                    s.add_layer(dialog);
                                }
                                _ => {}
                            }
                        }))
                    )
                    .button("Quit", |s| {
                        s.quit()
                    });
                s.add_layer(dialog);
            }
        })
        .button("Quit", |s| s.quit());

    siv.add_layer(dialog);

    siv.run();
}
