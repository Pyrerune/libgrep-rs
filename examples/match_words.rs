use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    
    let options = Options::default();
    let text = String::from("Steve Jobs unveiled the iProduct today");
    
    let searcher = Searcher::new(String::from("Jobs"), text, options, Some(String::from(" ")));
    let return_text: String = searcher.search();
    //should print: Jobs
    print!("{}", return_text);
}