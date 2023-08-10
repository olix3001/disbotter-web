const id = "_builtin_var_set";
const title = "Set Variable";
const description = "Set a variable to a value.";
const category = "Variables";

const inputs = #{
    name: #{
        name: "variable",
        type: "text",
        index: 0
    },
    value: #{
        name: "value",
        type: "any",
        index: 1
    }
};
const outputs = #{};

fn action(builder) {
    let name = inv name;
    let name = name.sub_string(1, name.len - 2);

    -> `__user_var_${name} = ${inv value}`;
}