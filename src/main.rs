//! ace-ctx - MCP server for codebase indexing and semantic search

use ace_tool::config::{Config, ConfigOptions};
use ace_tool::enhancer::prompt_enhancer::{get_enhancer_endpoint, PromptEnhancer};
use ace_tool::index::IndexManager;
use ace_tool::mcp::{McpServer, TransportMode};
use ace_tool::search_filter::SearchFilterOptions;
use ace_tool::service::get_third_party_config;
use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use std::env;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(ValueEnum, Debug, Copy, Clone)]
enum TransportArg {
    Auto,
    Lsp,
    Line,
}

#[derive(Parser, Debug)]
#[command(name = "ace-ctx")]
#[command(about = "CLI tool and MCP server for codebase indexing and semantic search")]
#[command(version)]
struct Args {
    /// API base URL for the indexing service
    #[arg(long, env = "ACE_BASE_URL")]
    base_url: Option<String>,

    /// Authentication token
    #[arg(long, env = "ACE_TOKEN")]
    token: Option<String>,

    /// Transport framing: auto, lsp, line
    #[arg(long, value_enum, default_value = "auto")]
    transport: TransportArg,

    /// Maximum lines per blob (default: 800)
    #[arg(long)]
    max_lines_per_blob: Option<usize>,

    /// Upload timeout in seconds (default: adaptive)
    #[arg(long)]
    upload_timeout: Option<u64>,

    /// Upload concurrency (default: adaptive)
    #[arg(long)]
    upload_concurrency: Option<usize>,

    /// Retrieval timeout in seconds (default: 60)
    #[arg(long)]
    retrieval_timeout: Option<u64>,

    /// Disable adaptive strategy
    #[arg(long, default_value = "false")]
    no_adaptive: bool,

    /// Disable web browser interaction for enhance_prompt, return API result directly
    #[arg(long, default_value = "false")]
    no_webbrowser_enhance_prompt: bool,

    /// Force using xdg-open instead of explorer.exe in WSL environment
    /// Use this if WSL localhost forwarding is disabled and browser can't reach the WSL server
    #[arg(long, default_value = "false")]
    force_xdg_open: bool,

    /// Bind address and port for the enhance_prompt Web UI server (e.g., "127.0.0.1:8754", "0.0.0.0:3456")
    /// If not specified, automatically selects an available port on 127.0.0.1.
    /// WARNING: Binding to 0.0.0.0 or a non-loopback address exposes the unauthenticated
    /// Web UI to the network. Only use this in trusted environments.
    #[arg(long)]
    webui_addr: Option<String>,

    /// Index-only mode: index current directory and exit (no MCP server)
    #[arg(long, default_value = "false")]
    index_only: bool,

    /// Enhance a prompt and output the result to stdout, then exit
    #[arg(long)]
    enhance_prompt: Option<String>,

    /// Search the codebase using a natural language query and exit
    #[arg(long)]
    search: Option<String>,

    /// Include document files (md, txt, etc.) in search results
    #[arg(long, default_value = "false")]
    include_document_files: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for stderr (MCP uses stdout for protocol)
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Search mode: execute semantic search and output to stdout
    if let Some(ref query) = args.search {
        info!("Search mode: executing semantic search");
        let project_root = env::current_dir()?;
        info!("Project root: {:?}", project_root);

        // For search, base_url and token are required
        let base_url = args
            .base_url
            .clone()
            .ok_or_else(|| anyhow!("--base-url is required for search"))?;
        let token = args
            .token
            .clone()
            .ok_or_else(|| anyhow!("--token is required for search"))?;

        let config = Config::new(
            base_url,
            token,
            ConfigOptions {
                max_lines_per_blob: args.max_lines_per_blob,
                upload_timeout: args.upload_timeout,
                upload_concurrency: args.upload_concurrency,
                retrieval_timeout: args.retrieval_timeout,
                no_adaptive: args.no_adaptive,
                no_webbrowser_enhance_prompt: args.no_webbrowser_enhance_prompt,
                force_xdg_open: args.force_xdg_open,
                webui_addr: args.webui_addr.clone(),
            },
        )?;

        let manager = IndexManager::new(config, project_root)?;
        let mut filters =
            SearchFilterOptions::from_args(&ace_tool::tools::search_context::SearchContextArgs {
                exclude_document_files: Some(!args.include_document_files),
                ..Default::default()
            });
        filters.compile_globs()?;

        let result = manager.search_context(query, &filters).await?;
        println!("{}", result);
        return Ok(());
    }

    // Enhance-prompt mode: enhance the prompt and output to stdout
    if let Some(ref prompt) = args.enhance_prompt {
        info!("Enhance-prompt mode: enhancing prompt");
        let project_root = env::current_dir()?;
        info!("Project root: {:?}", project_root);

        // Check if using third-party endpoint (claude/openai/gemini)
        let endpoint = get_enhancer_endpoint();
        let config = if endpoint.is_third_party() {
            // For third-party endpoints, base_url and token are not required from CLI
            // They will be read from environment variables
            // Validate early that required environment variables are set
            let _ = get_third_party_config(endpoint)
                .map_err(|e| anyhow!("Third-party endpoint configuration error: {}", e))?;
            info!("Using third-party endpoint: {}", endpoint);
            match (args.base_url.clone(), args.token.clone()) {
                (Some(base_url), Some(token)) => {
                    info!("Using CLI base_url/token to enable ACE search features");
                    Config::new(
                        base_url,
                        token,
                        ConfigOptions {
                            max_lines_per_blob: args.max_lines_per_blob,
                            upload_timeout: args.upload_timeout,
                            upload_concurrency: args.upload_concurrency,
                            retrieval_timeout: args.retrieval_timeout,
                            no_adaptive: args.no_adaptive,
                            no_webbrowser_enhance_prompt: args.no_webbrowser_enhance_prompt,
                            force_xdg_open: args.force_xdg_open,
                            webui_addr: args.webui_addr.clone(),
                        },
                    )?
                }
                (None, None) => Config::new_for_third_party_enhancer(),
                _ => {
                    return Err(anyhow!(
                        "--base-url and --token must be provided together in third-party enhance-prompt mode"
                    ));
                }
            }
        } else {
            // For new/old endpoints, base_url and token are required
            let base_url = args
                .base_url
                .clone()
                .ok_or_else(|| anyhow!("--base-url is required for '{}' endpoint", endpoint))?;
            let token = args
                .token
                .clone()
                .ok_or_else(|| anyhow!("--token is required for '{}' endpoint", endpoint))?;
            Config::new(
                base_url,
                token,
                ConfigOptions {
                    max_lines_per_blob: args.max_lines_per_blob,
                    upload_timeout: args.upload_timeout,
                    upload_concurrency: args.upload_concurrency,
                    retrieval_timeout: args.retrieval_timeout,
                    no_adaptive: args.no_adaptive,
                    no_webbrowser_enhance_prompt: args.no_webbrowser_enhance_prompt,
                    force_xdg_open: args.force_xdg_open,
                    webui_addr: args.webui_addr.clone(),
                },
            )?
        };

        let enhancer = PromptEnhancer::new(config.clone())?;
        let enhanced = enhancer
            .enhance_simple(prompt, "", Some(&project_root))
            .await?;

        // Output enhanced prompt to stdout
        println!("{}", enhanced);
        return Ok(());
    }

    // For non-enhance-prompt modes, base_url and token are always required
    let base_url = args
        .base_url
        .ok_or_else(|| anyhow!("--base-url is required"))?;
    let token = args.token.ok_or_else(|| anyhow!("--token is required"))?;

    // Initialize configuration
    let config = Config::new(
        base_url,
        token,
        ConfigOptions {
            max_lines_per_blob: args.max_lines_per_blob,
            upload_timeout: args.upload_timeout,
            upload_concurrency: args.upload_concurrency,
            retrieval_timeout: args.retrieval_timeout,
            no_adaptive: args.no_adaptive,
            no_webbrowser_enhance_prompt: args.no_webbrowser_enhance_prompt,
            force_xdg_open: args.force_xdg_open,
            webui_addr: args.webui_addr,
        },
    )?;

    // Index-only mode: index current directory and exit
    if args.index_only {
        info!("Index-only mode: indexing current directory");
        let project_root = env::current_dir()?;
        info!("Project root: {:?}", project_root);

        let manager = IndexManager::new(config, project_root)?;
        let result = manager.index_project().await;

        match result.status.as_str() {
            "success" => {
                info!("Indexing completed successfully: {}", result.message);
                if let Some(stats) = result.stats {
                    info!(
                        "Stats: {} total blobs, {} existing, {} new",
                        stats.total_blobs, stats.existing_blobs, stats.new_blobs
                    );
                }
                return Ok(());
            }
            "partial" => {
                warn!("Indexing completed with warnings: {}", result.message);
                if let Some(stats) = result.stats {
                    if let Some(failed_batches) = stats.failed_batches {
                        warn!(
                            "Stats: {} total blobs, {} existing, {} new, {} failed batches",
                            stats.total_blobs,
                            stats.existing_blobs,
                            stats.new_blobs,
                            failed_batches
                        );
                    } else {
                        warn!(
                            "Stats: {} total blobs, {} existing, {} new",
                            stats.total_blobs, stats.existing_blobs, stats.new_blobs
                        );
                    }
                }
                std::process::exit(2);
            }
            _ => {
                return Err(anyhow::anyhow!("Indexing failed: {}", result.message));
            }
        }
    }

    info!("Starting ace-ctx MCP server");

    let transport_mode = match args.transport {
        TransportArg::Auto => None,
        TransportArg::Lsp => Some(TransportMode::Lsp),
        TransportArg::Line => Some(TransportMode::Line),
    };

    // Create and run MCP server
    let server = McpServer::new(config, transport_mode);

    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_args_env_fallback() {
        // Clear variables to ensure clean test state
        env::remove_var("ACE_BASE_URL");
        env::remove_var("ACE_TOKEN");

        // Test without environment variables and without CLI arguments
        let args = Args::try_parse_from(["ace-ctx", "--search", "test"]).unwrap();
        assert_eq!(args.base_url, None);
        assert_eq!(args.token, None);

        // Test with environment variables only
        env::set_var("ACE_BASE_URL", "https://env.example.com");
        env::set_var("ACE_TOKEN", "env-token");
        let args = Args::try_parse_from(["ace-ctx", "--search", "test"]).unwrap();
        assert_eq!(args.base_url, Some("https://env.example.com".to_string()));
        assert_eq!(args.token, Some("env-token".to_string()));

        // Test environment variables overridden by CLI arguments
        let args = Args::try_parse_from([
            "ace-ctx",
            "--search",
            "test",
            "--base-url",
            "https://cli.example.com",
            "--token",
            "cli-token",
        ])
        .unwrap();
        assert_eq!(args.base_url, Some("https://cli.example.com".to_string()));
        assert_eq!(args.token, Some("cli-token".to_string()));

        // Clean up
        env::remove_var("ACE_BASE_URL");
        env::remove_var("ACE_TOKEN");
    }

    #[test]
    fn test_cli_include_document_files_parsing() {
        // Default should be false
        let args = Args::try_parse_from(["ace-ctx", "--search", "test"]).unwrap();
        assert!(!args.include_document_files);

        // Explicit true
        let args =
            Args::try_parse_from(["ace-ctx", "--search", "test", "--include-document-files"])
                .unwrap();
        assert!(args.include_document_files);
    }
}
