pub struct App {}

impl Default for App {
    fn default() -> Self {
        App {}
    }
}

impl ksni::Tray for App {
    fn icon_theme_path(&self) -> String {
        let icon_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("");
        icon_path.into_os_string().into_string().unwrap()
    }

    fn icon_name(&self) -> String {
        "trayicon".into()
    }

    fn title(&self) -> String {
        "Zazyr".into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        vec![
            StandardItem {
                enabled: false,
                label: self.title(),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(|_| {
                    println!("EXIT");
                    std::process::exit(0)
                }),
                ..Default::default()
            }
            .into(),
        ]
    }

    fn id(&self) -> String {
        "zazyr".into()
    }
}
