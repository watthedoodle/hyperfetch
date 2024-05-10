use crate::os::OS;
use crate::shell;
use std::path::Path;

const DISTROS: &'static [&'static str] = &[
    "AIX",
    "Hash",
    "Alpine",
    "AlterLinux",
    "Amazon",
    "Anarchy",
    "Android",
    "instantOS",
    "Antergos",
    "antiX",
    "AOSC OS",
    "AOSC OS/Retro",
    "Apricity",
    "ArchCraft",
    "ArcoLinux",
    "ArchBox",
    "ARCHlabs",
    "ArchStrike",
    "XFerience",
    "ArchMerge",
    "Arch",
    "Artix",
    "Arya",
    "Bedrock",
    "Bitrig",
    "BlackArch",
    "BLAG",
    "BlankOn",
    "BlueLight",
    "Bodhi",
    "bonsai",
    "BSD",
    "BunsenLabs",
    "Calculat",
    "Carbs",
    "CentOS",
    "Chakra",
    "ChaletOS",
    "Chapeau",
    "Chrom*",
    "Cleanjaro",
    "ClearOS",
    "Clear_Linux",
    "Clover",
    "Condres",
    "Container_Linux",
    "Crystal Linux",
    "CRUX",
    "Cucumber",
    "dahlia",
    "Debian",
    "Deepin",
    "DesaOS",
    "Devuan",
    "DracOS",
    "DarkOs",
    "Itc",
    "DragonFly",
    "Drauger",
    "Elementary",
    "EndeavourOS",
    "Endless",
    "EuroLinux",
    "Exherbo",
    "Fedora",
    "Feren",
    "FreeBSD",
    "FreeMiNT",
    "Frugalware",
    "Funtoo",
    "GalliumOS",
    "Garuda",
    "Gentoo",
    "Pentoo",
    "gNewSense",
    "GNOME",
    "GNU",
    "GoboLinux",
    "Grombyang",
    "Guix",
    "Haiku",
    "Huayra",
    "HydroOS",
    "Hyperbola",
    "iglunix",
    "janus",
    "Kali",
    "KaOS",
    "KDE_neon",
    "Kibojoe",
    "Kogaion",
    "Korora",
    "KSLinux",
    "Kubuntu",
    "LEDE",
    "LaxerOS",
    "LibreELEC",
    "LFS",
    "Linux_Lite",
    "LMDE",
    "Lubuntu",
    "Lunar",
    "macos",
    "Mageia",
    "MagpieOS",
    "Mandriva",
    "Manjaro",
    "TeArch",
    "Maui",
    "Mer",
    "Minix",
    "LinuxMint",
    "Live_Raizo",
    "MX_Linux",
    "Namib",
    "Neptune",
    "NetBSD",
    "Netrunner",
    "Nitrux",
    "NixOS",
    "Nurunner",
    "NuTyX",
    "OBRevenge",
    "OpenBSD",
    "openEuler",
    "OpenIndiana",
    "openmamba",
    "OpenMandriva",
    "OpenStage",
    "OpenWrt",
    "osmc",
    "Oracle",
    "OS Elbrus",
    "PacBSD",
    "Parabola",
    "Pardus",
    "Parrot",
    "Parsix",
    "TrueOS",
    "PCLinuxOS",
    "Pengwin",
    "Peppermint",
    "Pisi",
    "popos",
    "Porteus",
    "PostMarketOS",
    "Proxmox",
    "PuffOS",
    "Puppy",
    "PureOS",
    "Qubes",
    "Qubyt",
    "Quibian",
    "Radix",
    "Raspbian",
    "Reborn_OS",
    "Redstar",
    "Redcore",
    "Redhat",
    "Refracted_Devuan",
    "Regata",
    "Regolith",
    "Rocky",
    "Rosa",
    "sabotage",
    "Sabayon",
    "Sailfish",
    "SalentOS",
    "Scientific",
    "Septor",
    "SereneLinux",
    "SharkLinux",
    "Siduction",
    "SkiffOS",
    "Slackware",
    "SliTaz",
    "SmartOS",
    "Solus",
    "Source_Mage",
    "Sparky",
    "Star",
    "SteamOS",
    "SunOS",
    "openSUSE_Leap",
    "t2",
    "openSUSE_Tumbleweed",
    "openSUSE",
    "SwagArch",
    "Tails",
    "Trisquel",
    "Ubuntu-Cinnamon",
    "Ubuntu-Budgie",
    "Ubuntu-GNOME",
    "Ubuntu-MATE",
    "Ubuntu-Studio",
    "Ubuntu",
    "Univention",
    "Venom",
    "Void",
    "VNux",
    "LangitKetujuh",
    "semc",
    "Obarun",
    "windows10",
    "Windows7",
    "Xubuntu",
    "Zorin",
    "IRIX",
];

pub fn get(os: OS) -> String {
    // TODO: we're migrating/tramslating the original bash function
    // it's just easier to have it right here to reference. We can
    // delete once we're finished with it.
    let migrate_script = r#"
get_distro() {
        case $os in
            Linux|BSD|MINIX)                
                elif type -p pveversion >/dev/null; then
                    case $distro_shorthand in
                        on|tiny) distro="Proxmox VE" ;;
                        *)
                            distro=$(pveversion)
                            distro=${distro#pve-manager/}
                            distro="Proxmox VE ${distro%/*}"
                    esac
    
                elif type -p lsb_release >/dev/null; then
                    case $distro_shorthand in
                        on)   lsb_flags=-si ;;
                        tiny) lsb_flags=-si ;;
                        *)    lsb_flags=-sd ;;
                    esac
                    distro=$(lsb_release "$lsb_flags")
        
                elif [[ -f /etc/GoboLinuxVersion ]]; then
                    case $distro_shorthand in
                        on|tiny) distro=GoboLinux ;;
                        *) distro="GoboLinux $(< /etc/GoboLinuxVersion)"
                    esac
    
                elif [[ -f /etc/SDE-VERSION ]]; then
                    distro="$(< /etc/SDE-VERSION)"
                    case $distro_shorthand in
                        on|tiny) distro="${distro% *}" ;;
                    esac
    
                elif type -p crux >/dev/null; then
                    distro=$(crux)
                    case $distro_shorthand in
                        on)   distro=${distro//version} ;;
                        tiny) distro=${distro//version*}
                    esac
    
                elif type -p tazpkg >/dev/null; then
                    distro="SliTaz $(< /etc/slitaz-release)"
    
                elif type -p kpt >/dev/null && \
                     type -p kpm >/dev/null; then
                    distro=KSLinux
    
                elif [[ -d /system/app/ && -d /system/priv-app ]]; then
                    distro="Android $(getprop ro.build.version.release)"
    
                # Chrome OS doesn't conform to the /etc/*-release standard.
                # While the file is a series of variables they can't be sourced
                # by the shell since the values aren't quoted.
                elif [[ -f /etc/lsb-release && $(< /etc/lsb-release) == *CHROMEOS* ]]; then
                    distro='Chrome OS'
    
                elif type -p guix >/dev/null; then
                    case $distro_shorthand in
                        on|tiny) distro="Guix System" ;;
                        *) distro="Guix System $(guix -V | awk 'NR==1{printf $4}')"
                    esac
    
                # Display whether using '-current' or '-release' on OpenBSD.
                elif [[ $kernel_name = OpenBSD ]] ; then
                    read -ra kernel_info <<< "$(sysctl -n kern.version)"
                    distro=${kernel_info[*]:0:2}
    
                else
                    for release_file in /etc/*-release; do
                        distro+=$(< "$release_file")
                    done
    
                    if [[ -z $distro ]]; then
                        case $distro_shorthand in
                            on|tiny) distro=$kernel_name ;;
                            *) distro="$kernel_name $kernel_version" ;;
                        esac
    
                        distro=${distro/DragonFly/DragonFlyBSD}
    
                        # Workarounds for some BSD based distros.
                        [[ -f /etc/pcbsd-lang ]]       && distro=PCBSD
                        [[ -f /etc/trueos-lang ]]      && distro=TrueOS
                        [[ -f /etc/pacbsd-release ]]   && distro=PacBSD
                        [[ -f /etc/hbsd-update.conf ]] && distro=HardenedBSD
                    fi
                fi
    
                if [[ $(< /proc/version) == *Microsoft* || $kernel_version == *Microsoft* ]]; then
                    windows_version=$(wmic.exe os get Version)
                    windows_version=$(trim "${windows_version/Version}")
    
                    case $distro_shorthand in
                        on)   distro+=" [Windows $windows_version]" ;;
                        tiny) distro="Windows ${windows_version::2}" ;;
                        *)    distro+=" on Windows $windows_version" ;;
                    esac
    
                elif [[ $(< /proc/version) == *chrome-bot* || -f /dev/cros_ec ]]; then
                    [[ $distro != *Chrome* ]] &&
                        case $distro_shorthand in
                            on)   distro+=" [Chrome OS]" ;;
                            tiny) distro="Chrome OS" ;;
                            *)    distro+=" on Chrome OS" ;;
                        esac
                        distro=${distro## on }
                fi
    
                # distro=$(trim_quotes "$distro")
                distro=${distro/NAME=}
    
                # Get Ubuntu flavor.
                if [[ $distro == "Ubuntu"* ]]; then
                    case $XDG_CONFIG_DIRS in
                        *"studio"*)   distro=${distro/Ubuntu/Ubuntu Studio} ;;
                        *"plasma"*)   distro=${distro/Ubuntu/Kubuntu} ;;
                        *"mate"*)     distro=${distro/Ubuntu/Ubuntu MATE} ;;
                        *"xubuntu"*)  distro=${distro/Ubuntu/Xubuntu} ;;
                        *"Lubuntu"*)  distro=${distro/Ubuntu/Lubuntu} ;;
                        *"budgie"*)   distro=${distro/Ubuntu/Ubuntu Budgie} ;;
                        *"cinnamon"*) distro=${distro/Ubuntu/Ubuntu Cinnamon} ;;
                    esac
                fi"#;

    let distro = match os {
        OS::Linux | OS::Minix | OS::Bsd => {
            let mut distro = String::new();
            if Path::new("/bedrock/etc/bedrock-release").exists() {
                distro = format!("Bedrock Linux");
            }
            if Path::new("/etc/redstar-release").exists() {
                distro = format!("Red Star OS");
            }
            if Path::new("/etc/armbian-release").exists() {
                // TODO source /etc/armbian and then do
                // Armbian $DISTRIBUTION_CODENAME (${VERSION:-})
                distro = format!("Armbian");
            }
            if Path::new("/etc/siduction-version").exists() {
                distro = format!("Siduction");
            }
            if Path::new("/etc/mcst_version").exists() {
                distro = format!("OS Elbrus");
            }
            let mut sourced = String::new();
            if Path::new("/etc/os-release").exists() {
                sourced.push_str(&shell::slurp("/etc/os-release"));
            }
            if Path::new("/usr/lib/os-release").exists() {
                sourced.push_str(&shell::slurp("/usr/lib/os-release"));
            }
            if Path::new("/etc/openwrt_release").exists() {
                sourced.push_str(&shell::slurp("/etc/openwrt_release"));
            }
            if Path::new("/etc/lsb-release").exists() {
                sourced.push_str(&shell::slurp("/etc/lsb-release"));
            }
            if sourced != "" {
                sourced.push_str("echo ${NAME:-${DISTRIB_ID}} ${VERSION_ID:-${DISTRIB_RELEASE}}");
                distro = shell::clean(shell::exec(&sourced));
            }
            distro
        }
        OS::Solaris => format!("Solaris"), // TODO include meta from /etc/release?
        OS::Haiku => format!("Haiku"),
        OS::FreeMint => format!("FreeMiNT"),
        OS::Irix => format!("IRIX"), // TODO: include "kernel_version"
        OS::Aix => format!("AIX"),   // TODO: include "oslevel"
        _ => format!("Unknown"),
    };
    distro
}
