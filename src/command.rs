pub struct Command<'a> {
    pub name: &'a str,
    pub handler: fn(Vec<String>) -> Vec<String>,
}
