https://github1s.com/joshrotenberg/adrs/blob/main/src/cmd/new.rs
https://rust-cli.github.io/book/in-depth/docs.html
https://github.com/marouni/adr/blob/master/commands.go
https://github.com/npryce/adr-tools/tree/master?tab=readme-ov-file

# equidistant -> https://dba.stackexchange.com/questions/320935/efficient-downsampling-of-a-selected-timeseries-to-equidistant-samples

## Features

### Commands

-   Init - initialize a new project with a folder path
-   Switch - to switch between projects
-   All - to list all projects
-   List - list all ADRs
-   New - new ADR
-   Link - link different ADRs together
-   Update - update ADR
-   Delete - delete command
-   Reset - reset for a project
-   Help - description of all commands

### Features

-   should manage multiple projects at the same time
-   initialization requires a name and project path for a given project
-   The commands are always within a project scope
