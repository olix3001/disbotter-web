const id = "_builtin_flow_loop_times";
const title = "Loop N Times";
const description = "Loops a set number of times.";
const category = "Flow Control";

const inputs = #{
    n: #{
        name: "N",
        type: "number",
        start_value: 1,
    },
};
const outputs = #{
    __flow_loop: #{
        name: "loop",
        type: "flow",
    },
};

fn action(builder) {
    let times = inv n;
    let loop_var = builder.get_random_var_name();

    -> `for (let __loop_${loop_var}=0; __loop_${loop_var} < ${times}; __loop_${loop_var}++) {`;
    builder.begin_block();
    builder.compile_flow_output_here("__flow_loop");
    builder.end_block();
    -> "}";
}