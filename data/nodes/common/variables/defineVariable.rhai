const id = "_builtin_var_define";
const title = "Define variable";
const description = "Define a variable with a given name and value.";
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

    -> `var __user_var_${name}: any = ${inv value};`;
}