# filename-refactor

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/toshiki670/filename-refactor/rust.yml)
![Crates.io License](https://img.shields.io/crates/l/filename-refactor)
![Crates.io Size](https://img.shields.io/crates/size/filename-refactor)
![Crates.io Version](https://img.shields.io/crates/v/filename-refactor)
![Crates.io Total Downloads](https://img.shields.io/crates/d/filename-refactor)

## Overview

Command to refactor file names

## Usage

```bash
$ ilename-refactor -h
Filename refactor tool

Usage: filename-refactor [OPTIONS] [COMMAND]

Commands:
  f2h
  translate
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Show logs
      --generate-completions <SHELL_NAME>
          Generate shell completions [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help
          Print help
  -V, --version
          Print version
```

``` bash
$ filename-refactor f2h -h
Usage: filename-refactor f2h [OPTIONS] [INPUT PATTERNS]...

Arguments:
  [INPUT PATTERNS]...  Input files (glob patterns supported: *.json)

Options:
  -v, --verbose  Show logs
  -h, --help     Print help
```

``` bash
$ filename-refactor translate -h
Usage: filename-refactor translate [OPTIONS] --target <LANGUAGE> [INPUT PATTERNS]...

Arguments:
  [INPUT PATTERNS]...  Input files (glob patterns supported: *.json)

Options:
  -s, --source <LANGUAGE>  Translate from language [possible values: ja, en, ar, de, es, fr, it, pt, ru, zh]
  -v, --verbose            Show logs
  -t, --target <LANGUAGE>  Translate to language [possible values: ja, en, ar, de, es, fr, it, pt, ru, zh]
  -h, --help               Print help
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
