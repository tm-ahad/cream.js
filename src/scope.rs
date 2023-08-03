use crate::collect_scope::collect_scope;
use crate::config::Config;
use crate::matcher::Matcher;
use crate::sys_exec::sys_exec;
use std::fs::{read_to_string, write};

pub fn parse_scope(script: &mut String, scopes: &mut Vec<String>) {
    let mut indexes = (0, 0, 0);
    let matchers: [Matcher; 3] = [Matcher::Dom, Matcher::Cam, Matcher::Sin];

    for m in matchers {
        while let Some(pat) = collect_scope(script, &m, true) {
            let ind = pat.index();
            indexes.0 += ind;

            let end_i = match pat.ext() {
                &Some(a) => a,
                None => panic!("Unvalid code executed!"),
            };

            scopes.push(pat.mp().clone());
            script.replace_range(ind..end_i, "// ");
        }
    }
}

pub fn scopify(script: &mut String, scopes: Vec<String>, config: &Config) {
    let mut sid: usize = 0;

    while let Some(i) = script.find("// ")  {
        let v = scopes.get(sid)
            .unwrap_or_else(|| panic!("Invalid substring // "));

        write(format!("./build/.$.{}", config.expect("lang")), &v)
            .unwrap_or_else(|e| panic!("{:?}", e));

        sys_exec(format!(
            "{} ./build/.$.{}",
            config.expect("build"),
            config.expect("lang")
        ));
        let res = read_to_string("./build/.$.js").unwrap();
        script.replace_range(i..i+3, &res);

        sid += 1
    }
}
