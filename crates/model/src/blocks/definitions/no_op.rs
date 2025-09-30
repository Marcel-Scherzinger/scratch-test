super::define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum NoopStmtBlockKind (NoopStmtBlockKindUnit):

    "data_showvariable" => DataShowvariable,
    "data_showlist" => DataShowlist,
    "data_hidevariable" => DataHidevariable,
    "data_hidelist" => DataHidelist,

    "looks_show" => LooksShow,

    // TODO: move
    skip => {
        ("procedures_call")  ProceduresCall {
            inputs: (),
            procode: String,
            argumentids: String,
        },
    },
}
