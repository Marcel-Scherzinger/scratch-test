//! Model to parse the [`*.sb3` file format](https://en.scratch-wiki.info/wiki/Scratch_File_Format)
//! of the [Scratch](https://scratch.mit.edu/) block-oriented programming language
//!
//! <table><tr>
//!
//! <td>
//!
//! [summary](../scratch_test/index.html)
//!
//! </td><td>
//!
//! [interpreter](../interpreter/index.html)
//!
//! </td><td>
//!
//! [scratch-yew](../scratch_yew/index.html)
//!
//! </td><td>
//!
//! [testreports](../testreports/index.html)
//!
//! </td><td>
//!
//! [testdata](../testdata/index.html)
//!
//! </td></tr></table>
//!
//! (I want to note that I was unable to find the above link to the format specification
//! when I was developing this so I reverse-engineered the format from example files.
//! Luckily, it looks like I correctly understood the meaning of the components.)
//!
//!
//! The program components extracted from a `*.sb3` file are represented by the
//! [`ProjectDoc`] type which offers different methods for parsing a file or
//! a sequence of bytes representing the file's content.
//!
//! ```
//! let doc = ProjectDoc::from_sb3_file("/path/to/file.sb3");
//! println!("{doc:#?}");
//! ```
//!
//! <div class="warning">
//!
//! This project doesn't aim to support all of scratch blocks and when the parsing functions
//! encounter an unknown block or a block that is known to be unsupported those errors
//! won't stop the parsing and will result in a valid document representation.
//! Those invalid blocks are stored differently and may cause problems if
//! the used virtual machine doesn't know how to handle a suddenly not available block.
//!
//! Use for example [`ProjectDoc::ensure_no_invalid_blocks`] on a parsed object to only
//! allow completly usable blocks in the document.
//!
//! ```
//! let doc = ProjectDoc::from_sb3_file("/path/to/file.sb3");
//! println!("Maybe also contains invalid blocks {doc:#?}");
//!
//! match doc.ensure_no_invalid_blocks() {
//!     Ok(doc) => {
//!         println!("Totally valid: {doc:#?}");
//!     }
//!     Err(doc) => {
//!         println!("There are invalid blocks, be extra careful: {doc:#?}");
//!     }
//! }
//! ```
//!
//! </div>
//!
//! # Steps
//!
//! ## JSON extraction
//!
//! A [Scratch file](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Project_Files)
//! is a ZIP file with images, sounds, ... and a `project.json`
//! that contains all information about the program structure.
//!
//! [`ProjectDoc`] offers associated functions for constructing a
//! document from a ZIP file like [`ProjectDoc::from_sb3_file`].
//! But if the used decompression algorithm is unable to read the file
//! or if you want to provide the `project.json` file directly
//! you can use [`ProjectDoc::from_json`] which can be called with
//! an already parsed JSON document.
//!
//! (See [`ProjectDoc`] for details.)
//!
//! (The next sections will explain how all program blocks are
//! extracted from an entered JSON document
//! – regardless how it was obtained in this section.)
//!
//! ## Targets
//!
//! Each document contains some meta information and – most
//! importantly – a `targets` field containing a list of "targets".
//! A [target](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Targets)
//! is a sprite or the background and contains different data.
//! (The sprites have _local_ variables and the background's
//! variables and lists are seen as _global_.)
//!
//! The relevant information stored in the modeled target:
//! - [`TargetVariables`]: all variables of the target (local for sprites and global for the stage)
//! - [`TargetLists`]: all lists of the target (local for sprites and global for the stage)
//! - [`TargetBlocks`]: collection of all statement, expression, comparison, event, ... blocks of the program
//! - [`TargetProcedures`]: generated from the blocks.
//!   (The information about a procedure is split
//!   across multiple blocks in a weird format.
//!   This combines this data per procedure)
//!
//! (See [`Target`] for details.)

/// copied from [<https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js>]
pub mod constants;

mod blocks;
pub mod error;
mod ext;
mod interpret_json;
mod reader;
mod scopes;
mod scratch_expr;

pub use blocks::GetOpcodeUnit;
pub use blocks::{
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};

pub mod block_opcodes {
    pub use crate::blocks::{
        BlockKindUnit, CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit,
        NoopStmtBlockKindUnit, StmtBlockKindUnit, UnsupportedBlockKindUnit,
    };
}

pub type Id = std::rc::Rc<str>;
pub use interpret_json::OpcodeNum;

pub use error::{DocError, Error};
pub use reader::json_from_sb3_stream;
pub use scopes::*;
pub use scratch_expr::{SValue, SValue as VariableValue, ScratchExpr};

pub mod attr {
    pub use crate::interpret_json::{
        ArgumentReporterName, DropdownSelection, Expression, List, ProcedureArgumentDef, RefBlock,
        Variable,
    };
}
