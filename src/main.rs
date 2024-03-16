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
    /// Optional name for generated .data file [default: <input file>_data.mem)
    #[arg(short = 'd', long)]
    out_data: Option<PathBuf>,

    // Optional name for generated .text file [default: <input file>_text.mem]
    #[arg(short = 't', long)]
    out_text: Option<PathBuf>,

    /// Input file in elf format
    #[arg(short, long, default_value = "app.elf")]
    file: PathBuf,

    /// Width in bytes per package
    #[arg(short, long, default_value_t = 4)]
    width: u8,

    /// Packed (no spaces between bytes)
    #[arg(short, long, default_value_t = false)]
    packed: bool,

    /// Flip the byte order of the loaded words
    #[arg(short, long, default_value_t = true)]
    endianness_flip: bool,
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let out_path_data: PathBuf = if let Some(out) = cli.out_data.as_deref() {
        println!("Value for name: {:?}", out);
        out.to_owned()
    } else {
        let mut p: PathBuf = cli.file.clone().with_extension("");
        let name = p.file_name().unwrap().to_str().unwrap();
        let name = format!("{}_data", name);
        p.set_file_name(name);
        p.set_extension("mem");
        p
    };
    let out_path_text: PathBuf = if let Some(out) = cli.out_text.as_deref() {
        println!("Value for name: {:?}", out);
        out.to_owned()
    } else {
        let mut p: PathBuf = cli.file.clone().with_extension("");
        let name = p.file_name().unwrap().to_str().unwrap();
        let name = format!("{}_text", name);
        p.set_file_name(name);
        p.set_extension("mem");
        p
    };

    println!("Generate output:");
    println!("In elf file     : {:?}", cli.file);
    println!("Out data file   : {:?}", out_path_data);
    println!("Out text file   : {:?}", out_path_text);
    println!("Width           : {:?}", cli.width);
    println!("Packed          : {:?}", cli.packed);
    println!("Endianness flip : {:?}\n", cli.endianness_flip);
    
    let in_path = cli.file;
    let file_data = fs::read(in_path.clone())?;
    let mut f_out_text = fs::File::create(out_path_text.clone())?;
    let mut f_out_data = fs::File::create(out_path_data.clone())?;

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
        cli.endianness_flip,
    )?;
    let data_section = elf.find_section_by_name(".data").unwrap();
    dump_section(
        &elf,
        data_section,
        data,
        cli.width,
        cli.packed,
        &mut f_out_data,
        cli.endianness_flip,
    )?;

    Ok(())
}

fn dump_section(
    elf: &ElfFile,
    sh: SectionHeader,
    data: &[u8],
    width: u8,
    packed: bool,
    f_out: &mut File,
    flip: bool,
) -> Result<(), Box<dyn Error>> {
    println!(
        "section {:?}, address {:#10x}, size {:#10x}",
        sh.get_name(elf)?,
        sh.address(),
        sh.size()
    );

    writeln!(f_out, "// section {:?}", sh.get_name(elf)?)?;
    writeln!(f_out, "@{:x?}", sh.address() / width as u64)?;
    let mut v = vec![];
    let slice = if flip {
        let d = &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize];
        for chunk in d.chunks(4).into_iter() {
            let mut c = chunk.to_owned();
            c.reverse();
            for b in c {
                v.push(b);
            }
        }
        v.as_slice()
    } else {
        let d = &data[sh.offset() as usize..(sh.offset() + sh.size()) as usize];
        d
    };

    for (i, d) in slice.into_iter().enumerate() {
        write!(f_out, "{:02x?}{}", d, if packed { "" } else { " " })?;
        if (i + 1) % width as usize == 0 {
            writeln!(f_out)?;
        }
    }
    writeln!(f_out)?;
    Ok(())
}
