use crate::kernel;
use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OS::Darwin => write!(f, "Mac OS X"),
            OS::Solaris => write!(f, "Solaris"),
            OS::Haiku => write!(f, "Haiku"),
            OS::Minix => write!(f, "MINIX"),
            OS::Aix => write!(f, "AIX"),
            OS::Irix => write!(f, "IRIX"),
            OS::FreeMint => write!(f, "FreeMiNT"),
            OS::Linux => write!(f, "Linux"),
            OS::Bsd => write!(f, "BSD"),
            OS::Windows => write!(f, "Windows"),
            OS::Unknown => write!(f, "Unknown"),
        }
    }
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
