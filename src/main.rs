mod jvm;
#[macro_use]
mod mac;
mod c_str;
mod dirs2;
mod ini;
mod io2;
mod zprofile;

use std::{error::Error, ffi::CStr, fs, io, path::Path};

use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use jvm::{read_jvms, JvmInfo, Metadata};
use mac::JAVA_VM_DIR;

#[derive(Debug, Parser)]
#[command(
    name = "javamanager",
    version = "1.0",
    about = "Manages Java Virtual Mechines"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// The `list` (alias `ls`) subcommand
    #[command(about = "Lists all installed Java VMs", alias = "ls")]
    List,

    /// The `remove` (alias `rm`) subcommand
    /// Usage
    /// ```
    /// javamanager remove /path/to/jvm1 /path/to/jvm2 /path/to/jvmN
    /// ```
    /// or offer index giving by `list` subcommand
    /// ```
    /// javamanager remove idx
    /// ```
    #[command(about = "Removes specified Java VMs", alias = "rm")]
    Remove {
        #[arg(help = "Removes JVM by path", required_unless_present = "idx")]
        path: Option<String>,

        #[arg(
            short, long,
            help = "Removes JVM by index",
            action = ArgAction::Set,
        )]
        index: Option<usize>,
    },

    /// The `install` subcommand
    /// Usage
    /// ```
    /// javamanager install /path/to/jvm1 /path/to/jvm2 /path/to/jvmN
    /// ```
    /// or offer index giving by `list` subcommand
    /// ```
    /// javamanager install idx
    /// ```
    #[command(about = "Installs specfied Java VMs to /Library/Java/JavaMechines")]
    Install {
        #[arg(help = "Install JVM by path", required_unless_present = "index")]
        path: Option<String>,

        #[arg(short, long, help = "Install JVM by Index giving by -l")]
        index: Option<usize>,
    },

    #[command(about = "Initialized")]
    Init,
}

fn _handle_io_err_path<R, P>(rs: io::Result<R>, path: P)
where
    P: AsRef<Path>,
{
    if let Err(e) = rs {
        let errno = e.raw_os_error().unwrap();
        warnx!("{}: {}", path.as_ref().to_str().unwrap(), unsafe {
            CStr::from_ptr(libc::strerror(errno)).to_str().unwrap()
        },);
    }
}

fn _handle_io_err<R>(rs: io::Result<R>) {
    if let Err(e) = rs {
        let errno = e.raw_os_error().unwrap();
        warnx!("{}", unsafe {
            CStr::from_ptr(libc::strerror(errno)).to_str().unwrap()
        },);
    }
}

#[cfg(target_os = "macos")]
fn main() {
    let cli = Cli::parse();
    _handle_io_err(_main(cli.command));
}

#[cfg(target_os = "macos")]
fn _main(cmd: Commands) -> io::Result<()> {
    use std::{
        fs::{self, File},
        io::Write,
    };

    use javamanager::stdin;
    use mac::JAVA_VM_DIR;
    use zprofile::Profile;

    match cmd {
        Commands::List => {
            print_jvms(jvm::read_jvms()?)?;
        }
        Commands::Remove { index, path } => match (index, path) {
            (Some(idx), None) => {
                let info = JvmInfo::new()?;
                let which = info.remove_at(idx)?;
                println!("JVM removed");
                print_jvms(vec![Metadata::new(which)?])?;
            }
            (None, Some(_)) => {}
            _ => {}
        },
        Commands::Install { index, path } => {
            match (index, path) {
                (Some(idx), None) => {
                    let info = JvmInfo::new()?;
                    let which = info.load_at(idx)?;
                    println!("Changing JVM");
                    print_jvms(vec![Metadata::new(which)?])?;
                }
                (None, Some(path)) => {
                    // check if the directory is MacOS X Java Structure
                    fn _inner(path: String) -> io::Result<()> {
                        let md = Metadata::new(&path)?;
                        mac::rmxattr(&path)?;
                        _install(&path)?;
                        println!("JVM installed");
                        print_jvms(vec![md])?;
                        Ok(())
                    }
                    _handle_io_err_path(_inner(path.clone()), path);
                }
                _ => {}
            }
        }

        // TODO: not complete yet
        Commands::Init => {
            // Current存不存在，不存在就创建（如果有JAVA_HOME，根据创建，没有就叫用户选一个创建），存在就抛出异常
            match jvm::current_metadata() {
                // 不存在 "Current"
                Err(e) => {
                    if e.kind() != io::ErrorKind::NotFound {
                        let errno = e.raw_os_error().unwrap();
                        warnx!("{}", unsafe {
                            CStr::from_ptr(libc::strerror(errno)).to_str().unwrap()
                        });
                        return Ok(());
                    }
                    // 一下是 NotFound 的情况
                    // 如果设置了 $JAVA_HOME，就把源 $JAVA_HOME 设置为 Current
                    // 没设置就在 OS respo 里面找
                    if let Some(java_home) = dirs2::java_home() {
                        let current = dirs2::jvm_respo().unwrap().join("Current");
                        if java_home != current {
                            eprintln!("$JAVA_HOME detected but incorrect! Please modify $JAVA_HOME to {} manually", current.to_str().unwrap());
                            return Ok(());
                        }
                    } else {
                        // 没有java_home，创建 Current, 然后叫用户选择一个，没有就叫用户安装
                        // {
                        //     let mut zprofile = Profile::open()?;
                        //     zprofile.write_var(
                        //         "JAVA_HOME",
                        //         dirs2::jvm_respo().unwrap().join("Current").to_str().unwrap(),
                        //     )?;
                        // }
                        println!("Writing $JAVA_HOME");

                        let jvms = JvmInfo::new()?;
                        if jvms.is_empty() {
                            eprintln!(
                                "No JVMs found in {}, please run `sudo javamanager install /path/to/jvm`",
                                dirs2::jvm_respo().unwrap().to_str().unwrap(),
                            );
                        } else {
                            print_jvms(jvm::read_jvms()?)?;
                            let len = jvms.len();

                            let n = stdin!(
                                usize,
                                "Which JVM to install? ({}) ",
                                (0..len)
                                    .map(|n| n.to_string())
                                    .collect::<Vec<String>>()
                                    .join("/")
                            );
                            return _main(Commands::Install {
                                path: None,
                                index: Some(n),
                            });
                        }
                    }
                }
                // 存在 "Current"
                Ok(_) => {
                    println!("Reinitialized existing");
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() -> ! {
    panic!("Sorry, this command line tool is designed for Mac");
}

#[inline]
fn _print_vm_info(md: Metadata) {
    // println!("{:<10}{:<15}{:<15}", "版本", "虚拟机", "提供商");
    // println!("{:<10}{:<15}{:<15}", md.version(), md.variant(), md.vendor());
    println!("{:>6}  {:<7}  {:<18}", "VER", "VM", "VENDOR");
    println!(
        "{:>6}  {:<7}  {:<18}",
        md.version(),
        md.variant(),
        md.vendor()
    );
}

fn _install(path: &String) -> io::Result<()> {
    mac::mv_all(path, "/Library/Java/JavaVirtualMachines")
}

fn print_jvms(infos: Vec<Metadata>) -> io::Result<()> {
    #[inline]
    fn row(info: &Metadata, is_current: bool, i: usize) -> String {
        format!(
            "{:>3} {:>6}  {:<7}  {:<18}   {}",
            format!("{}{}", i, if is_current { "*" } else { " " }),
            info.version(),
            info.variant(),
            info.vendor(),
            format!(
                "{}{}",
                info.path().split("/").nth(4).unwrap(),
                if is_current { " (Current)" } else { "" },
            ),
        )
    }
    println!(
        "{:>2}  {:>6}  {:<7}  {:<18}   {}",
        "#", "VER", "VM", "VENDOR", "NAME"
    );

    for (i, info) in infos.iter().enumerate() {
        let is_current = info.is_current()?;

        if info.file_name() == "Current" {
            continue;
        }
        if is_current {
            println!("{}", row(info, is_current, i).yellow().bold());
        } else {
            println!("{}", row(info, is_current, i));
        }
    }
    Ok(())
}

#[test]
fn tests() {
    warnx!("{}", "dsd");
}
