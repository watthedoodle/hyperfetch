/* -------------------------------------------------------------------------------------------------
original code adapted from https://github.com/rilysh/uname/blob/main/src/nix.rs
--------------------------------------------------------------------------------------------------*/
#[derive(Default, Debug)]
pub struct Uname {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub domainname: String,
}

pub fn uname() -> Uname {
    let mut os: Uname = Default::default();

    let mut uts = libc::utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };

    unsafe {
        if libc::uname(&mut uts) == -1 {
            Uname {
                sysname: "".to_string(),
                nodename: "".to_string(),
                release: "".to_string(),
                version: "".to_string(),
                machine: "".to_string(),
                domainname: "".to_string(),
            };
        }
    }

    uts.sysname.iter().for_each(|each| {
        if *each != 0 {
            os.sysname.push(*each as u8 as char);
        }
    });

    uts.nodename.iter().for_each(|each| {
        if *each != 0 {
            os.nodename.push(*each as u8 as char);
        }
    });

    uts.release.iter().for_each(|each| {
        if *each != 0 {
            os.release.push(*each as u8 as char);
        }
    });

    uts.version.iter().for_each(|each| {
        if *each != 0 {
            os.version.push(*each as u8 as char);
        }
    });

    uts.machine.iter().for_each(|each| {
        if *each != 0 {
            os.machine.push(*each as u8 as char);
        }
    });

    #[cfg(target_os = "linux")]
    // Hidden under _GNU_SOURCE
    uts.domainname.iter().for_each(|each| {
        os.domainname.push(*each as u8 as char);
    });

    Uname {
        sysname: os.sysname,
        nodename: os.nodename,
        release: os.release,
        version: os.version,
        machine: os.machine,
        domainname: os.domainname,
    }
}
