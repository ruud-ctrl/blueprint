use crate::cli::UserInput;
use std::fs;
use std::path::Path;

pub fn generate_from_template(input: UserInput) -> anyhow::Result<()> {
    let template_path = format!("src/templates/{}.tmpl", input.template_choice);
    let content = fs::read_to_string(&template_path)?;

    let result = content.replace("{{project_name}}", &input.project_name);

    let output_dir = Path::new(&input.project_name);
    fs::create_dir_all(&output_dir)?;
    fs::write(output_dir.join("main.txt"), result)?;

    println!("✅ Generated project at {}/main.txt", input.project_name);
    Ok(())
}
