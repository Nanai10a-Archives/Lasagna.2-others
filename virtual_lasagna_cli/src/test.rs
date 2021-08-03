#[allow(unused_imports)]
use std::io::{Read, Write};

#[test]
fn parse_command_examples() {
    assert_eq!(
        "some strings ok?".split(' ').collect::<Vec<_>>(),
        vec!["some", "strings", "ok?"]
    );

    assert_eq!(
        "over 1  spaces   ...".split(' ').collect::<Vec<_>>(),
        vec!["over", "1", "", "spaces", "", "", "..."]
    );

    assert_eq!(
        vec!["empty", "", "removes", "", ""]
            .drain(..)
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>(),
        vec!["empty", "removes"]
    );

    let splitted = "".split(' ').collect::<smallvec::SmallVec<[&str; 8]>>();
    assert_eq!(splitted.len(), 1);
    assert_eq!(splitted[0], "");
}

#[test]
fn time_logic() {
    let time = chrono::offset::Local::now();
    let time_str = time.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
    println!("current time (rfc3339 with nanoseconds): {}", time_str);

    // nanosecondsまで入れることで情報をロス0で保存できる
    assert_eq!(
        chrono::DateTime::parse_from_rfc3339(time_str.as_str()).unwrap(),
        time
    );
}

#[test]
fn toml_test() {
    use serde::{Deserialize, Serialize};
    use smallvec::*;
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        string: String,
        number: u32,
        vec: SmallVec<[u32; 8]>,
        nullable: Option<u32>,
    }

    // serializeの挙動確認
    {
        let ts = TestStruct {
            string: "てすてす".to_string(),
            number: 7,
            vec: smallvec![0, 1, 2, 3, 4],
            nullable: Some(0),
        };

        let tomlized = toml::ser::to_string(&ts).unwrap();
        println!("{}", tomlized);
    }

    // nullableはOption<_>で表現可能
    {
        let ts = TestStruct {
            string: "てすてすっと".to_string(),
            number: 334,
            vec: smallvec![0, 2, 3, 4],
            nullable: None,
        };

        let tomlized = toml::ser::to_string(&ts).unwrap();
        println!("{}", tomlized);
    }

    // 配列を直接serializeする時はtableが`[[]]`となる
    // ここつまづきポイント
    {
        let tsv: SmallVec<[TestStruct; 3]> = smallvec![
            TestStruct {
                string: "ひとつめ".to_string(),
                number: 0,
                vec: smallvec![],
                nullable: None,
            },
            TestStruct {
                string: "ふたつめ".to_string(),
                number: 1,
                vec: smallvec![],
                nullable: None,
            },
            TestStruct {
                string: "みっつめ".to_string(),
                number: 2,
                vec: smallvec![],
                nullable: None,
            }
        ];

        let tomlized = toml::ser::to_string(&tsv).unwrap();
        println!("{}", tomlized);
    }

    #[derive(Serialize, Deserialize)]
    struct SmallStruct {
        data: String,
        number: u32,
    }
    #[derive(Serialize, Deserialize)]
    struct Vec2Struct {
        vec1: SmallVec<[SmallStruct; 8]>,
        vec2: SmallVec<[SmallStruct; 8]>,
    }

    // `[[{name}]]`としてほしくばnameはfield_nameですのでお間違いなく
    // (これで2hくらい溶かした
    {
        let v2s = Vec2Struct {
            vec1: smallvec![
                SmallStruct {
                    data: "hoge".to_string(),
                    number: 1,
                },
                SmallStruct {
                    data: "fuga".to_string(),
                    number: 2
                }
            ],
            vec2: smallvec![
                SmallStruct {
                    data: "any".to_string(),
                    number: 666,
                },
                SmallStruct {
                    data: "unknown".to_string(),
                    number: 777,
                }
            ],
        };

        let tomlized = toml::ser::to_string(&v2s).unwrap();
        println!("{}", tomlized);
    }
}

#[test]
fn file_io_test() {
    use std::fs;

    println!(
        "current temporary dir: {}",
        std::env::temp_dir().to_string_lossy()
    );

    let mut path = std::env::temp_dir();
    path.push(format!("cargo-test-{}", chrono::Local::now().timestamp()));

    println!("temporary file path: {}", path.to_string_lossy());

    let open = || {
        fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path.clone())
            .unwrap()
    };

    {
        let mut f = open();
        let mut buf = String::new();

        let r = f.read_to_string(&mut buf);

        // 空ファイルからは何も読めない.
        assert!(r.is_ok());
        let bytes = r.unwrap();
        assert_eq!(bytes, 0);
        assert_eq!(buf, "".to_string());
    }

    let first_text = "test some string";

    {
        let mut f = open();
        let buf = first_text.as_bytes();

        let r: anyhow::Result<_> = try {
            f.write_all(buf)?;
            f.flush()
        };

        // 書き込み成功の確認.
        assert!(r.is_ok());
    }

    {
        let mut f = open();

        let mut buf = String::new();
        let r = f.read_to_string(&mut buf);

        // 書き込まれた内容の読み込み.
        assert!(r.is_ok());
        assert_eq!(buf, first_text);
    }

    let second_text = "other string";

    {
        let mut f = open();
        let buf = second_text.as_bytes();

        let r: anyhow::Result<_> = try {
            f.write_all(buf)?;
            f.flush()
        };

        // 再度書き込みの成功を確認.
        assert!(r.is_ok());
    }

    {
        let mut f = open();
        let mut buf = String::new();

        let r = f.read_to_string(&mut buf);

        // 書き込んだ分だけ上書きされていることの確認.
        assert!(r.is_ok());
        let current_text = second_text.to_owned() + first_text.split_at(second_text.len()).1;
        assert_eq!("other stringring", current_text);
        assert_eq!(buf, current_text);
    }

    {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(path.clone())
            .unwrap();
        let mut buf = String::new();

        let r = f.read_to_string(&mut buf);

        // truncateで切り捨てられたことの確認.
        assert!(r.is_ok());
        assert_eq!(buf, "");
    }

    let astr = "AAAAAAA";

    {
        let mut f = open();
        let buf = astr.as_bytes();

        let r: anyhow::Result<_> = try {
            f.write_all(buf)?;
            f.flush()
        };

        // 書き込めたことの確認
        assert!(r.is_ok());
    }

    let bstr = "BBB";

    {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.clone())
            .unwrap();

        let buf = bstr.as_bytes();

        let r: anyhow::Result<_> = try {
            f.write_all(buf)?;
            f.flush()
        };

        // 書き込めたことの確認.
        assert!(r.is_ok());
    }

    {
        let mut f = open();
        let mut buf = String::new();

        let r = f.read_to_string(&mut buf);

        // 追記での書き込みの確認.
        assert!(r.is_ok());
        assert_eq!(buf, astr.to_owned() + bstr);
    }
}
