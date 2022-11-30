## Temperature conversion program written in Rust.
Toy console application that takes as input a number, including decimal number, and a unit designator.  The unit can either be C for Celsius or F for Fahrenheit.  The input is sanitized and tested for invalid inputs using the powerful match expression in Rust.  The input can be invalid and the program should not crash.  Valid input can be of format 44c, 56.3F, 21.45 f, 5 F, etc.  The output is the converted value to the opposite unit.

|Rustc version|OS|
|--|--|
|1.64.0 (a55dd71d5 2022-09-19)|Linux 5.15.74.2-microsoft-standard-WSL2 #1 SMP Wed Nov 2 19:50:29 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux  |
