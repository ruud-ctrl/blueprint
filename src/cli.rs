use dialoguer::{Input, Select};

pub struct UserInput {
    pub project_name: String,
    pub template_choice: String,
}

pub fn run_menu() -> anyhow::Result<UserInput> {
    let project_name: String = Input::new()
        .with_prompt("Enter your project name")
        .interact_text()?;

    let templates = vec!["hello_world"];
    let choice = Select::new()
        .with_prompt("Choose a template")
        .items(&templates)
        .default(0)
        .interact()?;

    Ok(UserInput {
        project_name,
        template_choice: templates[choice].to_string(),
    })
}
