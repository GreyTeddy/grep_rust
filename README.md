# grep_rust
*Simple implementation of grep using the Rust language*

## Run

Running from source using *cargo*

- Arguments
  - ```query```: the word to be searched on a file
  - ```file_path```: the file to be searched
```bash
cargo run -- query file_path
```
- Environment Variables
  - ```IGNORE_CASE=1```
    - ignore ```query``` case: *Rust = ruSt = RUSt = rust*

### Follows the tutorial from the [Rust Book](https://doc.rust-lang.org/book) with key differences

- Implementation of ```Iterator<Item = String>``` when collecting arguments 
  - Only needed arguments are set to memory instead of collecting all of them in a ```Vector```
- More Concise implementaion 
  - Implementation done in ```lib.rs``` with only ```config``` building and ```run``` functions called in `main.rs`
