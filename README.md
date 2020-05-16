# grep-rs  
A grep-like library written in rust  
  
#### Currently working features  
Searching through files  
Searching through standard input  
Excluding patterns  
Printing all lines before the first instance of the pattern  
Printing all lines after the first instance of the pattern
(NEW) Case Insensitivity  
#### Examples  
Examples take place in hypothetical directory containing  
Basic_file_1.txt  Basic_file_2.txt  
Basic_file_3.txt  Basic_file_4.txt  
##### Searching through stdin  
ls | grep-rs Basic_file_1  
output: Basic_file_1.txt  
  
##### Searching through files  
grep-rs text Basic_file_1.txt  
output: The text states .....  
  
##### Excluding patterns  
ls | grep-rs -e Basic_file_1  
output:  
Basic_file_2.txt  
Basic_file_3.txt  
Basic_file_4.txt  
##### Printing all lines before the first instance of the pattern  
ls | grep-rs -I Basic_file_3  
output:  
Basic_file_1.txt  
Basic_file_2.txt  
Basic_file_3.txt  
##### Printing all lines after the first instance of the pattern  
ls | grep-rs -i Basic_file_3  
output:  
Basic_file_3.txt  
Basic_file_4.txt  
##### Case Insensitivity  
ls | grep-rs -c basic_file_3
output: basic_file_3.txt