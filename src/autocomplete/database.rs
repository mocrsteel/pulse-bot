use futures::{Stream, StreamExt};

use crate::globals::Context;

static TEST_NAMES: [&str; 4] = ["test", "myles", "myself", "race"];

pub async fn autocomplete_name<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&TEST_NAMES)
        .filter( move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}
