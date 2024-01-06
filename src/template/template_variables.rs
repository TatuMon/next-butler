/// Struct that holds all the template variables that should be used
/// when formatting the content
/// 
/// For now it only has the name, but I think it may have more in the future
pub struct TemplateVariables<'a> {
    // Not Option, this has to be set
    pub name: &'a str,
}