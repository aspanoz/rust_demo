use serde_derive::Deserialize;
use std::char;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, Write};
use toml;

static SETINGS: &'static str = "settings.toml";
static CSOUND: &'static str = "csound/variables.csd";

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

struct ParamsTable {
    id: u8,      // index таблицы csound, первый параметр у ftgen
    size: usize, // кол-во элементов в таблице
}

impl fmt::Display for ParamsTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
/*
данные инициализации параметров Borovylo
для инициализации синтезатора параметра необходимо 4 атрибута

vtabwi <MIDI CC>, giP, <increment>, <min>, <max>, <value>
где:
  increment - величина изменения параметра
  min       - допустимый минимум
  max       - допустимый максимум
  value     - значение по умолчанию
*/

;{delim:>15}кол-во параметров * 4 атрибута
;{delim:>15}|
giP ftgen {id}, 0, {size}, -7, 0

; ДАННЫЕ ПАРАМЕТРОВ"#,
            delim = " ",
            id = self.id,
            size = self.size
        )
    }
}

struct LayoutsTable {
    id: u8,      // index таблицы csound, первый параметр у ftgen
    size: usize, // кол-во элементов в таблице
}

impl fmt::Display for LayoutsTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"

/*
данные инициализации кнопок раскладок по 8 на раскладку
для инициализации синтезатора параметра необходимо 3 атрибута

vtabwi <row id>, giB, <pid>, <lid>, <position>
где:
  lid      - индекс раскладки
  pid      - индекс параметра в таблице giP
  position - индекс одной из 8ми позиций кнопки в раскладке
             где индекс позиции:
               1 2  3 4
               5 6  7 8
*/

;{delim:>15}кол-во записей * 3 атрибута
;{delim:>15}|
giB ftgen {id}, 0, {size}, -7, 0

; раскладка A"#,
            delim = " ",
            id = self.id,
            size = self.size
        )
    }
}

fn settings_parse() -> Result<()> {
    let mut context = String::new();
    File::open(SETINGS).and_then(|mut f| f.read_to_string(&mut context))?;
    let mut s: Config = toml::from_str(&context)?;

    /*
     * CSOUND
     */
    let mut csound = File::create(CSOUND)?;
    writeln!(
        &mut csound,
        "; this file was auto-generated from settings.toml!"
    )?;

    // csound: объявление таблицы giP нужного размера
    let pdata = ParamsTable {
        id: 0,
        size: 4 * (s.params.len() + 1), // @FIX + 1 потому что первый нулевой блок данных
    };
    writeln!(&mut csound, "{}", pdata)?;

    // csound: заполнение данными таблицы giP
    for cs_param_data in &s.params {
        writeln!(&mut csound, "{}", cs_param_data)?;
    }

    writeln!(&mut csound, "\n; названия параметров")?;

    // csound: привязать label параметра к index'у параметра
    for p in &s.params {
        writeln!(
            &mut csound,
            r#"strset {id:>2}, "{label}""#,
            id = p.cc,
            label = p.label
        )?;
    }

    // csound: объявление таблицы liB нужного размера
    let ldata = LayoutsTable {
        id: 1,
        size: 3 * s.layouts.len(),
    };
    writeln!(&mut csound, "{}", ldata)?;

    let mut curr = 0;
    for (i, mut l) in s.layouts.iter_mut().enumerate() {
        if curr != l.lid {
            curr = l.lid.to_owned();
            let label: char = (65 + curr).into();
            writeln!(&mut csound, "\n; раскладка {}", label)?;
        }
        l.row = Some(i);
        writeln!(&mut csound, "{}", l)?;
    }

    writeln!(
        &mut csound,
        r#"
; всего раскладок
giL_len init {len}"#,
        len = curr + 1
    )?;

    Ok(())
}

pub(super) fn main() {
    if let Err(e) = settings_parse() {
        eprintln!("Error: {}", e);
    }
}
