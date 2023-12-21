pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COMPONENTS_DEFAULT_FOLDER: &str = "components/";

pub const NEXT_BUTLER_DIR: &str = "nextbutler/";
pub const CONFIG_FILE_NAME: &str = "nextbutler.json";

pub const DEFAULT_PAGE_TEMPLATE: &str = r#"export default function NNNN() {
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

pub const DEFAULT_COMPONENT_TEMPLATE: &str = r#"export default function NNNN() {
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
