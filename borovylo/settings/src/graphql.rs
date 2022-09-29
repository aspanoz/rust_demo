use serde_derive::Deserialize;
use std::char;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, Write};
use toml;

static SETINGS: &'static str = "settings.toml";
static GRAPHWQL: &'static str = "graphql/src/schema/init.rs";

// @FIX
#[derive(Debug, Deserialize)]
struct Config {
    params: Vec<Param>,
    layouts: Vec<Layout>,
}

#[derive(Debug, Deserialize)]
struct Layout {
    lid: u8,
    pid: u8,
    position: u8,
    action: String,
    row: Option<usize>,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "vtabwi {row:>2}, giB, {pid:>2}, {lid}, {position}",
            row = match self.row {
                Some(rid) => rid,
                _ => 0,
            },
            lid = self.lid,
            pid = self.pid,
            position = self.position
        )
    }
}

#[derive(Debug, Deserialize)]
struct Param {
    cc: u8,
    label: String,
    increment: f64,
    min: f64,
    max: f64,
    value: f64,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "vtabwi {cc:>2}, giP, {increment:>6}, {min:>5}, {max:>3}, {value:>5} ; {label}",
            cc = self.cc,
            increment = self.increment,
            min = self.min,
            max = self.max,
            value = self.value,
            label = self.label
        )
    }
}

fn settings_parse() -> Result<()> {
    let mut context = String::new();
    File::open(SETINGS).and_then(|mut f| f.read_to_string(&mut context))?;
    let s: Config = toml::from_str(&context)?;

    /*
     * RUST
     */
    let mut rust = File::create(GRAPHWQL)?;
    writeln!(
        &mut rust,
        "// this file was auto-generated from settings.toml\n"
    )?;
    writeln!(
        &mut rust,
        "use super::{{layout::{{ButtonUpdate, Layout}}, param::Param, Action}};"
    )?;
    writeln!(&mut rust, "use slab::Slab;")?;
    writeln!(
        &mut rust,
        r#"
impl Layout {{
{s:>4}pub fn init() -> Slab<Layout> {{
{s:>8}let mut init = Slab::new();

{s:>8}let mut layout = vec![ButtonUpdate::new(None, Action::None); 8];"#,
        s = " "
    )?;

    let mut curr = 0;
    for l in s.layouts {
        if curr != l.lid {
            let label: char = (65 + curr).into();
            curr = l.lid;
            writeln!(
                &mut rust,
                "\n{s:>8}init.insert(Layout::new(\"{label}\", layout.to_owned()));\n",
                s = " ",
                label = label
            )?;
            writeln!(
                &mut rust,
                "{s:>8}layout = vec![ButtonUpdate::new(None, Action::None); 8];",
                s = " "
            )?;
        }
        writeln!(
            &mut rust,
            "{s:>8}layout[{btn}] = ButtonUpdate::new(Some({id}), Action::{action});",
            s = " ",
            btn = l.position - 1,
            id = l.pid - 1,
            action = l.action
        )?;
    }

    let label: char = (65 + curr).into();
    writeln!(
        &mut rust,
        r#"
{s:>8}init.insert(Layout::new("{label}", layout.to_owned()));

{s:>8}return init;
{s:>4}}}
}}"#,
        s = " ",
        label = label
    )?;

    writeln!(
        &mut rust,
        r#"
impl Param {{
{s:>4}pub fn init() -> Slab<Param> {{
{s:>8}let mut init = Slab::new();
"#,
        s = " "
    )?;

    for p in &s.params {
        writeln!(
            &mut rust,
            r#"{s:>8}init.insert(Param::new({id}, "{label}"));"#,
            s = " ",
            id = p.cc,
            label = p.label
        )?;
    }

    writeln!(
        &mut rust,
        r#"
{s:>8}return init;
{s:>4}}}
}}"#,
        s = " "
    )?;

    Ok(())
}

pub(super) fn main() {
    if let Err(e) = settings_parse() {
        eprintln!("Error: {}", e);
    }
}
