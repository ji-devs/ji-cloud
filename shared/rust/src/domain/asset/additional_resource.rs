//! Trait for additional resources.

use crate::domain::meta::ResourceTypeId;
use uuid::Uuid;

/// trait over additional resources
pub trait AdditionalResourceTrait {
    /// The additional resources's ID.
    fn id_uuid(&self) -> &Uuid;

    /// Name for additional resource
    fn display_name(&self) -> &String;

    /// Type of additional resource
    fn resource_type_id(&self) -> &ResourceTypeId;

    // / Content of additional resource
    // fn resource_content(&self) -> &ResourceContent;
}
