# Help snapshots (0.0.1)

## `neuros --help`

```text
usage: neuros [-h] [--version] COMMAND ...

Neuros CLI - docs-first scaffold for a memory-enabled agent runtime. Use subcommand help to inspect planned command surfaces.

positional arguments:
  COMMAND
    about       Show package status and documentation entry points.
    memory      Memory command group (placeholder surfaces).
    tools       Tool registry command group (placeholder surfaces).
    files       File memory command group (placeholder surfaces).

options:
  -h, --help    show this help message and exit
  --version     show program's version number and exit
```

## `neuros memory --help`

```text
usage: neuros memory [-h] SUBCOMMAND ...

positional arguments:
  SUBCOMMAND
    status      Show memory subsystem status (placeholder).
    add-file    Plan to add a file into memory ingestion flow (placeholder).

options:
  -h, --help    show this help message and exit
```

## `neuros memory add-file --help`

```text
usage: neuros memory add-file [-h] [path]

positional arguments:
  path          File path to register for future memory ingestion.

options:
  -h, --help    show this help message and exit
```

## `neuros tools --help`

```text
usage: neuros tools [-h] SUBCOMMAND ...

positional arguments:
  SUBCOMMAND
    list        List configured tools (placeholder).

options:
  -h, --help    show this help message and exit
```

## `neuros files --help`

```text
usage: neuros files [-h] SUBCOMMAND ...

positional arguments:
  SUBCOMMAND
    index       Prepare a file for memory indexing (placeholder).

options:
  -h, --help    show this help message and exit
```

