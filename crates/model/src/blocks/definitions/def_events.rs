use crate::DropdownSelection;

super::define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum EventBlockKind:

    "event_whenflagclicked" => EventWhenflagclicked,
    "event_whenkeypressed" => EventWhenkeypressed {
        (field) key_option: DropdownSelection,
    },
}
