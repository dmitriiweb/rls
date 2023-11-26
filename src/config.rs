#[derive(Debug)]
pub enum SortBy {
    Name,
    Size,
    Time,
}
#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub colors: bool,
    pub dir_first: bool,
    pub sort_by: SortBy,
    pub all: bool,
}

impl Config {
    pub fn new() -> Self {
        let matches = clap::command!()
            .about("Rust implementation of ls")
            .arg(
                clap::Arg::new("path")
                    .default_value(".")
                    .help("The path to a dir"),
            )
            .arg(
                clap::Arg::new("all")
                    .long("all")
                    .short('a')
                    .num_args(0)
                    .help("Show hidden files and folders"),
            )
            .arg(
                clap::Arg::new("colors")
                    .long("no-colors")
                    .aliases(["no-color", "nc", "nocolors", "ncolors"])
                    .num_args(0)
                    .help("Disable colored output"),
            )
            .arg(
                clap::Arg::new("dir-first")
                    .long("dir-first")
                    .aliases(["dirfirst", "df"])
                    .num_args(0)
                    .help("Show directories first"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .aliases(["sortby", "sb"])
                    .default_value("name")
                    .help("Sort by name, size, or time")
                    .value_parser(["name", "size", "time"]),
            )
            .get_matches();
        let path = matches.get_one::<String>("path").unwrap().to_string();
        let colors = matches.get_one::<bool>("colors").unwrap_or(&true);
        let dir_first = matches.get_one::<bool>("dir-first").unwrap_or(&false);
        let sort_by = match matches.get_one::<String>("sort-by").unwrap().as_str() {
            "name" => SortBy::Name,
            "size" => SortBy::Size,
            "time" => SortBy::Time,
            _ => panic!("Invalid sort-by value"),
        };
        let all = matches.get_one::<bool>("all").unwrap_or(&false);
        Config {
            path,
            colors: *colors,
            dir_first: *dir_first,
            sort_by,
            all: *all,
        }
    }
}
