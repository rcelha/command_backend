use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CommandRegistry)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl Backend<CommandRegistry> for #name {
            fn execute(self: &Self, cmd: &CommandRegistry) -> Option<String> {
                match cmd {
                    CommandRegistry::Hello(cmd) => self.execute(cmd),
                    CommandRegistry::Exit(cmd) => self.execute(cmd),
                    _ => None,
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
