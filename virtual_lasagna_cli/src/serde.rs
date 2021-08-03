#[allow(unused_imports)]
use std::io::{Read, Write};

use crate::{constant, schema};

fn open_toml_file(truncate: bool) -> std::fs::File {
    let mut oo = &mut std::fs::OpenOptions::new();
    oo = oo.read(true).write(true);
    if truncate {
        oo = oo.truncate(true);
    }

    oo.open(constant::TOML_PATH).unwrap()
}

fn convert_to_dfsd(s: schema::Schema) -> schema::SchemaForSerde {
    let schema::Schema {
        user,
        max_num,
        mut posts,
    } = s;

    let posts = posts
        .drain(..)
        .map(|v| {
            let schema::Post {
                num,
                content,
                created,
                updated,
                is_deleted,
            } = v;
            schema::PostForSerde {
                num,
                content,
                created: created.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
                updated: updated.map(|v| v.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)),
                is_deleted,
            }
        })
        .collect::<smallvec::SmallVec<_>>();

    schema::SchemaForSerde {
        user,
        max_num,
        posts,
    }
}

fn convert_from_dfsd(s: schema::SchemaForSerde) -> schema::Schema {
    let schema::SchemaForSerde {
        user,
        max_num,
        mut posts,
    } = s;

    let posts = posts
        .drain(..)
        .map(|v| {
            let schema::PostForSerde {
                num,
                content,
                created,
                updated,
                is_deleted,
            } = v;
            schema::Post {
                num,
                content,
                created: chrono::prelude::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
                    created.as_str(),
                )
                .unwrap()
                .into(),
                updated: updated.map(|v| {
                    chrono::prelude::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(v.as_str())
                        .unwrap()
                        .into()
                }),
                is_deleted,
            }
        })
        .collect::<smallvec::SmallVec<_>>();

    schema::Schema {
        user,
        max_num,
        posts,
    }
}

pub fn de() -> schema::Schema {
    let r = de_inner();

    convert_from_dfsd(r.unwrap())
}

pub fn de_inner() -> anyhow::Result<schema::SchemaForSerde> {
    let mut f = open_toml_file(false);
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(toml::de::from_str::<schema::SchemaForSerde>(buf.as_str())?)
}

pub fn ser(data: schema::Schema) {
    let r: anyhow::Result<()> = try {
        let data = convert_to_dfsd(data);
        let s = toml::ser::to_string(&data)?;
        let mut f = open_toml_file(true);
        f.write_all(s.as_bytes())?;
    };

    r.unwrap();
}
