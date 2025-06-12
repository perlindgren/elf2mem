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
    /// Path for output
    #[arg(short, long)]
    out_dir: Option<PathBuf>,

    /// Input file in elf format
    #[arg(short, long, default_value = "app.elf")]
    file: PathBuf,

    /// Width in bytes per package
    #[arg(short, long, default_value_t = 4)]
    width: u8,

    /// Packed [default: non-packed (spaces)]
    #[arg(short, long)]
    packed: bool,

    /// Native byte order [default: flipped byte order]
    #[arg(short, long)]
    native: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut out_path_text: PathBuf = if let Some(out) = cli.out_dir.as_deref() {
        out.to_owned()
    } else {
        PathBuf::new()
    };
    out_path_text.push(out_path_text.with_file_name("text.mem"));

    let mut ro_path: Vec<PathBuf> = vec![];
    let mut ro_f = vec![];
    for i in 0..4 {
        let mut p = if let Some(out_dir) = cli.out_dir.clone() {
            out_dir
        } else {
            PathBuf::new()
        };
        p.push(p.with_file_name(format!("data_{}.mem", i)));
        ro_path.push(p.clone());
        ro_f.push(fs::File::create(p)?);
    }
    println!("Generate output:");
    println!("In elf file       : {:?}", cli.file);
    println!("Out-dir           : {:?}", cli.out_dir.unwrap_or_default());
    println!("Width             : {:?}", cli.width);
    println!("Packed            : {:?}", cli.packed);
    println!("Native byte order : {:?}\n", cli.native);

    let in_path = cli.file;
    let file_data = fs::read(in_path.clone())?;
    let mut f_out_text = fs::File::create(out_path_text.clone())?;

    let data = file_data.as_slice();
    let elf = ElfFile::new(data)?;

    let text_section = elf.find_section_by_name(".text").unwrap();
    dump_section(
        &elf,
        text_section,
        data,
        cli.width,
        cli.packed,
        &mut f_out_text,
        !cli.native,
    )?;

    if let Some(data_section) = elf.find_section_by_name(".rodata") {
        dump_ro_data(&elf, data_section, data, &mut ro_f)?;
    }

    Ok(())
}

fn dump_ro_data(
    elf: &ElfFile,
    sh: SectionHeader,
    data: &[u8],
    ro_f: &mut [fs::File],
) -> Result<(), Box<dyn Error>> {
    println!(
        "section {:?}, \taddress {:#10x}, \tsize {:#10x}",
        sh.get_name(elf)?,
        sh.address(),
        sh.size()
    );
    for (i, mut ro) in ro_f.iter().enumerate() {
        writeln!(ro, "// section {:?} [{}]", sh.get_name(elf)?, i)?;
        writeln!(ro, "@{:x?}", sh.address() - 0x5000_0000)?;
    }

    let d = &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize];

    for (i, byte) in d.iter().enumerate() {
        write!(ro_f[i % 4], "{:x} ", byte)?;
    }

    Ok(())
}

fn dump_section(
    elf: &ElfFile,
    sh: SectionHeader,
    data: &[u8],
    width: u8,
    spaced: bool,
    f_out: &mut File,
    flip: bool,
) -> Result<(), Box<dyn Error>> {
    println!(
        "section {:?}, \taddress {:#10x}, \tsize {:#10x}",
        sh.get_name(elf)?,
        sh.address(),
        sh.size()
    );

    writeln!(f_out, "// section {:?}", sh.get_name(elf)?)?;
    writeln!(f_out, "@{:x?}", sh.address() / width as u64)?;
    let mut v = vec![];
    let slice = if flip {
        let d = &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize];
        for chunk in d.chunks(4) {
            let mut c = chunk.to_owned();
            c.reverse();
            for b in c {
                v.push(b);
            }
        }
        v.as_slice()
    } else {
        &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize]
    };

    for (i, d) in slice.iter().enumerate() {
        write!(f_out, "{:02x?}{}", d, if spaced { " " } else { "" })?;
        if (i + 1) % width as usize == 0 {
            writeln!(f_out)?;
        }
    }
    writeln!(f_out)?;
    Ok(())
}
