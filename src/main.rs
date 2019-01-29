use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Side-by-side diff for two files
#[derive(StructOpt, Debug)]
#[structopt(name = "prettydiff")]
struct Opt {
    /// Left file
    #[structopt(name = "LEFT", parse(from_os_str))]
    left: PathBuf,
    /// Right file
    #[structopt(name = "RIGHT", parse(from_os_str))]
    right: PathBuf,
}

fn read_file(path: &PathBuf) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let left_data = read_file(&opt.left)?;
    let right_data = read_file(&opt.right)?;

    let dopt = prettydiff::DiffOpt {
        left: left_data,
        right: right_data,

        left_name: Some(opt.left.into_os_string().into_string().unwrap()),
        right_name: Some(opt.right.into_os_string().into_string().unwrap()),
        diff_only: false,
    };

    prettydiff::diff_text(dopt);

    Ok(())
}
