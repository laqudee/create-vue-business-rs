use crate::utils::util::{is_valid_package_name, to_valid_package_name};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::ConfiguresSelected;

pub fn work(configures: &mut ConfiguresSelected) -> (String, &ConfiguresSelected) {
    let selections = &["web", "h5"];

    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    let mut project_name: String = Input::with_theme(&theme)
        .with_prompt("projectName")
        .default("vue-business-project".to_string())
        .interact_on(&term)
        .unwrap();

    if !is_valid_package_name(&project_name) {
        println!(
            "! Invalid package.json name `{}`, Automatically converted to a valid name.",
            project_name
        );
        project_name = to_valid_package_name(&project_name)
    }
    println!("! Current project name: {}", project_name);

    let config_value = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Determine Web(Element-plus) or H5(Vant) project creation?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    configures.set_target_tag(&selections[config_value]);

    (project_name, configures)
}
