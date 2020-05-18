use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    //default() sets all options to false
    let options = Options::default();
    let text = String::from("Basic_file_1.txt\nBasic_file_2.txt\nBasic_file_3.txt\nBasic_file_4.txt\n");
    let searcher = Searcher::new(String::from("1"), text, options);
    let return_text: String = searcher.search();
    //return text should be: Basic_file_1.txt
    print!("{}", return_text);
}