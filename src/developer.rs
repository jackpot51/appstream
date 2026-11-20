use serde::{Deserialize, Serialize};

use super::TranslatableString;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
/// The person, team or organisation responsible for a component.
///
/// Maps to the `<developer>` tag introduced in AppStream 1.0, which replaces
/// the deprecated `<developer_name>` tag. See
/// <https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-developer>.
pub struct Developer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// A stable, machine-readable identifier shared by all of the developer's
    /// components, usually a reverse-DNS prefix
    pub id: Option<String>,

    #[serde(default)]
    /// The human-readable, translatable name.
    pub name: TranslatableString,
}
