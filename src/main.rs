mod ntval;
use ntval::ntval;

fn main() {
    let var_bool = true;
    let var_float = 3.1415926;
    let var_integer = 47;
    let var_string = String::from("a named String");
    let var_array = [0,1,0,1];
    let var_vector = Vec::from([1,2,3,4,5]);
    ntval!(
    'r'
    ,"unnamed str"
    ,var_bool
    ,var_float
    ,var_integer
    ,var_string
    ,var_array
    ,var_vector
);
}
