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
    version: u32,
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

impl Arg {
    fn to_rust_type(&self) -> &str {
        match self.ty {
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "crate::wire::Fixed",
            ArgType::String => "String",
            ArgType::Object => "crate::wire::ObjectId",
            ArgType::NewId => {
                if self.interface.is_some() {
                    "crate::wire::ObjectId"
                } else {
                    "crate::wire::NewId"
                }
            }
            ArgType::Array => "Vec<u8>",
            ArgType::Fd => "std::os::fd::RawFd",
        }
    }

    fn is_return_option(&self) -> bool {
        match self.ty {
            ArgType::String | ArgType::Object => true,
            ArgType::NewId => self.interface.is_some(),
            _ => false,
        }
    }

    fn to_caller(&self) -> &str {
        match self.ty {
            ArgType::Int => "int",
            ArgType::Uint => "uint",
            ArgType::Fixed => "fixed",
            ArgType::String => "string",
            ArgType::Object => "object",
            ArgType::NewId => {
                if self.interface.is_some() {
                    "object"
                } else {
                    "new_id"
                }
            }
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
        .open("src/protocol/interfaces.rs")?;

    writeln!(&mut generated_path, "#![allow(unused)]")?;
    writeln!(&mut generated_path, "#![allow(async_fn_in_trait)]")?;

    for path in PROTOCOLS {
        let protocol: Protocol = quick_xml::de::from_str(&fs::read_to_string(path)?)?;
        dbg!(&protocol.name);

        if let Some(description) = protocol.description {
            for line in description.lines() {
                writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
            }
        }

        writeln!(
            &mut generated_path,
            "pub mod {name} {{",
            name = &protocol.name
        )?;

        for interface in protocol.interfaces {
            writeln!(
                &mut generated_path,
                "pub mod {name} {{",
                name = interface.name
            )?;

            for enu in interface.enums {
                let mut variants = String::new();

                for entry in enu.entries {
                    let mut prefix = "";

                    if entry.name.chars().next().unwrap().is_numeric() {
                        prefix = "_"
                    }

                    if let Some(description) = entry.description {
                        for line in description.lines() {
                            writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                        }
                    }

                    variants.push_str(&format!(
                        "r#{prefix}{name},",
                        name = entry.name.to_upper_camel_case()
                    ))
                }

                if let Some(description) = enu.description {
                    for line in description.lines() {
                        writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                    }
                }

                writeln!(
                    &mut generated_path,
                    r#"#[repr(u32)]
                    #[non_exhaustive]
                    pub enum r#{name} {{{variants}}}"#,
                    name = enu.name.to_upper_camel_case()
                )?;
            }

            for line in interface.description.lines() {
                writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
            }

            writeln!(
                &mut generated_path,
                r#"pub trait r#{trait_name} {{
                    const INTERFACE: &'static str = "{name}";
                    const VERSION: u32 = {version};
                    async fn handle_request(client: &mut crate::Client, message: &mut crate::wire::Message) -> crate::Result<()> {{
                    match message.opcode {{"#,
                trait_name = interface.name.to_upper_camel_case(),
                name = interface.name,
                version = interface.version
            )?;

            for (opcode, request) in interface.requests.iter().enumerate() {
                let mut args = "client,".to_string();

                for arg in &request.args {
                    let mut optional = "".to_string();

                    if !arg.allow_null && arg.is_return_option() {
                        optional = format!(".ok_or(crate::wire::DecodeError::MalformedPayload)?");
                    }

                    args.push_str(&format!(
                        "message.{caller}()?{optional},",
                        caller = arg.to_caller()
                    ))
                }

                writeln!(
                    &mut generated_path,
                    r#"{opcode} => {{tracing::debug!("{interface_name} -> {name}");  Self::r#{name}({args}).await}}"#,
                    name = request.name.to_snek_case(),
                    interface_name = interface.name
                )?;
            }

            writeln!(
                &mut generated_path,
                "_ => Err(crate::error::Error::UnknownOpcode) }} }}"
            )?;

            writeln!(
                &mut generated_path,
                "fn create_dispatcher(id: crate::wire::ObjectId) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;"
            )?;

            for request in &interface.requests {
                let mut args = "client: &mut crate::Client,".to_string();

                for arg in &request.args {
                    let mut ty = arg.to_rust_type().to_string();

                    if arg.allow_null {
                        ty = format!("Option<{ty}>");
                    }

                    args.push_str(&format!("r#{name}: {ty},", name = arg.name.to_snek_case(),))
                }

                for line in request.description.lines() {
                    writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                }

                writeln!(
                    &mut generated_path,
                    "async fn r#{name}({args}) -> crate::Result<()>;",
                    name = request.name.to_snek_case()
                )?;
            }

            for (opcode, event) in interface.events.iter().enumerate() {
                let mut args =
                    "dispatcher_id: crate::wire::ObjectId, client: &mut crate::Client,".to_string();
                let mut build_args = String::new();

                for arg in &event.args {
                    let mut ty = arg.to_rust_type().to_string();
                    let build_ty = arg.to_caller();
                    let name = arg.name.to_snek_case();
                    let mut build_name = arg.name.to_snek_case();

                    if arg.allow_null {
                        ty = format!("Option<{ty}>");
                    }

                    if arg.is_return_option() && !arg.allow_null {
                        build_name = format!("Some({build_name})")
                    }

                    args.push_str(&format!("r#{name}: {ty},",));
                    build_args.push_str(&format!(".put_{build_ty}({build_name})",));
                }

                for line in event.description.lines() {
                    writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                }

                writeln!(
                    &mut generated_path,
                    "async fn r#{name}({args}) -> crate::Result<()> {{",
                    name = event.name.to_snek_case()
                )?;

                writeln!(
                    &mut generated_path,
                    r#"tracing::debug!("{interface_name} -> {name}");
                    let payload = crate::wire::PayloadBuilder::new()
                    {build_args}
                    .build();"#,
                    name = event.name.to_snek_case(),
                    interface_name = interface.name
                )?;

                writeln!(
                    &mut generated_path,
                    r#"client
                .send_message(crate::wire::Message::new(dispatcher_id, {opcode}, payload))
                .await
                .map_err(crate::error::Error::IoError)"#
                )?;

                writeln!(&mut generated_path, "}}")?;
            }

            writeln!(&mut generated_path, "}} }}")?;
        }
        writeln!(&mut generated_path, "}}")?;
    }

    Command::new("rustfmt")
        .arg("src/protocol/interfaces.rs")
        .output()?;

    Ok(())
}
