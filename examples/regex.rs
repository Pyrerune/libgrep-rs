use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    let options = Options::new(false, false, false, false, true);
    let text = String::from("Basic_file_1.txt\nBasic_file_2.txt\nBasic_file_3.txt\nBasic_file_4.txt\n");
    //Search for all text that includes a digit
    let searcher = Searcher::new(String::from(r"\d"), text, options);
    let return_text: String = searcher.search();
    //return text should include everything from text
    print!("{}", return_text);
}