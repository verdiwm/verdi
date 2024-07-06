use anyhow::Result;
use heck::{ToSnekCase, ToUpperCamelCase};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Protocol {
    #[serde(rename = "@name")]
    name: String,
    copyright: String,
    description: Option<String>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Interface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: usize,
    description: String,
    #[serde(default, rename = "request")]
    requests: Vec<Message>,
    #[serde(default, rename = "event")]
    events: Vec<Message>,
    #[serde(default, rename = "enum")]
    enums: Vec<Enum>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Message {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    ty: Option<MessageType>,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: String,
    #[serde(default, rename = "arg")]
    args: Vec<Arg>,
}

#[derive(Debug, Deserialize, Serialize)]
enum MessageType {
    #[serde(rename = "destructor")]
    Destructor,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Arg {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    ty: ArgType,
    #[serde(rename = "@interface")]
    interface: Option<String>,
    #[serde(rename = "@enum")]
    r#enum: Option<String>,
    #[serde(default, rename = "@allow-null")]
    allow_null: bool,
    #[serde(rename = "@summary")]
    summary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
enum ArgType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "uint")]
    Uint,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "new_id")]
    NewId,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "fd")]
    Fd,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Enum {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@bitfield")]
    bitfield: bool,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: Option<String>,
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@summary")]
    summary: Option<String>,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: Option<String>,
}

impl ArgType {
    fn to_rust_type(&self) -> &str {
        match self {
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "Fixed",
            ArgType::String => "String",
            ArgType::Object => "ObjectId",
            ArgType::NewId => "NewId",
            ArgType::Array => "Vec<u8>",
            ArgType::Fd => "RawFd",
        }
    }

    fn needs_borrow(&self) -> bool {
        match self {
            ArgType::String | ArgType::Array => true,
            _ => false,
        }
    }

    fn is_return_option(&self) -> bool {
        match self {
            ArgType::String | ArgType::Object => true,
            _ => false,
        }
    }

    fn to_caller(&self) -> &str {
        match self {
            ArgType::Int => "int",
            ArgType::Uint => "uint",
            ArgType::Fixed => "fixed",
            ArgType::String => "string",
            ArgType::Object => "object",
            ArgType::NewId => "new_id",
            ArgType::Array => "array",
            ArgType::Fd => "int",
        }
    }
}

const PROTOCOLS: [&str; 6] = [
    "wayland/protocol/wayland.xml",
    "wayland-protocols/stable/linux-dmabuf/linux-dmabuf-v1.xml",
    "wayland-protocols/stable/presentation-time/presentation-time.xml",
    "wayland-protocols/stable/tablet/tablet-v2.xml",
    "wayland-protocols/stable/viewporter/viewporter.xml",
    "wayland-protocols/stable/xdg-shell/xdg-shell.xml",
];

fn main() -> Result<()> {
    let mut generated_path = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("src/proto.rs")?;

    writeln!(&mut generated_path, "#![allow(unused)]")?;

    for path in PROTOCOLS {
        let protocol: Protocol = quick_xml::de::from_str(&fs::read_to_string(path)?)?;
        dbg!(&protocol.name);

        writeln!(
            &mut generated_path,
            "pub mod {name} {{",
            name = &protocol.name
        )?;

        writeln!(
            &mut generated_path,
            "use crate::{{Result, message::{{Message,Fixed,ObjectId,NewId}}, error::Error, Client}};"
        )?;
        writeln!(&mut generated_path, "use std::os::fd::RawFd;")?;

        for interface in protocol.interfaces {
            writeln!(
                &mut generated_path,
                "pub trait r#{name} {{",
                name = interface.name.to_upper_camel_case()
            )?;

            writeln!(
                &mut generated_path,
                "fn handle_request(client: &Client, message: &mut Message) -> Result<()> {{"
            )?;

            writeln!(&mut generated_path, "match message.opcode {{")?;

            for (opcode, request) in interface.requests.iter().enumerate() {
                let mut args = String::new();

                for arg in &request.args {
                    // let mut borrowed = "";

                    // if arg.ty.needs_borrow() {
                    //     borrowed = "&";
                    // }

                    let mut optional = "".to_string();

                    if !arg.allow_null && arg.ty.is_return_option() {
                        optional = format!(".unwrap()");
                    }

                    args.push_str(&format!(
                        "message.{caller}()?{optional},",
                        caller = arg.ty.to_caller()
                    ))
                }

                writeln!(
                    &mut generated_path,
                    "{opcode} => Self::r#{name}({args}),",
                    name = request.name.to_snek_case(),
                )?;
            }

            writeln!(&mut generated_path, "_ => Err(Error::UnknownOpcode)")?;
            writeln!(&mut generated_path, "}}")?;

            writeln!(&mut generated_path, "}}")?;

            for request in &interface.requests {
                let mut args = String::new();

                for arg in &request.args {
                    let mut ty = arg.ty.to_rust_type().to_string();

                    if arg.allow_null {
                        ty = format!("Option<{ty}>");
                    }

                    args.push_str(&format!("r#{name}: {ty},", name = arg.name.to_snek_case(),))
                }

                writeln!(
                    &mut generated_path,
                    "fn r#{name}({args}) -> Result<()>;",
                    name = request.name.to_snek_case()
                )?;
            }

            writeln!(&mut generated_path, "}}")?;
        }
        writeln!(&mut generated_path, "}}")?;
    }

    Command::new("rustfmt").arg("src/proto.rs").output()?;

    Ok(())
}
