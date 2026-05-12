use pest_derive::Parser;
use pest::Parser;
use anyhow::{Result, anyhow};

#[derive(Parser)]
#[grammar = "core/kairo.pest"]
pub struct KairoParser;

#[derive(Debug)]
pub struct Schema {
    pub tables: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_name: String,
    pub default_value: Option<String>,
}

pub fn parse_schema(input: &str) -> Result<Schema> {
    let pairs = KairoParser::parse(Rule::schema, input)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    let mut tables = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::table => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let mut fields = Vec::new();

                for field_pair in inner {
                    let mut field_inner = field_pair.into_inner();
                    let f_name = field_inner.next().unwrap().as_str().to_string();
                    let f_type = field_inner.next().unwrap().as_str().to_string();
                    let f_default = field_inner.next().map(|p| p.as_str().to_string());

                    fields.push(Field {
                        name: f_name,
                        type_name: f_type,
                        default_value: f_default,
                    });
                }

                tables.push(Table { name, fields });
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Schema { tables })
}
