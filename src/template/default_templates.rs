pub const DEFAULT_PAGE_TEMPLATE: &str = r#"export default function {{ name }}() {
    return (
        <div>
            <h1>\o/</h1>
        </div> 
    )
}
"#;

pub const DEFAULT_API_PAGE_TEMPLATE: &str = r#"// Next.js API route support: https://nextjs.org/docs/api-routes/introduction

export default function handler(req, res) {
  res.status(200).json({ name: 'John Doe' })
}"#;

pub const DEFAULT_COMPONENT_TEMPLATE: &str = r#"export default function {{ name }}() {
    return (
        <div>
            <h1>Hi! \o/</h1>
        </div> 
    )
}
"#;

pub const DEFAULT_STYLESHEET_TEMPLATE: &str = r#"html, body {
    background: red;
}
"#;
