use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use xmas_elf::{sections::SectionHeader, ElfFile};

use clap::Parser;

#[derive(Parser)]
#[command(
    about = "Extract .text and .data elf sections to Verilog .mem file",
    author = "Per Lindgren (per.lindgren@ltu.se)"
)]

struct Cli {
    /// Optional name for generated mem file [default: <input file>.mem)
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Input file in elf format
    #[arg(short, long, default_value = "app.elf")]
    file: PathBuf,

    /// Width in bytes per package
    #[arg(short, long, default_value_t = 4)]
    width: u8,

    /// Packed (no spaces between bytes)
    #[arg(short, long)]
    packed: bool,
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let out_path: PathBuf = if let Some(out) = cli.out.as_deref() {
        println!("Value for name: {:?}", out);
        out.to_owned()
    } else {
        let mut p: PathBuf = cli.file.clone();
        p.set_extension("mem");
        p
    };

    let in_path = cli.file;

    let file_data = fs::read(in_path.clone())?;
    let mut f_out = fs::File::create(out_path.clone())?;

    let data = file_data.as_slice();
    let elf = ElfFile::new(data)?;

    println!("elf2mem -f {:?} -o {:?}\n", in_path, out_path);
    let text_section = elf.find_section_by_name(".text").unwrap();
    dump_section(&elf, text_section, data, cli.width, cli.packed, &mut f_out)?;
    let data_section = elf.find_section_by_name(".data").unwrap();
    dump_section(&elf, data_section, data, cli.width, cli.packed, &mut f_out)?;

    Ok(())
}

fn dump_section(
    elf: &ElfFile,
    sh: SectionHeader,
    data: &[u8],
    width: u8,
    packed: bool,
    f_out: &mut File,
) -> Result<(), Box<dyn Error>> {
    println!(
        "section {:?}, address {:#10x}, size {:#10x}",
        sh.get_name(elf)?,
        sh.address(),
        sh.size()
    );

    writeln!(f_out, "// section {:?}", sh.get_name(elf)?)?;
    writeln!(f_out, "@ {:x?}", sh.address())?;
    let slice = &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize];
    for (i, d) in slice.iter().enumerate() {
        write!(f_out, "{:02x?}{}", d, if packed { "" } else { " " })?;
        if (i + 1) % width as usize == 0 {
            writeln!(f_out)?;
        }
    }
    writeln!(f_out)?;
    Ok(())
}
