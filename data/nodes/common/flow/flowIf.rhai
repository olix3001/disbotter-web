const id = "_builtin_flow_if";
const title = "Branch If";
const description = "Branches the flow based on a condition.";
const category = "Flow Control";

const inputs = #{
    condition: #{
        name: "condition",
        type: "boolean",
    },
};
const outputs = #{
    __flow_if_true: #{
        name: "if true",
        type: "flow",
        index: 0
    },
    __flow_if_false: #{
        name: "if false",
        type: "flow",
        index: 1
    },
};

fn action(builder) {
    let condition = inv condition;
    
    -> `if (${condition}) {`;
    builder.begin_block();
    builder.compile_flow_output_here("__flow_if_true");
    builder.end_block();
    -> "} else {";
    builder.begin_block();
    builder.compile_flow_output_here("__flow_if_false");
    builder.end_block();
    -> "}";
}