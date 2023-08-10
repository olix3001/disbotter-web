const id = "_builtin_number_constant";
const title = "Constant number";
const description = "Constant value.";
const category = "Math";

const pure;

const inputs = #{
    number: #{
        name: "number",
        type: "number",
    }
};
const outputs = #{
    result: #{
        name: "value",
        type: "number",
    }
};

fn action(builder) {
    out result = (inv number);
}