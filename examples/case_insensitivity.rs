use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    let options = Options {
        case_insensitive: true,
        ..Options::default()
    };
    let text = String::from("Basic_file_1.txt\nBasic_file_2.txt\nbasic_file_3.txt\nbasic_file_4.txt\n");
    let searcher = Searcher::new(String::from("b"), text, options, Some(String::from("\n")));
    let return_text: String = searcher.search();
    //return text should include everything
    print!("{}", return_text);
}