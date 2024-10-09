use crate::history::HistoryEntry;

pub fn filter_commands(history: &[HistoryEntry], search_terms: &[String]) -> Vec<HistoryEntry> {
    history
        .iter()
        .filter(|entry| {
            search_terms
                .iter()
                .all(|term| entry.command.contains(term))
        })
        .cloned()
        .collect()
}
