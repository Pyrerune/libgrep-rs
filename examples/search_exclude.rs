use libgrep_rs::options::Options;
use libgrep_rs::searcher::Searcher;

fn main() {
    let options = Options {
        exclude: true,
        ..Options::default()
    };
    let text = String::from("Basic_file_1.txt\nBasic_file_2.txt\nBasic_file_3.txt\nBasic_file_4.txt\n");
    let searcher = Searcher::new(String::from("1"), text, options, Some(String::from("\n")));
    let return_text: String = searcher.search();
    //return text should everything except Basic_file_1.txt
    print!("{}", return_text);
}