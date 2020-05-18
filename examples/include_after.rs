use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    let options = Options::new(false, false, true, false, false);
    let text = String::from("Basic_file_1.txt\nBasic_file_2.txt\nBasic_file_3.txt\nBasic_file_4.txt\n");
    let searcher = Searcher::new(String::from("2"), text, options);
    let return_text: String = searcher.search();
    //return text should Basic_file_2.txt   Basic_file_3.txt   Basic_file_4.txt
    print!("{}", return_text);
}