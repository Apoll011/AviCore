use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module};
use dyon::Type::*;
use crate::dialogue::utils::{speak, listen as device_listen};

pub fn add_functions(module: &mut Module) {
    module.ns("dialogue");
    module.add(Arc::new("say".into()), say, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("listen".into()), listen, Dfn::nl(vec![Any], Any)); // Last device that sent a utterance will start listening again
    /*module.add(Arc::new("on_reply".into()), json_stringify, Dfn::nl(vec![Any], Str)); //Sets a handles for the next user sopke text
    module.add(Arc::new("ask".into()), dir, Dfn::nl(vec![], Str)); //Ask a question with a list of asnwers, fuzzy the response or frist second trird etc
    module.add(Arc::new("confirm".into()), dir, Dfn::nl(vec![], Str)); //Ask a yes or no question
    module.add(Arc::new("repeat".into()), dir, Dfn::nl(vec![], Str)); //Repeats the last spoken utterance (Dont matter the skill)
    module.add(Arc::new("request_attention".into()), dir, Dfn::nl(vec![], Str)); //Call the user name without leaving the current skill */
}

dyon_fn! {fn say(text: String) {
    speak(&text);
}}

dyon_fn! {fn listen() {
    device_listen();
}}