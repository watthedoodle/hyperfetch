use crate::kernel;

pub enum OS {
    Darwin,
    Solaris,
    Haiku,
    Minix,
    Aix,
    Irix,
    FreeMint,
    Linux,
    Bsd,
    Windows,
    Unknown,
}

pub fn detect() -> OS {
    let kernel = kernel::uname();
    let machine = kernel.sysname;

    match machine.as_str() {
        "Darwin" => OS::Darwin,
        "SunOS" => OS::Solaris,
        "Haiku" => OS::Haiku,
        "MINIX" => OS::Minix,
        "AIX" => OS::Aix,
        machine if machine.contains("IRIX") => OS::Irix,
        "Linux" => OS::Linux,
        machine if machine.contains("GNU") => OS::Linux,
        "FreeMiNT" => OS::FreeMint,
        "BSD" | "Dragon" | "Bitrig" => OS::Bsd,
        machine if machine.contains("CYGWIN") => OS::Windows,
        machine if machine.contains("MSYS") => OS::Windows,
        machine if machine.contains("MINGW") => OS::Windows,
        _ => OS::Unknown,
    }
}
