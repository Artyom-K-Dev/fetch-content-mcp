# Fetch URL MCP Server

A Model Context Protocol (MCP) server written in Rust that allows AI agents to fetch URL content and convert it to Markdown.

## Features

*   **URL Fetching**: Retrieves content from a given URL.
*   **HTML to Markdown**: Automatically converts HTML content to Markdown for easier consumption by LLMs.

## Installation & Configuration

### Prerequisites
*   Rust toolchain installed.

### Build
```bash
cargo build --release
```

### Configuration (Cursor)
Add the server to your `.cursor/mcp.json` (project specific) or your global MCP settings.

**Copy-pasteable entry:**

```json
{
  "mcpServers": {
    "fetch-url-mcp": {
      "command": "/workspaces/MachineLearning/FetchUrlMCP/target/release/fetch_url_mcp",
      "args": []
    }
  }
}
```

## Usage

The server exposes a single tool: `fetch`.

### Tool: `fetch`

**Arguments:**
*   `url`: String. The URL to fetch.

**Example:**
*   URL: `https://example.com`
*   Input: `{"url": "https://example.com"}`

## Development

1.  **Run locally**:
    ```bash
    cargo run
    ```

## License

MIT
