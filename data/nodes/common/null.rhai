const id = "_builtin_constant_null";
const title = "Null";
const description = "";
const category = "Constants";

const pure;

const inputs = #{};
const outputs = #{
    result: #{
        name: "value",
        type: "any",
    }
};

fn action(builder) {
    builder.map_io("result", "null");
}