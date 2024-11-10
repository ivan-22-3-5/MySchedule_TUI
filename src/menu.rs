use crate::scanners;

pub struct MenuOption {
    text: String,
    action: Box<dyn FnMut()>,
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

    pub fn add_option(&mut self, text: &str, action: impl FnMut() + 'static) {
        self.options.push(MenuOption {
            text: text.to_string(),
            action: Box::new(action),
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
                (menu_option.action)();
            } else {
                println!("There is no such option, please try again: ");
            }
        }
    }
}
