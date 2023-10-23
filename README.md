# Data Engineering Mini Project 8
## Purpose
The purpose of this project is to rewrite an existing Python scipt for data processing in Rust, and demostrate improvements in speed and resource usage. The Python scipt reads in a csv file (cereal dataset here) and returns the maximum number of the 'calories' variable. I rewrote this in Rust in lib.rs and main.rs. In both Python and Rust scipts, I added functions that return elapsed time, cpu usage and memory usage to compare their performance. 

## Python Script Steps
1. Create a reusable Python library with a public function `get_max` that returns maximum number of a dataframe.
2. Use `Pandas` to read in the csv file and use `get_max` to return maximum number of each column
3. Add functions that print out elapsed time, cpu and memory usage.
4. Add a test function in `test_main.py` to check if the maximum number of calories is 160.
5. Make format, lint and test errors.

## Rust Script Steps
1. Rewrite the `get_max` function in Rust as a public function `find_max`. I created a for loop to accomplish this.
2. The `main.rs` file reads data from the csv file using external crates csv and calculates the maximum value of calories with `find_max`.
3. Calculate the usage of time by using instant to collect start time and end time. To get memory and cpu usage, std::process::Command and sys_info::mem_info are used.
4. Run `cargo build` and `cargo run`
5. Add a test function in `test_main.rs` to check if the maximum number of calories is 160.
6. Check format, lint and test errors.

<img width="640" alt="Screen Shot 2023-10-23 at 5 08 24 PM" src="https://github.com/nogibjj/KatherineT.DE.Mini-Project-8/assets/143833511/8893b1f7-73eb-4e3d-9fe8-a48d2230eb5d">

<img width="712" alt="Screen Shot 2023-10-23 at 3 19 26 PM" src="https://github.com/nogibjj/KatherineT.DE.Mini-Project-8/assets/143833511/0a4cc61f-4ea2-4fee-884a-c603382878bd">

## Performance Comparison 
Python usage and time:

<img width="256" alt="Screen Shot 2023-10-23 at 3 04 45 PM" src="https://github.com/nogibjj/KatherineT.DE.Mini-Project-8/assets/143833511/87e7ff16-9a13-4b2e-b2f1-5d2489fc5548">

Rust usage and time:

<img width="454" alt="Screen Shot 2023-10-23 at 3 04 17 PM" src="https://github.com/nogibjj/KatherineT.DE.Mini-Project-8/assets/143833511/80f2c42c-d519-47d4-88ae-fc8c99bfe3cb">

We can see that all three metrics show that Rust perform better than Python in this data processing function. 

