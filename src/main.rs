use app::App;
use clap::{arg, command, value_parser, ArgAction, Command, Arg};
use std::{path::PathBuf, io};
mod app;
mod file;

static TYPE_HELP: &'static str = "x, max, maximum   20 characters, contains symbols.{n}\
p, phrase         20 character sentence.";

fn main() -> io::Result<()> {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .help(TYPE_HELP)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(Command::new("init").about("初始化本地存储文件"))
        .subcommand(Command::new("list").about("显示所有Todos"))
        .subcommand(
            Command::new("add")
                .about("添加一个Todo")
                .arg(
                    Arg::new("content")
                        .action(ArgAction::Set)
                        .help("输入您要添加的Todo内容")
                )
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let mut app = App::new();
    app.run(&matches)?;

    Ok(())
}
