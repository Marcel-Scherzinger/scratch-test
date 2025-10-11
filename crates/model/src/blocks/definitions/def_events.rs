use crate::attr::DropdownSelection;

super::define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum EventBlockKind (EventBlockKindUnit):

    "event_whenflagclicked" => EventWhenflagclicked,
    "event_whenkeypressed" => EventWhenkeypressed {
        (field) key_option: DropdownSelection,
    },
}
