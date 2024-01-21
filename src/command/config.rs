use clap::{Args, Subcommand};
use crate::models::Config as ModelsConfig;
use crate::models::common::CONFIG;

#[derive(Args)]
pub struct Config {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Create config file
    New(ConfigArgs ),
    /// Set config key-value
    Set(ConfigArgs),
    /// Show config
    Show {},
}

#[derive(Args)]
pub struct ConfigArgs {
    /// Space ID for filter
    #[arg(short, long)]
    space_id: Option<u32>,
    /// Board ID for filter
    #[arg(short, long)]
    board_id: Option<u32>,
    /// Exclude board ids filter, comma separated.
    #[arg(long)]
    exclude_board_ids: Option<String>,
    /// Exclude lane ids filter, comma separated.
    #[arg(long)]
    exclude_lane_ids: Option<String>,
    /// Exclude columns ids filter, comma separated.
    #[arg(long)]
    exclude_column_ids: Option<String>,
}

impl Config {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = match &self.command {
            ConfigCommands::New(args) => {
                Config::set_config_values(args);
                String::new()
            }
            ConfigCommands::Set(args) => {
                Config::set_config_values(args);
                String::new()
            }
            ConfigCommands::Show{} => {
                let config = ModelsConfig::load()?;
                let result = serde_yaml::to_string(&config)?;
                result

            }
        };
        Ok(result)
    }

    fn set_config_values(args: &ConfigArgs) {
        let mut config = CONFIG.lock().unwrap();
        if let Some(space_id) = args.space_id {
            config.set_space_id(space_id)
        }
        if let Some(board_id) = args.board_id {
            config.set_board_id(board_id)
        }
        if let Some(exclude_board_ids) = &args.exclude_board_ids {
            config.set_exclude_board_ids(exclude_board_ids.to_string());
        }
        if let Some(exclude_lane_ids) = &args.exclude_lane_ids {
            config.set_exclude_lane_ids(exclude_lane_ids.to_string());
        }
        if let Some(exclude_column_ids) = &args.exclude_column_ids {
            config.set_exclude_column_ids(exclude_column_ids.to_string());
        }
        let _ = config.save();
    } 
}
