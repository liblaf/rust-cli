# Command-Line Help for `cf`

This document contains the help content for the `cf` command-line program.

**Command Overview:**

- [`cf`↴](#cf)
- [`cf complete`↴](#cf-complete)
- [`cf dns`↴](#cf-dns)
- [`cf dns install`↴](#cf-dns-install)
- [`cf dns list`↴](#cf-dns-list)
- [`cf dns update`↴](#cf-dns-update)

## `cf`

**Usage:** `cf [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `complete` —
- `dns` —

###### **Options:**

- `--api-url <API_URL>`

  Default value: `https://api.cloudflare.com/client/v4`

- `--token <TOKEN>`
- `--color <WHEN>` — Controls when to use color

  Default value: `auto`

  Possible values: `auto`, `always`, `never`

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

## `cf complete`

**Usage:** `cf complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `cf dns`

**Usage:** `cf dns [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `install` —
- `list` —
- `update` —

###### **Options:**

- `-z`, `--zone-id <ZONE_ID>`

  Default value: `919b04037636d3b4bbc0af135eaccdfa`

## `cf dns install`

**Usage:** `cf dns install`

## `cf dns list`

**Usage:** `cf dns list [OPTIONS]`

###### **Options:**

- `-n`, `--name <NAME>`

## `cf dns update`

**Usage:** `cf dns update [OPTIONS]`

###### **Options:**

- `-n`, `--name <NAME>`
- `--telepush-token <TELEPUSH_TOKEN>`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
