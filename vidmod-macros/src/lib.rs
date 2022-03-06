use proc_macro::TokenStream;

#[macro_use]
extern crate quote;

#[proc_macro_attribute]
pub fn node(_: TokenStream, item: TokenStream) -> TokenStream {
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
            fn inbuf_get_single(&mut self, name: &str) -> vidmod_node::FrameSingle {
                self.__node_node.inbuf_get_single(name)
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
pub fn node2(_: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = syn::parse_macro_input!(item as syn::ExprStruct);
    let fields = input_struct.fields.iter();
    let output = quote! {
        Self {
            #(#fields,)*
            __node_node: vidmod_node::Node2::new(),
        }
    };
    output.into()
}

#[proc_macro]
pub fn node_new(_item: TokenStream) -> TokenStream {
    let output = quote! {
        __node_node: vidmod_node::Node2::new(),
    };
    output.into()
}
