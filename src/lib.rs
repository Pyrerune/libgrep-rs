/*!
    grep-rs is a simple tool for searching through text  
    * [Github](https://github.com/Pyrerune/grep-rs)
    ## Currently Working Features
    * Support for searching through files
    * Support for searching through standard input
    * Searching through text that includes specific patterns
    * Searching through text that excludes specific patterns
    * Printing all lines before the first instance of the pattern  
    * Printing all lines after the first instance of the pattern 
    * Case Insensitivity 
    * Simple Regular Expressions 
    ## Installation
    Add this to your Cargo.toml
    ```toml
    libgrep-rs = "0.1.3"
    ```
    ## Example
    ```no_run
    use libgrep_rs::searcher::Searcher;
    use libgrep_rs::options::Options;

    fn main() {
        let options = Options::default();
        let text = String::from("Hello World\n libgrep-rs test");
        let pattern = String::from("World");
        let searcher = Searcher::new(pattern, text, options, Some(String::from("\n")));
        //Some(String::from("\n")) is the deliminator, it could be anything
        //If set to None, it will use the default "\n"
        let output = searcher.search();
        println!("{}", output);
    }
    ```
    If it worked, the output will be
    ```txt
    Hello World
    ```
    You can see other examples in the examples/ directory
*/

pub mod searcher;
pub mod options;
#[cfg(test)]
mod tests {
    
    use crate::searcher::Searcher;
    use crate::options::Options;
    #[test]
    fn exclude() {
        let options = Options {
            exclude: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options, None);
        let assert_text: String = String::from("Gates thrilled");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn include() {
        let options = Options::default();
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options, None);
        let assert_text: String = String::from("Steve Jobs Passed Away");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    
    }
    #[test]
    fn include_before() {
        let options = Options {
            include_before: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away\nGates thrilled\nApple Fans Devastated\nGates Thrilled and Devastated");
        let searcher = Searcher::new(String::from("Gates"), text, options, None);
        let assert_text: String = String::from("Steve Jobs Passed Away Gates thrilled Gates Thrilled and Devastated");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text = return_text.replace("\n", " ");
            if return_text.ends_with(" ") {
                return_text.remove(return_text.len() - 1);
            }
            if return_text.starts_with(" ") {
                return_text.remove(0);
            }
            println!("test: {}", return_text);
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn include_after() {
        let options = Options {
            include_after: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away\nGates thrilled\nApple Fans Devastated");
        let searcher = Searcher::new(String::from("Gates"), text, options, None);
        let assert_text: String = String::from("Gates thrilled Apple Fans Devastated");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text = return_text.replace("\n", " ");
            if return_text.ends_with(" ") {
                return_text.remove(return_text.len() - 1);
            }
            if return_text.starts_with(" ") {
                return_text.remove(0);
            }
            println!("test: {}", return_text);
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn case_insensitive() {
        let options = Options {
            case_insensitive: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away\nGates thrilled\n Steve jobs hasn't passed away");
        let searcher = Searcher::new(String::from("Jobs"), text, options, None);
        let assert_text: String = String::from("Steve Jobs Passed Away Steve jobs hasn't passed away").to_lowercase();
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        if return_text.ends_with("\n") {
            return_text.remove(return_text.len()-1);
        }

        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn regex() {
        let options = Options {
            regex: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away on 2020-05-18\nHe passed away at exactly midnight as he didn't like being late");
        let searcher = Searcher::new(String::from(r"\d"), text, options, None);
        let assert_text = String::from("Steve Jobs Passed Away on 2020-05-18");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        if return_text.ends_with("\n") {
            return_text.remove(return_text.len()-1);
        }

        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn custom_delim() {
        let options = Options::default();
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options, Some(String::from(" ")));
        let assert_text: String = String::from("Jobs");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn show_line() {
        let options = Options {
            show_line: true,
            ..Options::default()
        };
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options, None);
        let assert_text: String = String::from("found at line: 0\nSteve Jobs Passed Away\n");
        let mut return_text: String = searcher.search();

        assert_eq!(assert_text, return_text);
    }
}
