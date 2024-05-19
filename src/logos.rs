pub struct Logo {
    pub ascii: String,
    pub width: u16,
    pub height: u16,
}

const DISTROS: &'static [(&'static str, fn() -> Logo)] = &[
    ("Ubuntu", self::ubuntu),
    ("Garuda", self::garuda),
    ("Arch", self::arch),
    ("FreeBSD", self::free_bsd),
    ("Ubuntu Studio", self::ubuntu_studio),
    ("Kubuntu", self::kubuntu),
    ("Ubuntu MATE", self::ubuntu_mate),
    ("Xubuntu", self::xubuntu),
    ("Lubuntu", self::lubuntu),
    ("Ubuntu Budgie", self::ubuntu_budgie),
    ("Ubuntu Cinnamon", self::ubuntu_cinnamon),
];

pub fn detect(distro: &str) -> Logo {
    for (k, v) in DISTROS {
        if distro.contains(k) {
            return v();
        }
    }

    // use an unknown ascii art here
    Logo {
        ascii: "UNKNOWN".to_string(),
        width: 7,
        height: 1,
    }
}

pub fn free_bsd() -> Logo {
    let x = r"
    ${c2}```                        ${c1}`
    ${c2}` `.....---...${c1}....--.```   -/
    ${c2}+o   .--`         ${c1}/y:`      +.
    ${c2} yo`:.            ${c1}:o      `+-
      ${c2}y/               ${c1}-/`   -o/
     ${c2}.-                  ${c1}::/sy+:.
     ${c2}/                     ${c1}`--  /
    ${c2}`:                          ${c1}:`
    ${c2}`:                          ${c1}:`
     ${c2}/                          ${c1}/
     ${c2}.-                        ${c1}-.
      ${c2}--                      ${c1}-.
       ${c2}`:`                  ${c1}`:`
         .--             `--.
            .---.....----.
    ";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn ubuntu() -> Logo {
    let x = r"
${c1}            .-/+oossssoo+\-.
        ´:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
   /sssssssssss${c2}hdmmNNmmyNMMMMh${c1}ssssss\
  +sssssssss${c2}hm${c1}yd${c2}MMMMMMMNddddy${c1}ssssssss+
 /ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhmNMMMNh${c1}ssssssss\
.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
oss${c2}yNMMMNyMMh${c1}ssssssssssssss${c2}hmmmh${c1}ssssssso
oss${c2}yNMMMNyMMh${c1}sssssssssssssshmmmh${c1}ssssssso
+ssss${c2}hhhyNMMNy${c1}ssssssssssss${c2}yNMMMy${c1}sssssss+
.ssssssss${c2}dMMMNh${c1}ssssssssss${c2}hNMMMd${c1}ssssssss.
 \ssssssss${c2}hNMMM${c1}yh${c2}hyyyyhdNMMMNh${c1}ssssssss/
  +sssssssss${c2}dm${c1}yd${c2}MMMMMMMMddddy${c1}ssssssss+
   \sssssssssss${c2}hdmNNNNmyNMMMMh${c1}ssssss/
    .ossssssssssssssssss${c2}dMMMNy${c1}sssso.
      -+sssssssssssssssss${c2}yyy${c1}ssss+-
        `:+ssssssssssssssssss+:`
            .-\+oossssoo+/-.
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn ubuntu_studio() -> Logo {
    let x = r"
${c1}              ..-::::::-.`
         `.:+++++++++++${c2}ooo${c1}++:.`
       ./+++++++++++++${c2}sMMMNdyo${c1}+/.
     .++++++++++++++++${c2}oyhmMMMMms${c1}++.
   `/+++++++++${c2}osyhddddhys${c1}+${c2}osdMMMh${c1}++/`
  `+++++++++${c2}ydMMMMNNNMMMMNds${c1}+${c2}oyyo${c1}++++`
  +++++++++${c2}dMMNhso${c1}++++${c2}oydNMMmo${c1}++++++++`
 :+${c2}odmy${c1}+++${c2}ooysoohmNMMNmyoohMMNs${c1}+++++++:
 ++${c2}dMMm${c1}+${c2}oNMd${c1}++${c2}yMMMmhhmMMNs+yMMNo${c1}+++++++
`++${c2}NMMy${c1}+${c2}hMMd${c1}+${c2}oMMMs${c1}++++${c2}sMMN${c1}++${c2}NMMs${c1}+++++++.
`++${c2}NMMy${c1}+${c2}hMMd${c1}+${c2}oMMMo${c1}++++${c2}sMMN${c1}++${c2}mMMs${c1}+++++++.
 ++${c2}dMMd${c1}+${c2}oNMm${c1}++${c2}yMMNdhhdMMMs${c1}+y${c2}MMNo${c1}+++++++
 :+${c2}odmy${c1}++${c2}oo${c1}+${c2}ss${c1}+${c2}ohNMMMMmho${c1}+${c2}yMMMs${c1}+++++++:
  +++++++++${c2}hMMmhs+ooo+oshNMMms${c1}++++++++
  `++++++++${c2}oymMMMMNmmNMMMMmy+oys${c1}+++++`
   `/+++++++++${c2}oyhdmmmmdhso+sdMMMs${c1}++/
     ./+++++++++++++++${c2}oyhdNMMMms${c1}++.
       ./+++++++++++++${c2}hMMMNdyo${c1}+/.
         `.:+++++++++++${c2}sso${c1}++:.
              ..-::::::-..
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn kubuntu() -> Logo {
    let x = r"
${c1}           `.:/ossyyyysso/:.
        .:oyyyyyyyyyyyyyyyyyyo:`
      -oyyyyyyyo${c2}dMMy${c1}yyyyyyysyyyyo-
    -syyyyyyyyyy${c2}dMMy${c1}oyyyy${c2}dmMMy${c1}yyyys-
   oyyys${c2}dMy${c1}syyyy${c2}dMMMMMMMMMMMMMy${c1}yyyyyyo
 `oyyyy${c2}dMMMMy${c1}syysoooooo${c2}dMMMMy${c1}yyyyyyyyo`
 oyyyyyy${c2}dMMMMy${c1}yyyyyyyyyyys${c2}dMMy${c1}sssssyyyo
-yyyyyyyy${c2}dMy${c1}syyyyyyyyyyyyyys${c2}dMMMMMy${c1}syyy-
oyyyysoo${c2}dMy${c1}yyyyyyyyyyyyyyyyyy${c2}dMMMMy${c1}syyyo
yyys${c2}dMMMMMy${c1}yyyyyyyyyyyyyyyyyysosyyyyyyyy
yyys${c2}dMMMMMy${c1}yyyyyyyyyyyyyyyyyyyyyyyyyyyyy
oyyyyysos${c2}dy${c1}yyyyyyyyyyyyyyyyyy${c2}dMMMMy${c1}syyyo
-yyyyyyyy${c2}dMy${c1}syyyyyyyyyyyyyys${c2}dMMMMMy${c1}syyy-
 oyyyyyy${c2}dMMMy${c1}syyyyyyyyyyys${c2}dMMy${c1}oyyyoyyyo
 `oyyyy${c2}dMMMy${c1}syyyoooooo${c2}dMMMMy${c1}oyyyyyyyyo
   oyyysyyoyyyys${c2}dMMMMMMMMMMMy${c1}yyyyyyyo
    -syyyyyyyyy${c2}dMMMy${c1}syyy${c2}dMMMy${c1}syyyys-
      -oyyyyyyy${c2}dMMy${c1}yyyyyysosyyyyo-
        ./oyyyyyyyyyyyyyyyyyyo/.
           `.:/oosyyyysso/:.`
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn ubuntu_mate() -> Logo {
    let x = r"
${c1}            .:/+oossssoo+/:.`
        `:+ssssssssssssssssss+:`
      -+sssssssssssssss${c2}y${c1}ssssssss+-
    .osssssssssssss${c2}yy${c1}ss${c2}mMmh${c1}ssssssso.
   /sssssssss${c2}ydmNNNmmd${c1}s${c2}mMMMMNdy${c1}sssss/
 `+ssssssss${c2}hNNdy${c1}sssssss${c2}mMMMMNdy${c1}ssssss+`
 +sssssss${c2}yNNh${c1}ss${c2}hmNNNNm${c1}s${c2}mMmh${c1}s${c2}ydy${c1}sssssss+
-sssss${c2}y${c1}ss${c2}Nm${c1}ss${c2}hNNh${c1}ssssss${c2}y${c1}s${c2}hh${c1}ss${c2}mMy${c1}sssssss-
+ssss${c2}yMNdy${c1}ss${c2}hMd${c1}ssssssssss${c2}hMd${c1}ss${c2}NN${c1}sssssss+
sssss${c2}yMMMMMmh${c1}sssssssssssss${c2}NM${c1}ss${c2}dMy${c1}sssssss
sssss${c2}yMMMMMmhy${c1}ssssssssssss${c2}NM${c1}ss${c2}dMy${c1}sssssss
+ssss${c2}yMNdy${c1}ss${c2}hMd${c1}ssssssssss${c2}hMd${c1}ss${c2}NN${c1}sssssss+
-sssss${c2}y${c1}ss${c2}Nm${c1}ss${c2}hNNh${c1}ssssssss${c2}dh${c1}ss${c2}mMy${c1}sssssss-
 +sssssss${c2}yNNh${c1}ss${c2}hmNNNNm${c1}s${c2}mNmh${c1}s${c2}ymy${c1}sssssss+
  +ssssssss${c2}hNNdy${c1}sssssss${c2}mMMMMmhy${c1}ssssss+
   /sssssssss${c2}ydmNNNNmd${c1}s${c2}mMMMMNdh${c1}sssss/
    .osssssssssssss${c2}yy${c1}ss${c2}mMmdy${c1}sssssso.
      -+sssssssssssssss${c2}y${c1}ssssssss+-
        `:+ssssssssssssssssss+:`
            .:/+oossssoo+/:.
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn xubuntu() -> Logo {
    let x = r"
${c1}           `.:/ossyyyysso/:.
        `.yyyyyyyyyyyyyyyyyyyy.`
      `yyyyyyyyyyyyyyyyyyyyyyyyyy`
    `yyyyyyyyyyyyyyyyyyyy${c2}::${c1}yyyyyyyy`
   .yyyyyyyyyyy${c2}/+:${c1}yyyyyyy${c2}ds${c1}yyy${c2}+y${c1}yyyy.
  yyyyyyy${c2}:o/${c1}yy${c2}dMMM+${c1}yyyyy${c2}/M+${c1}y${c2}:hM+${c1}yyyyyy
 yyyyyyy${c2}+MMMy${c1}y${c2}mMMMh${c1}yyyyy${c2}yM::mM+${c1}yyyyyyyy
`yyyyyyy${c2}+MMMMysMMMd${c1}yyyyy${c2}dh:mN+${c1}yyyyyyyyy`
yyyyyyyy${c2}:NMMMMmMMMMmmdhyy+/y:${c1}yyyyyyyyyyy
yyyyyyyy${c2}+MMMMMMMMMMMMMMMMMMNho:${c1}yyyyyyyyy
yyyyyyyy${c2}mMMMMMMMMMMMMMMMMMMMMMMy${c1}yyyyyyyy
yyyyyyy${c2}+MMMMMMMMMMMMMMMMMMMMMMMM/${c1}yyyyyyy
`yyyyyy${c2}sMMMMMMMMMMMMMMMMMMMMMMmo${c1}yyyyyyy`
 yyyyyy${c2}oMMMMMMMMMMMMMMMMMMMmy+${c1}yyyyyyyyy
  yyyyy${c2}:mMMMMMMMMMMMMMMNho/${c1}yyyyyyyyyyy
   .yyyy${c2}:yNMMMMMMMNdyo:${c1}yyyyyyyyyyyyy.
    `yyyyyy${c2}:/++/::${c1}yyyyyyyyyyyyyyyyy`
      `yyyyyyyyyyyyyyyyyyyyyyyyyy`
        `.yyyyyyyyyyyyyyyyyyyy.`
           `.:/oosyyyysso/:.`
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn lubuntu() -> Logo {
    let x = r"
${c1}           `.:/ossyyyysso/:.
        `.:yyyyyyyyyyyyyyyyyy:.`
      .:yyyyyyyyyyyyyyyyyyyyyyyy:.
    .:yyyyyyyyyyyyyyyyyyyyyyyyyyyy:.
   -yyyyyyyyyyyyyy${c2}+hNMMMNh+${c1}yyyyyyyyy-
  :yy${c2}mNy+${c1}yyyyyyyy${c2}+Nmso++smMdhyysoo+${c1}yy:
 -yy${c2}+MMMmmy${c1}yyyyyy${c2}hh${c1}yyyyyyyyyyyyyyyyyyy-
.yyyy${c2}NMN${c1}yy${c2}shhs${c1}yyy${c2}+o${c1}yyyyyyyyyyyyyyyyyyyy.
:yyyy${c2}oNM+${c1}yyyy${c2}+sso${c1}yyyyyyy${c2}ss${c1}yyyyyyyyyyyyy:
:yyyyy${c2}+dNs${c1}yyyyyyy${c2}++${c1}yyyyy${c2}oN+${c1}yyyyyyyyyyyy:
:yyyyy${c2}oMMmhysso${c1}yyyyyyyyyy${c2}mN+${c1}yyyyyyyyyyy:
:yyyyyy${c2}hMm${c1}yyyyy${c2}+++${c1}yyyyyyy${c2}+MN${c1}yyyyyyyyyyy:
.yyyyyyy${c2}ohmy+${c1}yyyyyyyyyyyyy${c2}NMh${c1}yyyyyyyyyy.
 -yyyyyyyyyy${c2}++${c1}yyyyyyyyyyyy${c2}MMh${c1}yyyyyyyyy-
  :yyyyyyyyyyyyyyyyyyyyy${c2}+mMN+${c1}yyyyyyyy:
   -yyyyyyyyyyyyyyyyy${c2}+sdMMd+${c1}yyyyyyyy-
    .:yyyyyyyyy${c2}hmdmmNMNdy+${c1}yyyyyyyy:.
      .:yyyyyyy${c2}my${c1}yyyyyyyyyyyyyyy:.
        `.:yyyy${c2}s${c1}yyyyyyyyyyyyy:.`
           `.:/oosyyyysso/:.`
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn ubuntu_budgie() -> Logo {
    let x = r"
${c2}           ./oydmMMMMMMmdyo/.
        :smMMMMMMMMMMMhs+:++yhs:
     `omMMMMMMMMMMMN+`        `odo`
    /NMMMMMMMMMMMMN-            `sN/
  `hMMMMmhhmMMMMMMh               sMh`
 .mMmo-     /yMMMMm`              `MMm.
 mN/       yMMMMMMMd-              MMMm
oN-        oMMMMMMMMMms+//+o+:    :MMMMo
m/          +NMMMMMMMMMMMMMMMMm. :NMMMMm
M`           .NMMMMMMMMMMMMMMMNodMMMMMMM
M-            sMMMMMMMMMMMMMMMMMMMMMMMMM
mm`           mMMMMMMMMMNdhhdNMMMMMMMMMm
oMm/        .dMMMMMMMMh:      :dMMMMMMMo
 mMMNyo/:/sdMMMMMMMMM+          sMMMMMm
 .mMMMMMMMMMMMMMMMMMs           `NMMMm.
  `hMMMMMMMMMMM.oo+.            `MMMh`
    /NMMMMMMMMMo                sMN/
     `omMMMMMMMMy.            :dmo`
        :smMMMMMMMh+-`   `.:ohs:
           ./oydmMMMMMMdhyo/.
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn ubuntu_cinnamon() -> Logo {
    let x = r"
${c1}            .-/+oooooooo+/-.
        `:+oooooooooooooooooo+:`
      -+oooooooooooooooooooooooo+-
    .ooooooooooooooooooo${c2}:ohNd${c1}oooooo.
   /oooooooooooo${c2}:/+oo++:/ohNd${c1}ooooooo/
  +oooooooooo${c2}:osNdhyyhdNNh+:+${c1}oooooooo+
 /ooooooooo${c2}/dN/${c1}ooooooooo${c2}/sNNo${c1}ooooooooo/
.ooooooooo${c2}oMd:${c1}oooooooooooo${c2}:yMy${c1}ooooooooo.
+ooooo${c2}:+o/Md${c1}oooooo${c2}:sm/${c1}oo/ooo${c2}yMo${c1}oooooooo+
ooo${c2}:sdMdosMo${c1}ooooo${c2}oNMd${c1}//${c2}dMd+${c1}o${c2}:so${c1}ooooooooo
oooo${c2}+ymdosMo${c1}ooo${c2}+mMm${c1}+/${c2}hMMMMMh+hs${c1}ooooooooo
+oooooo${c2}:${c1}:${c2}/Nm:${c1}/${c2}hMNo${c1}:y${c2}MMMMMMMMMM+${c1}oooooooo+
.ooooooooo${c2}/NNMNy${c1}:o${c2}NMMMMMMMMMMo${c1}ooooooooo.
/oooooooooo${c2}:yh:${c1}+m${c2}MMMMMMMMMMd/${c1}ooooooooo/
  +oooooooooo${c2}+${c1}/h${c2}mMMMMMMNds//o${c1}oooooooo+
   /oooooooooooo${c2}+:////:o/ymMd${c1}ooooooo/
    .oooooooooooooooooooo${c2}/sdh${c1}oooooo.
      -+oooooooooooooooooooooooo+-
        `:+oooooooooooooooooo+:`
            .-/+oooooooo+/-.
";
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn arch() -> Logo {
    let x = r"
    ${c1}                   -`
                      .o+`
                     `ooo/
                    `+oooo:
                   `+oooooo:
                   -+oooooo+:
                 `/:-:++oooo+:
                `/++++/+++++++:
               `/++++++++++++++:
              `/+++o${c2}oooooooo${c1}oooo/`
    ${c2}         ${c1}./${c2}ooosssso++osssssso${c1}+`
    ${c2}        .oossssso-````/ossssss+`
           -osssssso.      :ssssssso.
          :osssssss/        osssso+++.
         /ossssssss/        +ssssooo/-
       `/ossssso+/:-        -:/+osssso+-
      `+sso+:-`                 `.-/+oso:
     `++:.                           `-/+/
     .`                                 `/";

    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

pub fn garuda() -> Logo {
    let x = r#"
${c3}
                     .%;888:8898898:
                   x;XxXB%89b8:b8%b88:
                .8Xxd                8X:.
              .8Xx;                    8x:.
            .tt8x          ${c6}.d${c3}            x88;
         .@8x8;          ${c6}.db:${c3}              xx@;
       ${c4},tSXX°          .bbbbbbbbbbbbbbbbbbbB8x@;
     .SXxx            bBBBBBBBBBBBBBBBBBBBbSBX8;
   ,888S                                     pd!
  8X88/                                       q
  GBB.
   ${c5}x%88        d888@8@X@X@X88X@@XX@@X@8@X.
     dxXd    dB8b8b8B8B08bB88b998888b88x.
      dxx8o                      .@@;.
        dx88                   .t@x.
          d:SS@8ba89aa67a853Sxxad.
            .d988999889889899dd.
"#;
    let d = self::dimensions(x);
    let ascii = self::colorise(x);
    Logo {
        ascii: ascii,
        width: d.0,
        height: d.1,
    }
}

fn colorise(x: &str) -> String {
    x.replace("${c1}", "\x1B[0m\x1B[31m")
        .replace("${c2}", "\x1B[0m\x1B[32m")
        .replace("${c3}", "\x1B[0m\x1B[33m")
        .replace("${c4}", "\x1B[0m\x1B[34m")
        .replace("${c5}", "\x1B[0m\x1B[35m")
        .replace("${c6}", "\x1B[0m\x1B[36m")
}

fn dimensions(s: &str) -> (u16, u16) {
    let tokens = s.to_string();
    let mut tokens: Vec<_> = tokens.split('\n').map(|x| x.len()).collect();
    tokens.sort();
    tokens.reverse();
    if tokens.len() > 0 {
        if let Ok(f) = tokens[0].try_into() {
            if let Ok(y) = tokens.len().try_into() {
                return (f, y);
            }
        }
    }
    (0, 0)
}
