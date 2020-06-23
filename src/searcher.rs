//! Contains structs used for searching through text
use crate::options::Options;
use regex::Regex;
use std::process::exit;
///Searches through text for the given pattern
#[derive(Clone)]
pub struct Searcher {
    pub pattern: String,
    pub search_text: String,
    pub options: Options,
    pub deliminator: String
}
impl Searcher {
    pub fn new(p: String, t: String, o: Options, d: Option<String>) -> Searcher {
        let mut pattern = p.clone();
        let mut search_text = t.clone();
        let mut delim = String::from("\n");
        if o.case_insensitive == true {
            pattern = p.to_lowercase();
            search_text = t.to_lowercase();
        }
        if d.is_some() {
            delim = d.unwrap();
        }
        Searcher {
            pattern,
            search_text,
            options: o,
            deliminator: delim,
        }
    }
    pub fn search(&self) -> String {
        let mut found: bool = false;

        let list: Vec<&str> = self.search_text.split(self.deliminator.as_str()).collect();
        
        let mut return_string = String::new();
        let mut len = 0;
        for i in list.clone() {
            if self.options.regex == true {
                
                let regex = Regex::new(self.pattern.as_str());
                if regex.is_err() {
                    eprintln!("Invalid Regex");
                    exit(1);
                } else if regex.unwrap().is_match(i) {
                    return_string.push_str(i);
                    return_string.push_str("\n");
                }
            }
            if i.contains(self.pattern.as_str()) {

                if self.options.exclude == false {
                    if self.options.show_line == true {
                        return_string.push_str(format!("found at line: {}\n", len).as_str());

                    }
                    return_string.push_str(i);
                    return_string.push_str("\n");
                }
                found = true;
            } else if self.options.exclude == true {
                    return_string.push_str(i);
                    return_string.push_str("\n");
                
                
            } else if self.options.include_before == true {
                if found == false {
                    return_string.push_str(i);
                    return_string.push_str("\n");
                }
            } else if self.options.include_after == true {
                if found == true {
                    return_string.push_str(i);
                    return_string.push_str("\n");
                }
            } else {
                len += 1;
            }
        }
 
        return_string
    }
}
