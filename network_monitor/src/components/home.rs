use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
};
use std::collections::HashMap;

struct RowData {
    url: String,
    status: Option<u64>,
    mime_type: Option<String>,
    protocol: Option<String>,
    initiator: cdp::network::Initiator,
    r#type: Option<cdp::network::ResourceType>,
    data_length: Option<u64>,
    encoded_data_length: Option<u64>,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    row_data: HashMap<String, RowData>,
    table_state: TableState,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Down => {
                self.table_state.select_next();
            }
            Action::Up => {
                self.table_state.select_previous();
            }
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::AddNetworkRequest {
                r#type,
                request_id,
                url,
                initiator,
            } => {
                self.row_data.insert(
                    request_id,
                    RowData {
                        initiator,
                        url,
                        r#type,
                        mime_type: None,
                        protocol: None,
                        status: None,
                        encoded_data_length: None,
                        data_length: None,
                    },
                );
            }
            Action::UpdateNetworkRequestA {
                request_id,
                encoded_data_length,
                data_length,
            } => {
                if let Some(row_data) = self.row_data.get_mut(&request_id) {
                    row_data.data_length = Some(data_length);
                    row_data.encoded_data_length = Some(encoded_data_length);
                }
            }
            Action::UpdateNetworkRequestB {
                request_id,
                status,
                mime_type,
                protocol,
            } => {
                if let Some(row_data) = self.row_data.get_mut(&request_id) {
                    row_data.status = Some(status);
                    row_data.mime_type = Some(mime_type);
                    row_data.protocol = protocol;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let rows: Vec<_> = self
            .row_data
            .iter()
            .map(|(id, row_data)| {
                Row::new(vec![
                    id.clone(),
                    row_data.url.clone(),
                    (if row_data.status.is_some() {
                        format!("{}", row_data.status.unwrap())
                    } else {
                        "".to_owned()
                    }),
                    (if row_data.mime_type.is_some() {
                        row_data.mime_type.as_ref().unwrap().to_string()
                    } else {
                        "".to_owned()
                    }),
                    (if row_data.protocol.is_some() {
                        row_data.protocol.as_ref().unwrap().to_string()
                    } else {
                        "".to_owned()
                    }),
                    (if row_data.r#type.is_some() {
                        row_data.r#type.as_ref().unwrap().to_string()
                    } else {
                        "".to_owned()
                    }),
                    format_initiator(row_data.initiator.clone()),
                    (if row_data.data_length.is_some() && row_data.encoded_data_length.is_some() {
                        format!(
                            "{}/{}",
                            row_data.data_length.as_ref().unwrap(),
                            row_data.encoded_data_length.as_ref().unwrap()
                        )
                    } else {
                        "".to_owned()
                    }),
                ])
            })
            .collect();
        // status, scheme, type, initiator, size, time

        let widths = [
            Constraint::Length(15),
            Constraint::Length(75),
            Constraint::Length(10),
            Constraint::Length(17),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .header(
                Row::new(vec![
                    "ID",
                    "URL",
                    "STATUS",
                    "MIME TYPE",
                    "PROTOCOL",
                    "TYPE",
                    "INITIATOR",
                    "SIZE",
                ])
                .style(Style::new().bold())
                .bottom_margin(1),
            )
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");

        frame.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }
}

fn format_initiator(initiator: cdp::network::Initiator) -> String {
    if let Some(url) = initiator.url {
        let path = url.rsplit_once('/');
        let s = match path {
            Some((_, p)) => p,
            None => &url,
        };

        if initiator.line_number.is_some() {
            format!("{}:{}", s, initiator.line_number.unwrap())
        } else {
            s.to_string()
        }
    } else {
        String::new()
    }
}
