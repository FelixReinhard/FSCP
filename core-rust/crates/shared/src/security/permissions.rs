// TODO
pub enum Permissions {
    Admin,
    User,
    All,
    Group(String),
    Multiple(Vec<Permissions>),
}
