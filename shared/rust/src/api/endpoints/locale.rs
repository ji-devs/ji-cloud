/// Entry endpoints
pub mod entry {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::locale::{
            CreateEntryRequest, CreateEntryResponse, GetEntryResponse, ListEntryQuery,
            ListEntryResponse, UpdateEntryRequest,
        },
        error::EmptyError,
    };

    /// Create an [`Entry`](crate::domain::locale::Entry)
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = CreateEntryRequest;
        type Res = CreateEntryResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/entry";
        const METHOD: Method = Method::Post;
    }

    /// List entries
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ListEntryQuery;
        type Res = ListEntryResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/entry";
        const METHOD: Method = Method::Get;
    }

    /// Get an individual [`Entry`](crate::domain::locale::Entry) by id
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = GetEntryResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/entry/{id}";
        const METHOD: Method = Method::Get;
    }

    /// Update an [`Entry`](crate::domain::locale::Entry)
    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = UpdateEntryRequest;
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/entry/{id}";
        const METHOD: Method = Method::Patch;
    }

    /// Delete an [`Entry`](crate::domain::locale::Entry)
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/entry/{id}";
        const METHOD: Method = Method::Delete;
    }
}

/// [`ItemKind`](crate::domain::locale::ItemKind) endpoints
pub mod item_kind {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::locale::ListItemKindResponse,
        error::EmptyError,
    };

    /// List [`ItemKind`](crate::domain::locale::ItemKind)s
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = ListItemKindResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/item-kind";
        const METHOD: Method = Method::Get;
    }
}

/// [`Bundle`](crate::domain::locale::Bundle) endpoints
pub mod bundle {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::locale::ListBundleResponse,
        error::EmptyError,
    };

    /// List [`Bundle`](crate::domain::locale::Bundle)s
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = ListBundleResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/locale/bundle";
        const METHOD: Method = Method::Get;
    }
}
