use std::fs;
use std::io::{Result, Write};

static TOML_SOURCES: &'static str = "./Cargo.toml";
static CONSTANTS: &'static str = "src/vars.rs";
static CSOUND_PATH: &'static str = "borovylo.csd";

/// Читает данные из Cargo.toml и добавлет их в проект
fn generate_toml_variables() -> Result<()> {
   println!("cargo:rerun-if-changed={}/*", TOML_SOURCES);
   println!("cargo:rerun-if-changed={}/*", CSOUND_PATH);

   let toml_str = fs::read_to_string(TOML_SOURCES)?;
   let value = toml::from_str::<toml::Value>(&toml_str).unwrap();

   let mut file = fs::File::create(CONSTANTS)?;
   writeln!(&mut file, "#![allow(dead_code)]")?;
   writeln!(&mut file, "#![allow(unused_variables)]")?;
   writeln!(&mut file, r#"/// Этот файл автоматически создан скриптом build.rs"#)?;

   writeln!(&mut file, "")?;
   writeln!(&mut file, r#"/// Версия приложения, данные из {TOML_SOURCES}"#)?;
   writeln!(
      &mut file,
      r#"pub static VERSION: &'static str = {};"#,
      value["package"]["version"]
   )?;

   writeln!(&mut file, "")?;
   writeln!(
      &mut file,
      r#"/// Название исполняемого файла, данные из {TOML_SOURCES}"#
   )?;
   writeln!(
      &mut file,
      r#"pub static APP_ID: &'static str = {};"#,
      value["package"]["name"]
   )?;

   writeln!(&mut file, "")?;
   writeln!(&mut file, r#"/// Описание приложения, данные из {TOML_SOURCES}"#)?;
   writeln!(
      &mut file,
      r#"pub static DESCRIPTION: &'static str = {};"#,
      value["package"]["description"]
   )?;

   writeln!(&mut file, "")?;
   writeln!(&mut file, r#"/// Название приеложения, данные из {TOML_SOURCES}"#)?;
   writeln!(
      &mut file,
      r#"pub static LABEL: &'static str = {};"#,
      value["package"]["metadata"]["label"]
   )?;

   writeln!(&mut file, "")?;
   writeln!(&mut file, r#"/// Порт WebSocket сервера"#)?;
   writeln!(&mut file, r#"pub static PORT: u16 = 5000;"#,)?;

   let file_context = fs::read_to_string(CSOUND_PATH)?;
   println!("{file_context}");

   writeln!(&mut file, "")?;
   writeln!(&mut file, r#"/// csound"#)?;
   writeln!(
      &mut file,
      r###"pub static ORC: &'static str = r##"{}"##;"###,
      file_context
   )?;
   // writeln!(&mut file, r###"   r##"{}"##);"###, file_context)?;

   // writeln!(&mut file, "  response.insert(\"{}\",", file_name)?;
   // writeln!(&mut file, r###"   r##"{}"##);"###, file_context)?;
   // writeln!(&mut file, "")?;

   // writeln!(&mut file, r#"/// Порт WebSocket сервера"#)?;
   // writeln!(&mut file, r#"pub static PORT: u16 = 5000;"#,)?;

   Ok(())
}

fn main() {
   // Читает данные из Cargo.toml и добавлет их в проект
   if let Err(e) = generate_toml_variables() {
      eprintln!("Error: {}", e);
   }
}
