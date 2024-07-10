use anyhow::Result;
use heck::{ToSnekCase, ToUpperCamelCase};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write as _,
    fs::{self, OpenOptions},
    io::Write as _,
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
    fn to_enum_name(&self) -> Option<String> {
        if let Some(e) = &self.r#enum {
            if let Some((_, name)) = e.split_once('.') {
                return Some(name.to_string());
            } else {
                return Some(e.to_string());
            }
        }

        None
    }

    fn to_rust_type(&self) -> String {
        if let Some(e) = &self.r#enum {
            if let Some((module, name)) = e.split_once('.') {
                return format!("super::{module}::{name}", name = name.to_upper_camel_case());
            } else {
                return e.to_upper_camel_case();
            }
        }

        let ret = match self.ty {
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
            ArgType::Fd => "rustix::fd::OwnedFd",
        };

        ret.to_string()
    }

    fn is_return_option(&self) -> bool {
        match self.ty {
            ArgType::String | ArgType::Object => true,
            ArgType::NewId => self.interface.is_some(),
            _ => false,
        }
    }

    fn to_caller(&self) -> &str {
        if self.r#enum.is_some() {
            return "uint";
        }

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
            ArgType::Fd => "fd",
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

const KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
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

        if let Some(description) = &protocol.description {
            for line in description.lines() {
                writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
            }
        }

        writeln!(
            &mut generated_path,
            "pub mod {name} {{",
            name = &protocol.name
        )?;

        for interface in &protocol.interfaces {
            writeln!(
                &mut generated_path,
                "pub mod {name} {{",
                name = interface.name
            )?;

            for enu in &interface.enums {
                if !enu.bitfield {
                    let mut variants = String::new();
                    let mut match_variants = String::new();

                    for entry in &enu.entries {
                        let mut prefix = "";

                        if entry.name.chars().next().unwrap().is_numeric() {
                            prefix = "_"
                        }

                        if let Some(summary) = &entry.summary {
                            for line in summary.lines() {
                                let doc = line.trim();

                                let mut c = doc.chars();
                                let doc = c.next().unwrap().to_uppercase().collect::<String>()
                                    + c.as_str();

                                writeln!(&mut variants, r##"#[doc = r#"{doc}"#]"##,)?;
                            }
                        }

                        let name = entry.name.to_upper_camel_case();
                        let value = &entry.value;

                        variants.push_str(&format!("{prefix}{name} = {value},",));

                        match_variants.push_str(&format!("{value} => Ok(Self::{prefix}{name}),"));
                    }

                    if let Some(description) = &enu.description {
                        for line in description.lines() {
                            writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                        }
                    }

                    let mut name = enu.name.to_upper_camel_case();

                    if KEYWORDS.contains(&name.as_str()) {
                        name = format!("r#{name}")
                    }

                    writeln!(
                        &mut generated_path,
                        r#"#[repr(u32)]
                        #[non_exhaustive]
                        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
                        pub enum {name} {{{variants}}}"#,
                    )?;

                    writeln!(
                        &mut generated_path,
                        r#"impl TryFrom<u32> for {name} {{
                            type Error = crate::wire::DecodeError;
        
                            fn try_from(v: u32) -> Result<Self, Self::Error> {{
                                match v {{
                                    {match_variants}
                                    _ => Err(crate::wire::DecodeError::MalformedPayload)
                                }}
                            }}
                        }}"#
                    )?;

                    // writeln!(
                    //     &mut generated_path,
                    //     r#"impl std::fmt::Display for {name} {{
                    //         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                    //             write!(f, "{{}}", *self as u32)
                    //         }}
                    //     }}"#
                    // )?;
                } else {
                    let mut variants = String::new();

                    for entry in &enu.entries {
                        let mut prefix = "";

                        if entry.name.chars().next().unwrap().is_numeric() {
                            prefix = "_"
                        }

                        if let Some(summary) = &entry.summary {
                            for line in summary.lines() {
                                let doc = line.trim();

                                let mut c = doc.chars();
                                let doc = c.next().unwrap().to_uppercase().collect::<String>()
                                    + c.as_str();

                                writeln!(&mut variants, r##"#[doc = r#"{doc}"#]"##,)?;
                            }
                        }

                        variants.push_str(&format!(
                            "const {prefix}{name} = {value};",
                            name = entry.name.to_upper_camel_case(),
                            value = entry.value
                        ))
                    }

                    if let Some(description) = &enu.description {
                        for line in description.lines() {
                            writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                        }
                    }

                    let name = enu.name.to_upper_camel_case();

                    writeln!(
                        &mut generated_path,
                        r#"bitflags::bitflags! {{
                            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
                            pub struct {name}: u32 {{{variants}}}
                        }}"#,
                    )?;

                    writeln!(
                        &mut generated_path,
                        r#"impl TryFrom<u32> for {name} {{
                            type Error = crate::wire::DecodeError;
        
                            fn try_from(v: u32) -> Result<Self, Self::Error> {{
                               Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
                            }}
                        }}"#
                    )?;
                }
            }

            for line in interface.description.lines() {
                writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
            }

            writeln!(
                &mut generated_path,
                r#"pub trait {trait_name}: crate::Dispatcher {{
                    const INTERFACE: &str = "{name}";
                    const VERSION: u32 = {version};

                    fn into_object(self, id: crate::ObjectId) -> crate::Object where Self: Sized
                    {{
                        crate::Object::new(id, self)
                    }}
                    
                    async fn handle_request(&self, object: &crate::Object, client: &mut crate::Client, message: &mut crate::wire::Message) -> crate::Result<()> {{
                    match message.opcode {{"#,
                trait_name = interface.name.to_upper_camel_case(),
                name = interface.name,
                version = interface.version
            )?;

            for (opcode, request) in interface.requests.iter().enumerate() {
                let mut args = "object,client,".to_string();

                for arg in &request.args {
                    let mut optional = String::new();

                    if !arg.allow_null && arg.is_return_option() {
                        optional = format!(".ok_or(crate::wire::DecodeError::MalformedPayload)?");
                    }

                    let mut tryinto = String::new();

                    if arg.r#enum.is_some() {
                        tryinto.push_str(".try_into()?")
                    }

                    args.push_str(&format!(
                        "message.{caller}()?{optional}{tryinto},",
                        caller = arg.to_caller()
                    ))
                }

                let mut name = request.name.to_snek_case();

                if KEYWORDS.contains(&name.as_str()) {
                    name = format!("r#{name}")
                }

                writeln!(
                    &mut generated_path,
                    r#"{opcode} => {{tracing::debug!("{interface_name}#{{}}.{og_name}()", object.id);  self.{name}({args}).await}}"#,
                    og_name = request.name.to_snek_case(),
                    interface_name = interface.name
                )?;
            }

            writeln!(
                &mut generated_path,
                "_ => Err(crate::error::Error::UnknownOpcode) }} }}"
            )?;

            for request in &interface.requests {
                let mut args =
                    "&self, _object: &crate::Object, client: &mut crate::Client,".to_string();

                for arg in &request.args {
                    let mut ty = arg.to_rust_type().to_string();

                    if arg.allow_null {
                        ty = format!("Option<{ty}>");
                    }

                    let mut name = request.name.to_snek_case();

                    if KEYWORDS.contains(&name.as_str()) {
                        name = format!("r#{name}")
                    }

                    args.push_str(&format!("{name}: {ty},"))
                }

                for line in request.description.lines() {
                    writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                }

                let mut name = request.name.to_snek_case();

                if KEYWORDS.contains(&name.as_str()) {
                    name = format!("r#{name}")
                }

                writeln!(
                    &mut generated_path,
                    "async fn {name}({args}) -> crate::Result<()>;",
                )?;
            }

            for (opcode, event) in interface.events.iter().enumerate() {
                let mut args =
                    "&self, _object: &crate::Object, client: &mut crate::Client,".to_string();
                let mut build_args = String::new();
                let mut tracing_args = String::new();
                let mut num_tracing_args = 0usize;

                for arg in &event.args {
                    let mut ty = arg.to_rust_type().to_string();
                    let build_ty = arg.to_caller();
                    let mut name = arg.name.to_snek_case();
                    let mut build_name = arg.name.to_snek_case();

                    if KEYWORDS.contains(&name.as_str()) {
                        name = format!("r#{name}")
                    }

                    if KEYWORDS.contains(&build_name.as_str()) {
                        build_name = format!("r#{name}")
                    }

                    if let Some(name) = arg.to_enum_name() {
                        let e = find_enum(&protocol, &name);

                        if e.bitfield {
                            build_name.push_str(".bits()");
                        } else {
                            build_name.push_str(" as u32");
                        }
                    }

                    if arg.allow_null {
                        ty = format!("Option<{ty}>");
                    }

                    if arg.is_return_option() && !arg.allow_null {
                        build_name = format!("Some({build_name})")
                    }

                    args.push_str(&format!("{name}: {ty},",));
                    build_args.push_str(&format!(".put_{build_ty}({build_name})",));
                    tracing_args.push_str(&format!("{name},"));
                    num_tracing_args += 1;
                }

                for line in event.description.lines() {
                    writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
                }

                let mut name = event.name.to_snek_case();

                if KEYWORDS.contains(&name.as_str()) {
                    name = format!("r#{name}")
                }

                writeln!(
                    &mut generated_path,
                    "async fn {name}({args}) -> crate::Result<()> {{",
                )?;

                let mut tracing_brackets = String::new();
                (0..num_tracing_args).for_each(|_| tracing_brackets.push_str("{}, "));

                let tracing_brackets = tracing_brackets
                    .strip_suffix(", ")
                    .unwrap_or(&tracing_brackets);

                writeln!(
                    &mut generated_path,
                    r#"tracing::debug!("-> {interface_name}#{{}}.{og_name}()", _object.id);
                    let (payload,fds) = crate::wire::PayloadBuilder::new()
                    {build_args}
                    .build();"#,
                    og_name = event.name.to_snek_case(),
                    interface_name = interface.name,
                )?;

                writeln!(
                    &mut generated_path,
                    r#"client
                .send_message(crate::wire::Message::new(_object.id, {opcode}, payload, fds))
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

fn find_enum<'a>(protocol: &'a Protocol, name: &str) -> &'a Enum {
    protocol
        .interfaces
        .iter()
        .find_map(|interface| interface.enums.iter().find(|e| e.name == name))
        .unwrap()
}
