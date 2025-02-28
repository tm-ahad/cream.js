use crate::collect_scope::collect_scope;
use crate::component_markup::ComponentMarkUp;
use crate::extract_component::extract_component;
use crate::helpers::expected::expect_some;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::import_script::import_script;
use crate::matcher::Matcher;
use crate::scope::{parse_scope, scopify};
use crate::script_module::module;
use crate::state_base::_StateBase;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::transpile_component::transpile_component;
use crate::import_component::import_component;
use crate::consts::{COMPONENT_CALL_SIGN, COMPONENT_CALL_SIGN_LEN, DOUBLE_QUOTE, IGNORE_STATE, NEW_LINE_CHAR, NIL};
use crate::helpers::merge_dom_script::merge_dom_script;
use crate::import_template::import_template;
use crate::import_html::import_html;
use crate::import_ext::import_ext;
use crate::template::template;
use crate::comment::comment;
use crate::gen_id::gen_id;
use crate::state::_state;
use crate::udt::UDT;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use crate::component_map::ComponentMap;

pub struct Component {
    pub html: ComponentMarkUp,
    pub dyn_script: String,
    pub script: String,
    pub name: String,
}

impl Component {
    pub fn new(
        script: String,
        dyn_script: String,
        html: ComponentMarkUp,
        name: String
    ) -> Self {
        Component {
            script,
            dyn_script,
            html,
            name
        }
    }
}

impl Clone for Component {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            script: self.script.clone(),
            html: self.html.clone(),
            dyn_script: self.dyn_script.clone()
        }
    }
}

pub fn component(
    f_name: String,
    c_name: String,
    component_map: &mut ComponentMap
) -> Component {
    let import_base = &mut ImportBase::new();
    let st = &mut _StateBase::new();
    let config = component_map.config();

    let __script__ = &String::from("script");
    let lang = config.get("lang").unwrap_or(__script__);
    let path = format!("./{f_name}").replace(DOUBLE_QUOTE, NIL);

    let mut app = read_to_string(path.clone()).unwrap_or_else(|e| {
        StdErr::exec(OSError, &format!("{path}: {}", &e));
        todo!()
    }).lines()
      .map(|e| e.trim())
      .collect::<Vec<&str>>()
      .join("\n");

    comment(&mut app);

    let macher = Matcher::Component(&c_name);
    let pat = expect_some(collect_scope(app.as_str(), &macher, false), &c_name);
    let main_app = pat.mp_val();
    let mut dom_script = String::new();

    let binding = &main_app;
    let split = binding.split('\n');

    let mut script = String::new();
    let binding = Matcher::Template.to_string();
    let t = binding.as_str();

    for s in split {
        if s != t {
            script.push(NEW_LINE_CHAR);
            script.push_str(s)
        } else {
            break;
        }
    }

    let template_mp = expect_some(
        collect_scope(&main_app, &Matcher::Template, false),
        "Template",
    );

    let mut html = template_mp.mp_val();

    import_script(&mut app, import_base, &mut script, &f_name);
    import_template(&mut app, &f_name, &mut html);

    let mut cmu = ComponentMarkUp::new(html.clone(), html.clone());
    let mut ccm = BTreeMap::new();
    let mut scopes = Vec::new();
    let mut dyn_script = script.clone();

    gen_id(
        &mut script,
        &mut dyn_script,
        &mut cmu,
        import_base,
        true,
        lang,
        &f_name
    );

    import_lib(&mut app, import_base, &mut script, &f_name);
    module(&mut app, import_base, &mut script, &f_name);
    parse_scope(&mut script, &mut scopes);

    script = script.replace(IGNORE_STATE, NIL).replace(".cam()", "");

    import_npm(&mut app, &mut script, &f_name);
    scopify(&mut script, scopes, config, st, &f_name);

    let imports = import_component(&app, f_name.clone(), component_map);
    extract_component(&mut ccm, &imports, &mut cmu, &f_name);
    UDT(&mut html, &mut script, &imports, &f_name);

    template(&mut cmu, &mut dom_script, st, &f_name);

    let script_writer_ptr = &mut dom_script;
    transpile_component(
        ccm,
        script_writer_ptr,
        &mut cmu
    );

    merge_dom_script(&mut script, &dom_script);
    _state(&mut script, st, &f_name);
    import_ext(&mut app, &f_name, &mut script);
    import_html(&mut app, &f_name, &mut html);

    Component::new(
        script,
        dyn_script,
        cmu,
        c_name.to_string(),
    )
}

pub fn component_call(id: u32) -> String {
    format!("{}{id}", COMPONENT_CALL_SIGN)
}

pub fn component_call_len(dnl: usize) -> usize {
    COMPONENT_CALL_SIGN_LEN + dnl
}
