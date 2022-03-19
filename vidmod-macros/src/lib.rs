use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Ident};

#[macro_use]
extern crate quote;

#[proc_macro_attribute]
pub fn node_decl(_: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let ident = input_struct.ident.clone();
    let fields1 = input_struct.fields.iter();
    let output = quote! {
        #[derive(Debug)]
        pub struct #ident{
            #(#fields1,)*
            __node_node: vidmod_node::Node2,
        }

        impl #ident{
        }

        impl vidmod_node::Node2MT for #ident{
            fn register_pullport(&mut self, name:&str, kind: vidmod_node::FrameKind, buf_size: usize) {
                self.__node_node.register_pullport(name,kind,buf_size)
            }
            fn register_pushport(&mut self, name:&str, kind: vidmod_node::FrameKind, buf_size: usize) {
                self.__node_node.register_pushport(name,kind,buf_size)
            }
            fn get_pull_port(&self, id: usize, name: &str) -> anyhow::Result<PullPort> {
                self.__node_node.get_pull_port(id,name)
            }
            fn get_push_port(&self, id: usize, name: &str) -> anyhow::Result<PushPort> {
                self.__node_node.get_push_port(id,name)
            }
            fn attach_pull_port(&self, name: &str, port: PullPort) -> anyhow::Result<()> {
                self.__node_node.attach_pull_port(name,port)
            }
            fn attach_push_port(&self, name: &str, port: PushPort) -> anyhow::Result<()> {
                self.__node_node.attach_push_port(name,port)
            }
            fn ready_to_pull(&self, port: &PullPort) -> usize {
                self.__node_node.ready_to_pull(port)
            }
            fn ready_to_push(&self, port: &PushPort) -> usize {
                self.__node_node.ready_to_push(port)
            }
            fn pull_frame(&mut self, port: &PullPort, count: usize) -> vidmod_node::Frame {
                self.__node_node.pull_frame(port,count)
            }
            fn push_frame(&mut self, port: &PushPort, frame: vidmod_node::Frame) {
                self.__node_node.push_frame(port,frame)
            }
            fn inbuf_avail(&self, name: &str) -> usize {
                self.__node_node.inbuf_avail(name)
            }
            fn outbuf_avail(&self, name: &str) -> usize {
                self.__node_node.outbuf_avail(name)
            }
            fn outbuf_put(&mut self, name: &str, frame: vidmod_node::Frame) {
                self.__node_node.outbuf_put(name,frame)
            }
            fn outbuf_put_single(&mut self, name: &str, frame: vidmod_node::FrameSingle) {
                self.__node_node.outbuf_put_single(name,frame)
            }
            fn inbuf_get(&mut self, name: &str, count: usize) -> vidmod_node::Frame {
                self.__node_node.inbuf_get(name,count)
            }
            fn inbuf_peek(&mut self, name: &str, count: usize) -> vidmod_node::Frame {
                self.__node_node.inbuf_peek(name,count)
            }
            fn inbuf_get_single(&mut self, name: &str) -> vidmod_node::FrameSingle {
                self.__node_node.inbuf_get_single(name)
            }
            fn inbuf_get_all(&mut self, name: &str) -> vidmod_node::Frame {
                self.__node_node.inbuf_get_all(name)
            }
        }

        //Compile-time check to ensure our node implements Node2T
        const _: () = {
            fn assert_Node2T<T: vidmod_node::Node2T>() {}
            fn assert_all() {
                assert_Node2T::<#ident>();
            }
        };
    };
    output.into()
}

#[proc_macro_attribute]
pub fn node_new(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let stmts = &mut input_fn.block.stmts;
    let stmts_len = stmts.len();
    let struct_stmt = &mut stmts[stmts_len - 1];
    if let syn::Stmt::Expr(syn::Expr::Struct(s)) = struct_stmt {
        s.fields
            .push(syn::parse_quote!(__node_node: vidmod_node::Node2::new()));
    }
    let output = quote! {
        #input_fn
    };
    output.into()
}

struct Args {
    kind:   syn::Type,
    _comma: syn::token::Comma,
    dims:   syn::LitInt,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Args {
            kind:   input.parse()?,
            _comma: input.parse()?,
            dims:   input.parse()?,
        })
    }
}

#[proc_macro]
pub fn unwrap_impl_frame(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as Args);
    let kind = args.kind;
    let kind_str = quote!(#kind).to_string();
    let kind_str_lower = kind_str.to_lowercase();
    let kind_str_upper = kind_str.to_uppercase();
    let err_msg = format!("Tried to unwrap {{:?}} as {}", kind_str_upper);
    let dims = args.dims.base10_parse::<u8>().unwrap();
    let function_name = Ident::new(
        match dims {
            0 => format!("unwrap_{}", kind_str_lower),
            _ => format!("unwrap_{}x{}", kind_str_lower, dims),
        }
        .as_str(),
        Span::call_site(),
    );
    let enum_var = Ident::new(
        match dims {
            0 => kind_str_upper,
            _ => format!("{}x{}", kind_str_upper, dims),
        }
        .as_str(),
        Span::call_site(),
    );
    let retval = match dims {
        0 => quote!(#kind),
        1 => quote!(ArcArray1<#kind>),
        2 => quote!(ArcArray2<#kind>),
        _ => todo!("Return val for dim {}", dims),
    };
    let output = quote! {
        /// Unwrap the frame into its contents
        pub fn #function_name(self) -> LimVecDeque<#retval> {
            match self {
                Frame::#enum_var(v) => v,
                _ => panic!(#err_msg, FrameKind::from(&self)),
            }
        }
    };
    output.into()
}

#[proc_macro]
pub fn unwrap_impl_frame_single(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as Args);
    let kind = args.kind;
    let kind_str = quote!(#kind).to_string();
    let kind_str_lower = kind_str.to_lowercase();
    let kind_str_upper = kind_str.to_uppercase();
    let err_msg = format!("Tried to unwrap {{:?}} as {}", kind_str_upper);
    let dims = args.dims.base10_parse::<u8>().unwrap();
    let function_name = Ident::new(
        match dims {
            0 => format!("unwrap_{}", kind_str_lower),
            _ => format!("unwrap_{}x{}", kind_str_lower, dims),
        }
        .as_str(),
        Span::call_site(),
    );
    let enum_var = Ident::new(
        match dims {
            0 => kind_str_upper,
            _ => format!("{}x{}", kind_str_upper, dims),
        }
        .as_str(),
        Span::call_site(),
    );
    let retval = match dims {
        0 => quote!(#kind),
        1 => quote!(ArcArray1<#kind>),
        2 => quote!(ArcArray2<#kind>),
        _ => todo!("Return val for dim {}", dims),
    };
    let output = quote! {
        /// Unwrap the frame into its contents
        pub fn #function_name(self) -> #retval {
            match self {
                FrameSingle::#enum_var(v) => v,
                _ => panic!(#err_msg, FrameKind::from(&self)),
            }
        }
    };
    output.into()
}
