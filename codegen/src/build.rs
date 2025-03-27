use proc_macro2::TokenStream;
use quote::quote;

pub fn build_file() -> TokenStream {
    quote! {
        fn main() {
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            {
                if cfg!(target_feature = "sse2") {
                    println!("cargo:rustc-cfg=v_escape_sse");
                }

                if cfg!(target_feature = "avx2") {
                    println!("cargo:rustc-cfg=v_escape_avx");
                }
            }
        }
    }
}
