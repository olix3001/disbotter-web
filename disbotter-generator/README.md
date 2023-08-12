# Disbotter CLI

## Installation

clone this repository, then go to the `disbotter-generator` folder and run `cargo install` to install the CLI.

## Usage

### Generate new declarations

Web interface uses node declarations in .json files to show the nodes. If you added a new node, you need to generate a new declaration file.

Declarations are located in `static/generated/` folder. There is a file named `command_node_declarations.json` which contains all the command and common nodes.

To generate a new declaration file, run `disbotter gen-node-declarations` command. Remember to provide all necessary arguments:

- `--path` - path to the folder where the node declarations are located
- `--output` - path to the output file

Example:

```bash
disbotter gen-node-declarations --path ./data/nodes --output ./static/generated/command_node_declarations.json
```

### Compile the project

To compile the project, run `disbotter compile` command. Remember to provide all necessary arguments:

- `--path` - path to the folder where the node declarations are located
- `--nodes` - path where the nodes are located (usually `./data/nodes`)
- `--output` - path to the output file

Example:

```bash
disbotter compile --nodes ./data/nodes --path ./my_project.dbp --output ./my_project/
```

This will generate basic project structure in `./my_project/` folder. Remember to use [disbotter](https://github.com/olix3001/disbotter) as all the files are generated for it.
