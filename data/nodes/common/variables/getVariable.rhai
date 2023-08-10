const id = "_builtin_var_get";
const title = "Get Variable";
const description = "Get the value of a variable.";
const category = "Variables";

const inputs = #{
    name: #{
        name: "variable",
        type: "text",
    }
};
const outputs = #{
    value: #{
        name: "value",
        type: "any",
    }
};

fn action(builder) {
    let name = inv name;
    let name = name.sub_string(1, name.len - 2);

    builder.map_io("value", `__user_var_${name}`);
}