use crate::language_gen::LangExtGen;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LangConfig {
    pub path: String,
    pub lang: String,
    pub scope_name: String,
}

impl LangConfig {}

pub struct GrammarGen {}

impl GrammarGen {
    pub fn new() -> Self {
        GrammarGen {}
    }

    pub fn build_grammar_bin_data() -> BTreeMap<String, LangConfig> {
        let langs = LangExtGen::default();
        let mut raw_grammar_map: BTreeMap<String, LangConfig> = Default::default();
        for (ext, entry) in langs.ext_map.iter() {
            let lang = entry.name.clone();
            let tm_grammar = langs.grammar_map[&lang].clone();

            let mut grammar_path = entry.path.clone();
            grammar_path.push_str(&*tm_grammar.path);
            grammar_path = grammar_path.replace("./", "/");

            if tm_grammar.language.is_some() {
                raw_grammar_map.insert(
                    String::from(ext),
                    LangConfig {
                        path: grammar_path,
                        lang: tm_grammar.language.unwrap(),
                        scope_name: tm_grammar.scope_name,
                    },
                );
            }
        }

        raw_grammar_map
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar_gen::GrammarGen;

    #[test]
    fn should_build_default_maps() {
        let map = GrammarGen::build_grammar_bin_data();
        assert!(map[".zshrc"].path.contains("extensions/shellscript/syntaxes/shell-unix-bash.tmLanguage.json"));
    }
}
