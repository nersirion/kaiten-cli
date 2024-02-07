use crate::api::ApiClient;
use crate::models::common::{CONFIG, INFO};
use crate::models::{Card as ModelsCard, User};
use clap::{Args, Subcommand};
use tabled::{
    settings::{
        measurement::Percent,
        object::{Columns, Rows},
        Modify, Style, Width,
    },
    Table,
};

#[derive(Args)]
pub struct Card {
    #[command(subcommand)]
    pub command: CardCommands,
}

#[derive(Subcommand)]
pub enum CardCommands {
    /// print all cards for user
    Ls(Ls),
    /// get card info
    Get {
        card_id: u32,
    },
    /// edit card
    Edit {
        card_id: u32,
        /// Set new desctiption for card
        #[arg(long,short='d')]
        description: Option<String>,
        /// Add description for card
        #[arg(long,short='D')]
        add_description: Option<String>,
        #[arg(long,short='t')]
        title: Option<String>,
        #[arg(long,short='t')]
        column_id: Option<u32>,
        #[arg(long,short='l')]
        lane_id: Option<u32>,
    },
    /// create new card
    New {},
    /// move card to next column
    Mv {
        card_id: u32,
        /// Column id to move
        #[arg(long, short)]
        column_id: u32,
        /// Lane id to move
        #[arg(long, short)]
        lane_id: u32,
        add_responsible: Option<String>,
    },
    Parents {
        card_id: u32,
    },
    Childrens {
        card_id: u32,
    },
}

#[derive(Args)]
pub struct Ls {
    /// Filter by condition: 1 - on board, 2 - archived.
    #[arg(short, long, default_value = "1")]
    condition: u8,
    /// Search by states filter, comma separated. 1-queued, 2-inProgress, 3-done.
    #[arg(short = 'S', long, default_value = "1,2")]
    states: String,
    #[arg(short, long)]
    properties_id: Option<u32>,
    #[arg(short = 'P', long)]
    properties_value_id: Option<u32>,
    /// Created before search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    created_before: Option<String>,
    /// Created after search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    created_after: Option<String>,
    /// Updated before search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    updated_before: Option<String>,
    /// Updated after search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    updated_after: Option<String>,
    /// First moved in progress after date search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    first_moved_in_progress_after: Option<String>,
    /// First moved in progress before date search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    first_moved_in_progress_before: Option<String>,
    /// Last time moved to done column after date search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    last_moved_to_done_at_after: Option<String>,
    /// Last time moved to done column before date search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    last_moved_to_done_at_before: Option<String>,
    /// Due date is set to date after search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    due_date_after: Option<String>,
    /// Due date is set to date before search filter (format: ISO 8601).
    #[arg(long, value_parser = validate_iso8601)]
    due_date_before: Option<String>,
    /// Card contains text search filter.
    #[arg(long, short)]
    query: Option<String>,
    /// Tag search filter.
    #[arg(long, short)]
    tag: Option<String>,
    /// Search by tag ids filter, comma separated.
    #[arg(long)]
    tag_ids: Option<String>,
    /// Search by type ids filter, comma separated.
    #[arg(long)]
    type_ids: Option<String>,
    /// Exclude board ids filter, comma separated.
    #[arg(long)]
    exclude_board_ids: Option<String>,
    /// Exclude lane ids filter, comma separated.
    #[arg(long)]
    exclude_lane_ids: Option<String>,
    /// Exclude columns ids filter, comma separated.
    #[arg(long)]
    exclude_column_ids: Option<String>,
    /// Search by column ids filter, comma separated.
    #[arg(long)]
    column_ids: Option<String>,
    /// Search by member ids filter, comma separated.
    #[arg(long)]
    member_ids: Option<String>,
    /// Search by owner ids filter, comma separated.
    #[arg(long)]
    owner_ids: Option<String>,
    /// Search by responsible ids filter, comma separated.
    #[arg(long)]
    responsible_ids: Option<String>,
    /// Maximum amount of cards in response.
    #[arg(long)]
    limit: Option<u32>,
    /// Number of records to skip.
    #[arg(long)]
    offset: Option<u32>,
    /// Order by space id.
    #[arg(long)]
    order_space_id: Option<u32>,
    /// Filter by column id.
    #[arg(long)]
    column_id: Option<u32>,
    /// Filter by lane id.
    #[arg(long)]
    lane_id: Option<u32>,
    /// Filter by type id.
    #[arg(long)]
    type_id: Option<u32>,
    /// Filter by responsible id.
    #[arg(long)]
    responsible_id: Option<u32>,
    /// Filter by owner id.
    #[arg(long)]
    owner_id: Option<u32>,
    /// Archived flag.
    #[arg(long)]
    archived: Option<bool>,
    /// ASAP marker.
    #[arg(long)]
    asap: Option<bool>,
    /// Filter by completed on time.
    #[arg(long)]
    overdue: Option<bool>,
    /// Filter by done on time.
    #[arg(long)]
    done_on_time: Option<bool>,
    /// Filter by due date is set.
    #[arg(long)]
    with_due_date: Option<bool>,
}

impl Ls {
    pub fn get_url(&self) -> String {
        let mut url = String::from("cards?");
        let config = CONFIG.lock().unwrap();
        if let Some(board_id) = config.get_board_id() {
            url.push_str(&format!("board_id={}&", board_id));
        }
        url.push_str(&format!("condition={}&", self.condition));
        if !self.states.is_empty() {
            url.push_str(&format!("states={}&", self.states));
        }
        if let Some(created_before) = &self.created_before {
            url.push_str(&format!("created_before={}&", created_before));
        }
        if let Some(created_after) = &self.created_after {
            url.push_str(&format!("created_after={}&", created_after));
        }
        if let Some(updated_before) = &self.updated_before {
            url.push_str(&format!("updated_before={}&", updated_before));
        }
        if let Some(updated_after) = &self.updated_after {
            url.push_str(&format!("updated_after={}&", updated_after));
        }
        if let Some(first_moved_in_progress_after) = &self.first_moved_in_progress_after {
            url.push_str(&format!(
                "first_moved_in_progress_after={}&",
                first_moved_in_progress_after
            ));
        }
        if let Some(first_moved_in_progress_before) = &self.first_moved_in_progress_before {
            url.push_str(&format!(
                "first_moved_in_progress_before={}&",
                first_moved_in_progress_before
            ));
        }
        if let Some(last_moved_to_done_at_after) = &self.last_moved_to_done_at_after {
            url.push_str(&format!(
                "last_moved_to_done_at_after={}&",
                last_moved_to_done_at_after
            ));
        }
        if let Some(last_moved_to_done_at_before) = &self.last_moved_to_done_at_before {
            url.push_str(&format!(
                "last_moved_to_done_at_before={}&",
                last_moved_to_done_at_before
            ));
        }
        if let Some(due_date_after) = &self.due_date_after {
            url.push_str(&format!("due_date_after={}&", due_date_after));
        }
        if let Some(due_date_before) = &self.due_date_before {
            url.push_str(&format!("due_date_before={}&", due_date_before));
        }
        if let Some(query) = &self.query {
            url.push_str(&format!("query={}&", query));
        }
        if let Some(tag) = &self.tag {
            url.push_str(&format!("tag={}&", tag));
        }
        if let Some(tag_ids) = &self.tag_ids {
            url.push_str(&format!("tag_ids={}&", tag_ids));
        }
        if let Some(type_ids) = &self.type_ids {
            url.push_str(&format!("type_ids={}&", type_ids));
        }
        if let Some(exclude_board_ids) = config.get_exclude_board_ids() {
            url.push_str(&format!("exclude_board_ids={}&", exclude_board_ids));
        }
        if let Some(exclude_lane_ids) = config.get_exclude_lane_ids() {
            url.push_str(&format!("exclude_lane_ids={}&", exclude_lane_ids));
        }
        if let Some(exclude_column_ids) = config.get_exclude_column_ids() {
            url.push_str(&format!("exclude_column_ids={}&", exclude_column_ids));
        }
        if let Some(column_ids) = &self.column_ids {
            url.push_str(&format!("column_ids={}&", column_ids));
        }
        if let Some(member_ids) = &self.member_ids {
            url.push_str(&format!("member_ids={}&", member_ids));
        }
        if let Some(owner_ids) = &self.owner_ids {
            url.push_str(&format!("owner_ids={}&", owner_ids));
        }
        if let Some(responsible_ids) = &self.responsible_ids {
            url.push_str(&format!("responsible_ids={}&", responsible_ids));
        }
        if let Some(space_id) = config.get_space_id() {
            url.push_str(&format!("space_id={}&", space_id));
        }
        if let Some(limit) = self.limit {
            url.push_str(&format!("limit={}&", limit));
        }
        if let Some(offset) = self.offset {
            url.push_str(&format!("offset={}&", offset));
        }
        if let Some(order_space_id) = self.order_space_id {
            url.push_str(&format!("order_space_id={}&", order_space_id));
        }
        if let Some(column_id) = self.column_id {
            url.push_str(&format!("column_id={}&", column_id));
        }
        if let Some(lane_id) = self.lane_id {
            url.push_str(&format!("lane_id={}&", lane_id));
        }
        if let Some(type_id) = self.type_id {
            url.push_str(&format!("type_id={}&", type_id));
        }
        if let Some(responsible_id) = self.responsible_id {
            url.push_str(&format!("responsible_id={}&", responsible_id));
        }
        if let Some(owner_id) = self.owner_id {
            url.push_str(&format!("owner_id={}&", owner_id));
        }
        if let Some(archived) = self.archived {
            url.push_str(&format!("archived={}&", archived));
        }
        if let Some(asap) = self.asap {
            url.push_str(&format!("asap={}&", asap));
        }
        if let Some(overdue) = self.overdue {
            url.push_str(&format!("overdue={}&", overdue));
        }
        if let Some(done_on_time) = self.done_on_time {
            url.push_str(&format!("done_on_time={}&", done_on_time));
        }
        if let Some(with_due_date) = self.with_due_date {
            url.push_str(&format!("with_due_date={}&", with_due_date));
        }
        if url.ends_with('&') {
            url.pop();
        }
        url
    }
}

/// Helper function to validate ISO 8601 dates.
fn validate_iso8601(s: &str) -> Result<String, String> {
    Ok(s.to_string())
}

impl Card {
    pub fn get_url(&self) -> String {
        match &self.command {
            CardCommands::Ls(ls) => ls.get_url(),
            CardCommands::Get { card_id }
            | CardCommands::Edit { card_id, .. }
            | CardCommands::Mv { card_id, .. }
            | CardCommands::Parents { card_id }
            | CardCommands::Childrens { card_id } => {
                format!("cards/{}", card_id)
            }
            _ => String::new(),
        }
    }
    pub async fn get_table(&self, client: ApiClient) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = self.get_url();
        let response = client.get_data(&api_url).await?;
        let table = match &self.command {
            CardCommands::Get { card_id: _ } => {
                let card: ModelsCard = response.json().await?;
                card.to_table_string()
            }
            CardCommands::Edit{card_id: _, description, add_description, title, column_id, lane_id} => {
                let mut card: ModelsCard = response.json().await?;
                if let Some(desc) = description {
                    card.set_description(desc.to_owned(), false);
                };
                if let Some(add_desc) = add_description {
                    card.set_description(add_desc.to_owned(), true);
                };
                if let Some(title) = title {
                    card.set_title(title.to_owned());
                };
                if let Some(column_id) = column_id {
                    card.set_column_id(*column_id);
                    let info = INFO.get().unwrap();
                    let board_id = info.get_board_id_by_column_id(*column_id);
                    if let Some(board_id) = board_id {
                        card.set_board_id(board_id);
                    };
                };
                if let Some(lane_id) = lane_id {
                    card.set_lane_id(*lane_id);
                };
                let _ = client.patch_data(&api_url, card).await?;
                String::new()
            }
            CardCommands::Ls(ls) => {
                let mut cards: Vec<ModelsCard> = response.json().await?;
                if let Some(p_id) = ls.properties_id {
                    cards = cards.into_iter().filter(|c| c.is_property(p_id)).collect()
                }
                if let Some(pv_id) = ls.properties_value_id {
                    cards = cards
                        .into_iter()
                        .filter(|c| c.is_property_value(pv_id))
                        .collect()
                }
                cards.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());

                Table::new(cards)
                    .modify(Columns::first(), Width::increase(10))
                    .modify(Columns::single(1), Width::wrap(70).keep_words())
                    .modify(Columns::single(2), Width::increase(10))
                    .with(Style::modern())
                    .with(Width::wrap(135).keep_words())
                    .to_string()
            }
            CardCommands::Parents { card_id: _ } => {
                let card: ModelsCard = response.json().await?;
                Table::new(card.get_parents())
                    .modify(Columns::single(1), Width::wrap(80).keep_words())
                    .with(Style::modern())
                    .to_string()
            }
            CardCommands::Childrens { card_id: _ } => {
                let card: ModelsCard = response.json().await?;
                Table::new(card.get_childrens())
                    .modify(Columns::single(1), Width::wrap(80).keep_words())
                    .with(Style::modern())
                    .to_string()
            }

            CardCommands::New {} => ModelsCard::from_string(),

            CardCommands::Mv {
                card_id: _,
                column_id,
                lane_id,
                add_responsible,
            } => {
                let mut card: ModelsCard = response.json().await?;
                let info = INFO.get().unwrap();
                card.set_column_id(*column_id);
                let board_id = info.get_board_id_by_column_id(*column_id);
                card.set_lane_id(*lane_id);
                if let Some(board_id) = board_id {
                    card.set_board_id(board_id);
                } else {
                    let err: Box<dyn std::error::Error> =
                        format!("Not found board_id for column_id: {}", column_id).into();
                    return Err(err);
                }
                let api_url = self.get_url();
                let _ = client.patch_data(&api_url, card).await?;
                if let Some(username) = add_responsible {
                    let user = info.get_user(&username, None);
                    if let Some(mut user) = user {
                        user.set_responsible();
                        let api_url = format!("{}/members", api_url);
                        let _ = client.post_data(&api_url, &user).await?;
                        let api_url = format!("{}/{}", api_url, user.get_id());
                        let _ = client.patch_data(&api_url, user).await?;
                    }
                };
                String::from("")
            }
            _ => String::from(""),
        };
        Ok(table)
    }
}
