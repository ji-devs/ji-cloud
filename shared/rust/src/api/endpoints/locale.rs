/// Entry endpoints
pub mod entry {
    use crate::{
        api::{endpoints::ApiEndpoint, Method},
        domain::locale::{
            CreateEntryPath, CreateEntryRequest, CreateEntryResponse, DeleteEntryPath,
            GetEntryPath, GetEntryResponse, ListEntryPath, ListEntryQuery, ListEntryResponse,
            UpdateEntryPath, UpdateEntryRequest,
        },
        error::EmptyError,
    };

    /// Create an [`Entry`](crate::domain::locale::Entry)
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = CreateEntryRequest;
        type Res = CreateEntryResponse;
        type Path = CreateEntryPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Post;
    }

    /// List entries
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ListEntryQuery;
        type Res = ListEntryResponse;
        type Path = ListEntryPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }

    /// Get an individual [`Entry`](crate::domain::locale::Entry) by id
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = GetEntryResponse;
        type Path = GetEntryPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }

    /// Update an [`Entry`](crate::domain::locale::Entry)
    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = UpdateEntryRequest;
        type Res = ();
        type Path = UpdateEntryPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Patch;
    }

    /// Delete an [`Entry`](crate::domain::locale::Entry)
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Path = DeleteEntryPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Delete;
    }
}

/// [`ItemKind`](crate::domain::locale::ItemKind) endpoints
pub mod item_kind {
    use crate::{
        api::{endpoints::ApiEndpoint, Method},
        domain::locale::{ListItemKindPath, ListItemKindResponse},
        error::EmptyError,
    };

    /// List [`ItemKind`](crate::domain::locale::ItemKind)s
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = ListItemKindResponse;
        type Path = ListItemKindPath;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }
}

/// [`Bundle`](crate::domain::locale::Bundle) endpoints
pub mod bundle {
    use crate::{
        api::{endpoints::ApiEndpoint, Method},
        domain::locale::{ListBundlePath, ListBundleResponse},
        error::EmptyError,
    };

    /// List [`Bundle`](crate::domain::locale::Bundle)s
    pub struct List;
    impl ApiEndpoint for List {
        type Req = ();
        type Res = ListBundleResponse;
        type Path = ListBundlePath;
        type Err = EmptyError;
        const METHOD: Method = Method::Get;
    }
}
