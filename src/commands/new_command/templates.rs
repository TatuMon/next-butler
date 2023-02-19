fn page_template(page_name: &String) {
    format!("\
    export default function ${}() {
        return (
            <div></div>
        )
    }",
    page_name);
}