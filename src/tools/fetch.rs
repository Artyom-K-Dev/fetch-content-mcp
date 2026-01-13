use rmcp::{tool, tool_router, handler::server::tool::{ToolRouter, Parameters}, ErrorData};
use rmcp::schemars::JsonSchema;
use rmcp::serde::Deserialize;
use std::future::Future;

#[derive(Clone)]
pub struct FetchTool {
    pub tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct FetchArgs {
    url: String,
}

#[tool_router]
impl FetchTool {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Fetch a URL and convert its content to Markdown")]
    async fn fetch(&self, params: Parameters<FetchArgs>) -> Result<String, ErrorData> {
        let url = params.0.url;
        
        let response = reqwest::get(&url).await.map_err(|e| ErrorData { code: rmcp::model::ErrorCode::INTERNAL_ERROR, message: e.to_string().into(), data: None })?;
        
        if !response.status().is_success() {
            return Err(ErrorData { code: rmcp::model::ErrorCode::INTERNAL_ERROR, message: format!("Request failed: {}", response.status()).into(), data: None });
        }
        
        let html = response.text().await.map_err(|e| ErrorData { code: rmcp::model::ErrorCode::INTERNAL_ERROR, message: e.to_string().into(), data: None })?;
        let markdown = html2md::parse_html(&html);

        Ok(markdown)
    }
}
