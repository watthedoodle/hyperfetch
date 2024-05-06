pub fn free_bsd() -> String {
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
    self::colorise(x)
}

pub fn arch() -> String {
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
    self::colorise(x)
}

fn colorise(x: &str) -> String {
    x.replace("${c1}", "\x1B[0m\x1B[31m")
     .replace("${c2}", "\x1B[0m\x1B[32m")
     .replace("${c3}", "\x1B[0m\x1B[33m")
     .replace("${c4}", "\x1B[0m\x1B[34m")
     .replace("${c5}", "\x1B[0m\x1B[35m")
     .replace("${c6}", "\x1B[0m\x1B[36m")
}