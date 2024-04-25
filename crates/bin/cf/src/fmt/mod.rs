use api::cloudflare::dns::records::Record;
use tabled::{
    builder::Builder,
    settings::{object::Columns, Color, Style},
    Table,
};

pub fn dns_record_table(records: &[&Record]) -> Table {
    let mut builder = Builder::new();
    builder.push_record(["Name", "Type", "TTL", "Address"]);
    for record in records {
        builder.push_record([
            &record.name,
            &record.type_,
            &format!("{}s", record.ttl),
            &record.content,
        ]);
    }
    let mut table = builder.build();
    table
        .with(Style::rounded())
        .modify(Columns::first(), Color::BOLD | Color::FG_BRIGHT_GREEN)
        .modify(Columns::single(1), Color::BOLD | Color::FG_BRIGHT_BLUE);
    table
}
