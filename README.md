# create_cpp_app

Small script written in `Rust` that will automatically create a basic `C++` app at a specified location. By default there will be a `main.cpp` file, as well as a
`Makefile`. Optionally, you can add an input file, an output file or initialize the project with `git`.

```
USAGE:
    create_cpp_app [FLAGS] [OPTIONS] <name>

ARGS:
    <name>    The name of the project

FLAGS:
        --git        Initialize with git
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input file
    -o, --output <output>    Output file
```
After running the program, you can run:
* `make` - compile and run `main.cpp`
* `make io` - create the input and/or the output file (if the prgram was given the option `--input` and/or `--output`)
* `make clean` - delete the binary created when compiling `main.cpp`
