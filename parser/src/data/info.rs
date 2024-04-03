#[derive(Debug, Clone, Default)]
pub struct Info {
    pub is_default: bool,
    pub config_name: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub version: Option<String>,
    pub base: Option<String>,
}
