mod frame;
mod instance;
mod tests;
mod value_store;
pub mod wasi;

// Main external interface
pub use instance::Instance;
pub use wasi::{WasiDispatcher, WasiFile};

pub use roc_wasm_module::Value;
use roc_wasm_module::ValueType;

pub trait ImportDispatcher {
    /// Dispatch a call from WebAssembly to your own code, based on module and function name.
    fn dispatch(
        &mut self,
        module_name: &str,
        function_name: &str,
        arguments: &[Value],
        memory: &mut [u8],
    ) -> Option<Value>;
}

impl Default for DefaultImportDispatcher<'_> {
    fn default() -> Self {
        DefaultImportDispatcher {
            wasi: WasiDispatcher::new(&[]),
        }
    }
}

pub struct DefaultImportDispatcher<'a> {
    pub wasi: WasiDispatcher<'a>,
}

impl<'a> DefaultImportDispatcher<'a> {
    pub fn new(args: &'a [&'a [u8]]) -> Self {
        DefaultImportDispatcher {
            wasi: WasiDispatcher::new(args),
        }
    }
}

impl<'a> ImportDispatcher for DefaultImportDispatcher<'a> {
    fn dispatch(
        &mut self,
        module_name: &str,
        function_name: &str,
        arguments: &[Value],
        memory: &mut [u8],
    ) -> Option<Value> {
        if module_name == wasi::MODULE_NAME {
            self.wasi.dispatch(function_name, arguments, memory)
        } else {
            panic!(
                "DefaultImportDispatcher does not implement {}.{}",
                module_name, function_name
            );
        }
    }
}

/// Errors that can happen while interpreting the program
/// All of these cause a WebAssembly stack trace to be dumped
#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    Type(ValueType, ValueType),
    StackEmpty,
    UnreachableOp,
}

impl Error {
    pub fn to_string_at(&self, file_offset: usize) -> String {
        match self {
            Error::Type(expected, actual) => {
                format!(
                    "ERROR: I found a type mismatch at file offset {:#x}. Expected {:?}, but found {:?}.\n", 
                    file_offset, expected, actual
                )
            }
            Error::StackEmpty => {
                format!(
                    "ERROR: I tried to pop a value from the stack at file offset {:#x}, but it was empty.\n",
                    file_offset
                )
            }
            Error::UnreachableOp => {
                format!(
                    "WebAssembly `unreachable` instruction at file offset {:#x}.\n",
                    file_offset
                )
            }
        }
    }
}

impl From<(ValueType, ValueType)> for Error {
    fn from((expected, actual): (ValueType, ValueType)) -> Self {
        Error::Type(expected, actual)
    }
}
