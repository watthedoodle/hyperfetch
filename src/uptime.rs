use crate::os::OS;
use crate::shell;
use std::path::Path;

pub fn get(os: OS) -> String {
    match os {
        OS::Linux | OS::Windows | OS::Minix => {
            if Path::new("/proc/uptime").exists() {
                let uptime = shell::clean(shell::slurp("/proc/uptime"));
                let tokens: Vec<&str> = uptime.split(" ").collect();
                if tokens.len() > 0 {
                    let tokens: Vec<&str> = tokens[0].split(".").collect();
                    if tokens.len() > 0 {
                        return self::to_pretty_print_time(&tokens[0]);
                    }
                }
            } else {
                return "???".to_string();
            }
        }
        OS::Darwin | OS::Bsd | OS::FreeMint => {
            return "not implemented for this OS".to_string();
        }
        OS::Solaris => {
            return "not implemented for this OS".to_string();
        }
        OS::Aix | OS::Irix => {
            return "not implemented for this OS".to_string();
        }
        OS::Haiku => {
            return "not implemented for this OS".to_string();
        }
        OS::Unknown => {
            return "not implemented for this OS".to_string();
        }
    }
    "???".to_string()
}

fn to_pretty_print_time(s: &str) -> String {
    if let Ok(seconds) = s.to_string().parse::<i32>() {
        let days = ((seconds / 60) / 60) / 24;
        let hours = (seconds / 60) / 60 % 24;
        let minutes = seconds / 60 % 60;
        let mut pretty = String::new();

        if days > 0 {
            if days > 2 {
                pretty.push_str(&format!("{} days,", days));
            } else {
                pretty.push_str(&format!("{} day,", days));
            }
        }
        if hours > 0 {
            if hours > 2 {
                pretty.push_str(&format!(" {} hours,", hours));
            } else {
                pretty.push_str(&format!(" {} hour,", hours));
            }
        }
        if minutes > 0 {
            if minutes > 2 {
                pretty.push_str(&format!(" {} minutes", minutes));
            } else {
                pretty.push_str(&format!(" {} minute", minutes));
            }
        }
        return pretty.trim().to_string();
    }
    s.to_string()
}
