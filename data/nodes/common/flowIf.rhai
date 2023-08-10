const id = "_builtin_flow_if";
const title = "Branch If";
const description = "Branches the flow based on a condition.";
const category = "Flow Control";

const inputs = #{
    condition: #{
        name: "Condition",
        type: "boolean",
    },
};
const outputs = #{
    __flow_if_true: #{
        name: "If True",
        type: "flow",
    },
    __flow_if_false: #{
        name: "If False",
        type: "flow",
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