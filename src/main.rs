mod emulator;
mod tests;
use clap::Parser;
use emulator::cpu::Cpu;

///  A RISC-V emulator, specifically the RV32I base integer instruction set.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to be executed
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    file: std::path::PathBuf,

    /// Print instructions as they are executed
    #[clap(short, long)]
    debug: bool,

    /// Show register values after each instruction
    #[clap(short, long)]
    registers: bool,

    /// Show register ABI names or numeric values (x0-x31)
    /// Use with the `--registers` option.
    #[clap(short, long)]
    aliases: bool,

    /// Interactive mode. Use with either `--registers` and/or `--debug`
    #[clap(short, long)]
    interactive: bool,

    /// Custom program counter start address in hex
    #[clap(short, long)]
    pc: Option<usize>,
}
fn main() {
    let args = Args::parse();

    let mut cpu = Cpu::new();
    cpu.load(
        args.file
            .into_os_string()
            .to_str()
            .expect("not valid unicode"),
    );

    cpu.run(
        true,
        args.debug,
        args.registers,
        args.pc,
        args.aliases,
        args.interactive,
    );
}
