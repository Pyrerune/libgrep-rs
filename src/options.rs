//! Contains structs required for the configuration of the searcher module

///Configurations options for Searcher

#[derive(Clone)]
pub struct Options {
    pub exclude: bool,
    pub include_before: bool,
    pub include_after: bool,
    pub case_insensitive: bool,
    pub regex: bool,
}
impl Default for Options {

    fn default() -> Options {
        Options {
            exclude: false,
            include_before: false,
            include_after: false,
            case_insensitive: false,
            regex: false,
        }
    }
}
impl Options {
    pub fn new(exclude: bool, include_before: bool, include_after: bool,
         case_insensitive: bool, regex: bool) -> Options {
        Options {
            exclude,
            include_before,
            include_after,
            case_insensitive,
            regex,
        }
    }
}
