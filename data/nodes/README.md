# Disbotter nodes

Here you can find all the nodes that are currently available in Disbotter. You can also create your own ones!

## What language are the nodes written in?

The nodes are written in a slightly modified version of [rhai](https://rhai.rs/).

Basic node structure looks like this:

```rust
// Basic node data
const id = "node_id";
const title = "Node title (displayed in the editor)";
const description = "Node description (displayed in the editor)";
const category = "Node category (displayed in the editor)";

// Optionals (remove variables to disable)
const noFlowIn; // Disables input flow
const noFlowOut; // Disables output flow
// Makes the node pure, this means that node will not have
// flow I/O but will be compiled when generating code from nodes
// (Pure functions should depend only on their input and not on any other data)
const pure;

// Inputs and outputs
const inputs = #{
    example_input: #{
        name: "Example input",
        // Input type (text, number, boolean, struct, any)
        type: "text",
        // Input default value
        // (supported for text, number and boolean)
        start_value: "Hello world!"
    },
    example_struct_input: #{
        name: "Example struct input",
        type: "struct",
        // Struct tags (used to identify structs)
        // Input will only accept structs with all the tags
        struct_tags: [
            "example_tag"
        ]
    }
};
const outputs = #{
    example_output: #{
        name: "Example output",
        // Output type (flow, text, number, boolean, struct, any)
        type: "text"
    },
};

fn action(builder) {
    let example_input = inv example_input;
    let example_struct_input = inv example_struct_input;

    -> `console.log(${example_input});`;

    out example_output = "Hello world!";
}
```

## What is the `builder` variable?

`builder` is a variable that is used to build the node code. It has the following methods:

- `builder.add_line(code)` - Adds code to the node
- `builder.begin_block()` - Begins a new code block
- `builder.end_block()` - Ends the current code block
- `builder.add_import(imports, name)` - Adds an import to the node
- `builder.get_input(name)` - Gets the input variable
- `builder.get_out_var(name)` - Gets the output variable
- `builder.set_output(name, value)` - Sets the output variable
- `builder.bind_io(in_var, out_var)` - Directly redirects input to output
- `builder.push_stack()` - Pushes scope to the stack (automatic in begin_block)
- `builder.pop_stack()` - Pops scope from the stack (automatic in end_block)
- `builder.compile_flow_output_here(name)` - Compiles code from the flow output with the given name and adds it to the current node

However, there is some special syntax that you can use in the node code to make it easier to write:

- Instead of writing `builder.add_line(code);` you can just write `-> code;`.
- Instead of writing `builder.get_input(name);` you can just write `inv name;`.
- Instead of writing `builder.set_output(name, value);` you can just write `out name = value;`.

Nodes also have some special variables that you can use, those begin with `___`:

For example, in commands, there is a `___interaction` variable that contains the command interaction data and `___translations` variable that contains `LocalizedTranslations` struct.
