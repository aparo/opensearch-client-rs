//! # OpenSearch CLI Tool
//!
//! A powerful command-line interface for managing OpenSearch clusters, indices, and data operations.
//!
//! ## Features
//!
//! - **Cluster Metadata Management**: Dump, restore, and fix cluster metadata
//! - **Index Operations**: List, dump, restore, and copy indices
//! - **Data Migration**: Efficient bulk data operations between clusters
//! - **Remote Cluster Support**: Copy data between different OpenSearch clusters
//! - **Configuration Management**: Environment variable and CLI flag support
//!
//! ## Usage
//!
//! ```bash
//! # Set up environment
//! export OPENSEARCH_URL="http://localhost:9200"
//! export OPENSEARCH_USER="admin"
//! export OPENSEARCH_PASSWORD="admin"
//!
//! # List all indices
//! opensearch-cli list-indices
//!
//! # Dump cluster metadata
//! opensearch-cli dump-metadata --output ./backup
//!
//! # Copy index to remote cluster
//! opensearch-cli copy-index my_index --remote --target-index backup_index
//! ```
//!
//! For detailed usage instructions, see the README.md file.

use std::{path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use tracing::info;
use url::Url;
mod dumper;
mod restorer;

/// OpenSearch CLI - A powerful tool for managing OpenSearch clusters
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// OpenSearch server URL
    #[clap(
        short,
        long,
        env = "OPENSEARCH_URL",
        default_value = "http://localhost:9200"
    )]
    server: String,

    /// OpenSearch user to be used for the connection
    #[clap(short, long, env = "OPENSEARCH_USER", default_value = "admin")]
    user: String,

    /// OpenSearch password to be used for the connection
    #[clap(short, long, env = "OPENSEARCH_PASSWORD", default_value = "admin")]
    password: String,

    /// Remote OpenSearch server URL for copy operations
    #[clap(
        long,
        env = "OPENSEARCH_REMOTE_URL",
        default_value = "http://localhost:9200"
    )]
    remote_server: String,

    /// Remote OpenSearch user for copy operations
    #[clap(long, env = "OPENSEARCH_REMOTE_USER", default_value = "admin")]
    remote_user: String,

    /// Remote OpenSearch password for copy operations
    #[clap(long, env = "OPENSEARCH_REMOTE_PASSWORD", default_value = "admin")]
    remote_password: String,

    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Dump Cluster metadata
    DumpMetadata {
        /// Dump ingest pipelines
        #[clap(long, default_value = "true")]
        ingest_pipelines: bool,

        /// Dump index templates
        #[clap(long, default_value = "true")]
        index_templates: bool,

        /// Dump index components
        #[clap(long, default_value = "true")]
        index_components: bool,

        /// Sets the output path
        #[clap(value_name = "FILE", default_value = "output")]
        output: PathBuf,
    },
    /// Restore Cluster metadata
    RestoreMetadata {
        /// Restore ingest pipelines
        #[clap(long, default_value = "true")]
        ingest_pipelines: bool,

        /// Restore index templates
        #[clap(long, default_value = "true")]
        index_templates: bool,

        /// Restore index components
        #[clap(long, default_value = "true")]
        index_components: bool,

        /// Sets the input path
        #[clap(value_name = "FILE", default_value = "output")]
        input: PathBuf,
    },
    /// Fix Metadata
    /// This command will fix the metadata of the cluster
    FixMetadata {
        /// Fix ingest pipelines
        #[clap(long, default_value = "true")]
        ingest_pipelines: bool,

        /// Fix index templates
        #[clap(long, default_value = "true")]
        index_templates: bool,

        /// Fix index components
        #[clap(long, default_value = "true")]
        index_components: bool,

        /// Sets the input path
        #[clap(value_name = "FILE", default_value = "output")]
        input: PathBuf,
    },
    ///  ListIndices
    /// This command will list all the indices in the cluster
    ListIndices {
        /// Show hidden indices (usually that starts with a dot)
        #[clap(long, default_value = "false")]
        show_hidden: bool,
        /// Use the contains the pattern to filter the indices
        #[clap(long)]
        contains: Option<String>,
    },
    /// Dump index data and mappings
    /// This command will dump the data and mappings of the indices
    Dump {
        /// Size of the bulk read operation
        #[clap(long, default_value = "500")]
        read_bulk: u64,

        /// Set the index,indices to dump (comma-separated or wildcard patterns)
        #[clap(value_name = "indices")]
        indices: String,

        /// Sets the output path
        #[clap(value_name = "FILE", default_value = "output")]
        output: PathBuf,
    },
    /// Restore index data and mappings
    /// This command will restore the data and mappings of the indices
    Restore {
        /// Skip the data
        #[clap(long, default_value = "false")]
        skip_data: bool,
        /// Skip the mappings
        #[clap(long, default_value = "false")]
        skip_mappings: bool,
        /// Size of the bulk
        #[clap(long, default_value = "1000")]
        bulk_size: u32,
        /// Mode to restore the index
        #[clap(
    long,
    require_equals = true,
    value_name = "MODE",
    // num_args = 0..=1,
    default_value_t = restorer::RestoreMode::Index,
    default_missing_value = "index",
    value_enum)]
        mode: restorer::RestoreMode,
        /// Set the index files to restore
        #[clap(long)]
        index: Option<String>,
        /// Set the index name to restore
        #[clap(long)]
        rename_index: Option<String>,
        /// Sets the input path
        #[clap(value_name = "FILE", default_value = "output")]
        input: PathBuf,
    },
    ///  RemoteCopyIndex
    /// This command will remotely copy an index from one cluster to another
    CopyIndex {
        /// Use a remote cluster
        #[clap(long, default_value = "false")]
        remote: bool,
        /// Copy the mapping
        #[clap(long, default_value = "false")]
        copy_mapping: bool,
        /// Deleting existsing index before copying
        #[clap(long, default_value = "false")]
        delete_existing: bool,
        /// The target index name
        #[clap(long, default_value = "1000")]
        size: u64,
        /// The target index name
        #[clap(long)]
        target_index: Option<String>,
        /// The source index name
        index: String,
    },
}

fn create_remote_client(cli: &Args) -> anyhow::Result<Arc<opensearch_client::OsClient>> {
    let url = Url::parse(cli.remote_server.as_str())?;
    let mut builder = opensearch_client::ConfigurationBuilder::new().base_url(url);
    builder = builder.basic_auth(cli.remote_user.clone(), cli.remote_password.clone());
    Ok(Arc::new(builder.build()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let cli = Args::parse();

    // we setup logging
    let tracing_level = match cli.verbose {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        // enable everything
        .with_max_level(tracing_level)
        // sets this to be the default, global subscriber for this application.
        .init();

    let client = Arc::new({
        let url = Url::parse(cli.server.clone().as_str())?;
        let mut builder = opensearch_client::ConfigurationBuilder::new().base_url(url.clone());
        builder = builder.basic_auth(cli.user.clone(), cli.password.clone());
        builder.build()
    });

    match &cli.command {
        Commands::DumpMetadata {
            ingest_pipelines,
            index_templates,
            index_components,
            output,
        } => {
            info!("Dumping metadata");
            info!("Output: {:?}", output);
            if *ingest_pipelines {
                info!("Ingest pipelines: {}", ingest_pipelines);
                client.tools().dump_pipelines(output.clone()).await?;
            }
            if *index_templates {
                info!("Index Templates: {}", index_templates);
                client.tools().dump_index_templates(output.clone()).await?;
            }
            if *index_components {
                info!("Index Components: {}", index_components);
                client.tools().dump_index_components(output.clone()).await?;
            }
        }
        Commands::RestoreMetadata {
            ingest_pipelines,
            index_templates,
            index_components,
            input,
        } => {
            info!("Restoring metadata");
            info!("Input: {:?}", input);
            if *ingest_pipelines {
                info!("Ingest pipelines: {}", ingest_pipelines);
                client.tools().restore_pipelines(input.clone()).await?;
            }
            if *index_templates {
                info!("Index Templates: {}", index_templates);
                client
                    .tools()
                    .restore_index_templates(input.clone())
                    .await?;
            }
            if *index_components {
                info!("Index Components: {}", index_components);
                client
                    .tools()
                    .restore_index_components(input.clone())
                    .await?;
            }
        }
        Commands::FixMetadata {
            ingest_pipelines,
            index_templates,
            index_components,
            input,
        } => {
            info!("Fixing metadata");
            info!("Input: {:?}", input);
            if *ingest_pipelines {
                info!("Ingest pipelines: {}", ingest_pipelines);
                client.tools().fix_pipelines(input.clone()).await?;
            }
            if *index_templates {
                info!("Index Templates: {}", index_templates);
                client.tools().fix_index_templates(input.clone()).await?;
            }
            if *index_components {
                info!("Index Components: {}", index_components);
                client.tools().fix_components(input.clone()).await?;
            }
        }
        Commands::ListIndices {
            show_hidden,
            contains,
        } => {
            info!("Listing indices");
            let indices = client.indices().list_indices().await?;
            for index in indices {
                if show_hidden == &false && index.starts_with('.') {
                    continue;
                }
                if let Some(contains) = contains {
                    if index.contains(contains) {
                        println!("{}", index);
                    }
                } else {
                    println!("{}", index);
                }
            }
        }
        Commands::Dump {
            read_bulk,
            indices,
            output,
        } => {
            info!("Dumping index");
            info!("Output: {:?}", output);

            let dumper = dumper::Dumper {
                client,
                output: output.clone(),
                indices: indices.clone(),
                read_bulk: *read_bulk,
            };

            dumper.dump().await?;
        }
        Commands::Restore {
            input,
            skip_data,
            skip_mappings,
            bulk_size,
            mode,
            index,
            rename_index,
        } => {
            info!("Restoring index");
            info!("Input: {:?}", input);
            let restorer = restorer::Restorer {
                client,
                input: input.clone(),
                skip_data: *skip_data,
                skip_mapping: *skip_mappings,
                bulk_size: *bulk_size,
                mode: *mode,
                index: index.clone(),
                rename_index: rename_index.clone(),
            };

            restorer.restore().await?;
        }
        Commands::CopyIndex {
            remote,
            copy_mapping,
            delete_existing,
            size,
            target_index,
            index,
        } => {
            info!("Copying index");
            info!("Remote: {}", remote);
            info!("Copy mapping: {}", copy_mapping);
            info!("Delete existing: {}", delete_existing);
            info!("Size: {}", size);
            info!("Target index: {:?}", target_index);
            info!("Source index: {}", index);

            // Here you would implement the logic to copy the index
            // This is a placeholder for the actual implementation
            if *remote {
                // Implement remote copy logic
                let remote_client = create_remote_client(&cli)?;
                opensearch_client::tools::copy_index_remotely()
                    .source_client(client)
                    .target_client(remote_client)
                    .copy_mappings(*copy_mapping)
                    .delete_existing(*delete_existing)
                    .size(*size)
                    .source_index(index)
                    .maybe_target_index(target_index.clone())
                    .call()
                    .await?;

                //   index.clone(),
                //   target_index.clone(),
                //   *copy_mapping,
                //   *delete_existing,
                //   *size,
                // )
            } else {
                // Implement local copy logic
                unimplemented!();
            }
        }
    }

    Ok(())
}
