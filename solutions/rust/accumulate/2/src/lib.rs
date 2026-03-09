/// What should the type of function be?
pub fn map<Input, Output, Func>(input: Vec<Input>, mut function: Func) -> Vec<Output>
where
    Func: FnMut(Input) -> Output,
{
    let mut res = Vec::with_capacity(input.len());
    for i in input {
        res.push(function(i));
    }
    res
}
