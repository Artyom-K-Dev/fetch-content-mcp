# Fetch Content MCP Server

A Model Context Protocol (MCP) server that fetches URL content and converts it to Markdown.

## Features

- **Fetch & Convert**: Downloads HTML content from a URL and converts it to readable Markdown
- **Clean Output**: Strips unnecessary HTML tags and formatting

## Usage

### Docker (Recommended)

```bash
docker run -i --rm ghcr.io/artyom-k-dev/fetch-content-mcp:latest
```

### Local Development

1. Install dependencies:
   ```bash
   cargo build --release
   ```

2. Run the server:
   ```bash
   cargo run --release
   ```

## Tools Available

- `fetch`: Fetch a URL and convert its content to Markdown.
  - Arguments:
    - `url`: The URL to fetch

## Configuration

No environment variables are required for this server.
