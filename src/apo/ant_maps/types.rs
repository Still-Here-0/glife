

pub mod node {
    use std::fmt::Debug;

    pub trait NodeType: Debug {}
    
    #[derive(Debug)]
    pub struct Priced;
    #[derive(Debug)]
    pub struct Unpriced;
    
    impl NodeType for Priced {}
    impl NodeType for Unpriced {}
}

pub mod map {
    pub mod direcional {
        use std::fmt::Debug;

        pub trait DirType: Debug {}

        #[derive(Debug)]
        pub struct Bidirecional;
        #[derive(Debug)]
        pub struct Unidirecional;

        impl DirType for Bidirecional {}
        impl DirType for Unidirecional {}
    }
    
    pub mod initialization {
        use std::fmt::Debug;

        pub trait InitType: Debug {}

        #[derive(Debug)]
        pub  struct Initialized;
        #[derive(Debug)]
        pub  struct Uninitialized;

        impl InitType for Initialized {}
        impl InitType for Uninitialized {}
    }
}
