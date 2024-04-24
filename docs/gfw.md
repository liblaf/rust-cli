# Command-Line Help for `gfw`

This document contains the help content for the `gfw` command-line program.

**Command Overview:**

- [`gfw`↴](#gfw)
- [`gfw complete`↴](#gfw-complete)
- [`gfw ip`↴](#gfw-ip)
- [`gfw list`↴](#gfw-list)
- [`gfw now`↴](#gfw-now)

## `gfw`

**Usage:** `gfw [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `complete` —
- `ip` —
- `list` —
- `now` —

###### **Options:**

- `--color <WHEN>` — Controls when to use color

  Default value: `auto`

  Possible values: `auto`, `always`, `never`

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

## `gfw complete`

**Usage:** `gfw complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `gfw ip`

**Usage:** `gfw ip [OPTIONS] [ADDR]`

###### **Arguments:**

- `<ADDR>`

###### **Options:**

- `--geo <GEO>`

  Default value: `true`

  Possible values: `true`, `false`

- `--risk <RISK>`

  Default value: `true`

  Possible values: `true`, `false`

- `--security <SECURITY>`

  Default value: `true`

  Possible values: `true`, `false`

## `gfw list`

**Usage:** `gfw list [OPTIONS] [URLS]...`

###### **Arguments:**

- `<URLS>`

###### **Options:**

- `-u`, `--url`

  Default value: `false`

  Possible values: `true`, `false`

- `--uuid <UUID>`

## `gfw now`

**Usage:** `gfw now [OPTIONS]`

###### **Options:**

- `-a`, `--api <API>`

  Default value: `http://127.0.0.1:9090`

- `--delay <DELAY>`

  Default value: `true`

  Possible values: `true`, `false`

- `--emoji <EMOJI>`

  Default value: `true`

  Possible values: `true`, `false`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
