use std::path::PathBuf;

use tracing::info;
use clap::{Parser, Subcommand};
use url::Url;
mod dumper;
mod restorer;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Turn debugging information on
  #[clap(short, long, action = clap::ArgAction::Count)]
  verbose: u8,

  /// Opensearch server url
  #[clap(short, long, env = "OPENSEARCH_URL", default_value = "http://localhost:9200")]
  server: String,

  /// Opensearch user to be used for the connection
  #[clap(short, long, env = "OPENSEARCH_USER", default_value = "admin")]
  user: String,

  /// Opensearch password to be used for the connection
  #[clap(short, long, env = "OPENSEARCH_PASSWORD", default_value = "admin")]
  password: String,

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
    /// Compress the data
    #[clap(long, default_value = "true")]
    compress: bool,
    /// Size of the bulk read
    #[clap(long, default_value = "500")]
    read_bulk: u64,
    /// Split file after this number of records (0: no split)
    #[clap(long, default_value = "0")]
    max_record_for_file: u32,
    /// Set the index,indices to dump
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

  let client = {
    let url = Url::parse(cli.server.clone().as_str())?;
    let mut builder = opensearch::OsClientBuilder::new().base_url(url.clone());
    builder = builder.basic_auth(url, cli.user.clone(), Some(cli.password.clone()));
    builder.build()
  };

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
        client.tools().restore_index_templates(input.clone()).await?;
      }
      if *index_components {
        info!("Index Components: {}", index_components);
        client.tools().restore_index_components(input.clone()).await?;
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
    Commands::ListIndices { show_hidden, contains } => {
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
      compress,
      read_bulk,
      max_record_for_file,
      indices,
      output,
    } => {
      info!("Dumping index");
      info!("Output: {:?}", output);
      let dumper = dumper::Dumper {
        client: &client,
        compress: *compress,
        output: output.clone(),
        indices: indices.clone(),
        read_bulk: *read_bulk,
        max_record_for_file: *max_record_for_file,
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
        client: &client,
        input: input.clone(),
        skip_data: *skip_data,
        skip_mapping: *skip_mappings,
        bulk_size: *bulk_size,
        mode: mode.clone(),
        index: index.clone(),
        rename_index: rename_index.clone(),
      };

      restorer.restore().await?;
    }
  }

  Ok(())
}
