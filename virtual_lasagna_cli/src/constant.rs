pub const PREFIX: &str = ":";
pub const PREFIX_LEN: u32 = PREFIX.len() as u32;
pub const TOML_PATH: &str = "posts.toml";

lazy_static::lazy_static! {
    pub static ref HELP_TEXT: String = {
        format!(
            r#"help:

    main:
        [Post#content: ...String]
            => post with content.

    commands (current prefix: "{}"):
        check
            => check toml file integrity.

        init
            => initialize toml file.

        show [once_show: usize] [page_num: usize]
            => shows toml as friendly format.

        edit [Post#num: u32] [Post#content: ...String]
            => edit [number] post.

        remove [Post#num: u32]
            => remove [number] post.

        help
            => show this text.

        exit
            => exit program."#,
        PREFIX
    )};
}
