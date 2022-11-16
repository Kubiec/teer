
# teer

A simple rust clone of UNIX tee, just for fun. Inspiration taken from O'Reilly book. 


## Usage/Examples

```bash
Copy standard input to each FILE, and also to standard output

Usage: teer [OPTIONS] [FILE]...

Arguments:
  [FILE]...  output files

Options:
  -a, --append             append to the given FILEs, do not overwrite
  -i, --ignore-interrupts  ignore interrupt signals
  -h, --help               Print help information
  -V, --version            Print version information
```


## License

[MIT](https://choosealicense.com/licenses/mit/)

