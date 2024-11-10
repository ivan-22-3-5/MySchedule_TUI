use crate::scanners;

pub enum MenuOptionAction {
    Action(Box<dyn FnMut()>),
    SubMenu(Menu),
    Exit,
}

pub struct MenuOption {
    text: String,
    action: MenuOptionAction,
}

pub struct Menu {
    title: String,
    options: Vec<MenuOption>,
}

impl Menu {
    pub fn new(title: &str) -> Menu {
        Menu {
            title: title.to_string(),
            options: Vec::new(),
        }
    }

    pub fn add_option(&mut self, text: &str, action: MenuOptionAction) {
        self.options.push(MenuOption {
            text: text.to_string(),
            action,
        });
    }

    pub fn open(&mut self) {
        loop {
            println!("{}", self.title);
            self.options
                .iter()
                .enumerate()
                .for_each(|(index, option)| println!("{}. {}", index + 1, option.text));

            let selected_option: usize = scanners::scan("Invalid input, try again: ");
            if selected_option == 0 {
                break;
            }
            if let Some(menu_option) = self.options.get_mut(selected_option - 1) {
                match &mut menu_option.action {
                    MenuOptionAction::Action(action) => action(),
                    MenuOptionAction::SubMenu(sub_menu) => sub_menu.open(),
                    MenuOptionAction::Exit => break,
                }
            } else {
                println!("There is no such option, please try again: ");
            }
        }
    }
}
