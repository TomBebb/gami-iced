use std::fmt;

use gami_sdk::GameData;
use iced::{
    font::Weight,
    widget::{container, responsive, scrollable, text},
    Element, Font, Length, Renderer, Task, Theme,
};
use iced_table::table;

#[derive(Debug, Copy, Clone)]
pub enum TableMessage {
    SyncHeader(scrollable::AbsoluteOffset),
}

#[derive(Copy, Clone, Debug)]
pub enum ColumnKind {
    Name,
    LastPlayed,
    Playtime,
    Description,
}

impl fmt::Display for ColumnKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ColumnKind::Name => "Name",
            ColumnKind::LastPlayed => "Last Played",
            ColumnKind::Playtime => "Time Played",
            ColumnKind::Description => "Description",
        })
    }
}
#[derive(Clone, Debug)]
struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        Self {
            kind,
            width: 100.,
            resize_offset: None,
        }
    }
}

impl<'a, Message> table::Column<'a, Message, Theme, Renderer> for Column
where
    Message: 'a,
{
    type Row = GameData;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message, Theme, Renderer> {
        container(text(self.kind.to_string()).font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        }))
        .center_y(24)
        .into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, Message, Theme, Renderer> {
        let content: Element<_> = match self.kind {
            ColumnKind::Description => text(&row.description).into(),
            ColumnKind::Name => text(&row.name).into(),
            ColumnKind::LastPlayed => {
                /*
                let d= row.last_played;

                text(d.map(|t| format("{}", t.duration_since(earlier)))).into()
                */
                text("TODO").into()
            }
            ColumnKind::Playtime => text(row.play_time.as_secs()).into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

#[derive(Clone, Debug)]
pub struct LibraryTable {
    columns: Vec<Column>,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    pub rows: Vec<GameData>,
}

impl LibraryTable {
    pub fn new() -> Self {
        Self {
            columns: [
                ColumnKind::Name,
                ColumnKind::Playtime,
                ColumnKind::LastPlayed,
                ColumnKind::Description,
            ]
            .into_iter()
            .map(Column::new)
            .collect(),
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            rows: vec![],
        }
    }
    pub fn update(&mut self, msg: TableMessage) -> Task<TableMessage> {
        match msg {
            TableMessage::SyncHeader(offset) => {
                return Task::batch(vec![
                    scrollable::scroll_to(self.header.clone(), offset),
                    scrollable::scroll_to(self.footer.clone(), offset),
                ])
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<TableMessage> {
        responsive(|size| {
            table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                TableMessage::SyncHeader,
            )
            .min_width(size.width)
            .into()
        })
        .into()
    }
}
