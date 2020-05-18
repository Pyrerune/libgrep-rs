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
    pub fn new(e: bool, ib: bool, ia: bool, cs: bool, r: bool) -> Options {
        Options {
            exclude: e,
            include_before: ib,
            include_after: ia,
            case_insensitive: cs,
            regex: r,
        }
    }
}
