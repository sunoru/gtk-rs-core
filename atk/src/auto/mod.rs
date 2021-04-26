// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod action;
pub use self::action::{Action, NONE_ACTION};

mod component;
pub use self::component::{Component, NONE_COMPONENT};

mod document;
pub use self::document::{Document, NONE_DOCUMENT};

mod editable_text;
pub use self::editable_text::{EditableText, NONE_EDITABLE_TEXT};

mod gobject_accessible;
pub use self::gobject_accessible::{GObjectAccessible, NONE_GOBJECT_ACCESSIBLE};

mod hyperlink;
pub use self::hyperlink::{Hyperlink, NONE_HYPERLINK};

mod hyperlink_impl;
pub use self::hyperlink_impl::{HyperlinkImpl, NONE_HYPERLINK_IMPL};

mod hypertext;
pub use self::hypertext::{Hypertext, NONE_HYPERTEXT};

mod image;
pub use self::image::{Image, NONE_IMAGE};

mod misc;
pub use self::misc::{Misc, NONE_MISC};

mod no_op_object;
pub use self::no_op_object::{NoOpObject, NONE_NO_OP_OBJECT};

mod no_op_object_factory;
pub use self::no_op_object_factory::{NoOpObjectFactory, NONE_NO_OP_OBJECT_FACTORY};

mod object;
pub use self::object::{Object, NONE_OBJECT};

mod object_factory;
pub use self::object_factory::{ObjectFactory, NONE_OBJECT_FACTORY};

mod plug;
pub use self::plug::{Plug, NONE_PLUG};

mod registry;
pub use self::registry::{Registry, NONE_REGISTRY};

mod relation;
pub use self::relation::{Relation, NONE_RELATION};

mod relation_set;
pub use self::relation_set::{RelationSet, NONE_RELATION_SET};

mod selection;
pub use self::selection::{Selection, NONE_SELECTION};

mod socket;
pub use self::socket::{Socket, NONE_SOCKET};

mod state_set;
pub use self::state_set::{StateSet, NONE_STATE_SET};

mod streamable_content;
pub use self::streamable_content::{StreamableContent, NONE_STREAMABLE_CONTENT};

mod table;
pub use self::table::{Table, NONE_TABLE};

mod table_cell;
pub use self::table_cell::{TableCell, NONE_TABLE_CELL};

mod text;
pub use self::text::{Text, NONE_TEXT};

mod util;
pub use self::util::{Util, NONE_UTIL};

mod value;
pub use self::value::{Value, NONE_VALUE};

mod window;
pub use self::window::{Window, NONE_WINDOW};

mod range;
pub use self::range::Range;

mod rectangle;
pub use self::rectangle::Rectangle;

mod text_range;
pub use self::text_range::TextRange;

mod enums;
pub use self::enums::CoordType;
pub use self::enums::Layer;
pub use self::enums::RelationType;
pub use self::enums::Role;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
pub use self::enums::ScrollType;
pub use self::enums::StateType;
pub use self::enums::TextAttribute;
pub use self::enums::TextBoundary;
pub use self::enums::TextClipType;
pub use self::enums::TextGranularity;
pub use self::enums::ValueType;

mod flags;
pub use self::flags::HyperlinkStateFlags;

mod alias;
pub use self::alias::State;

#[doc(hidden)]
pub mod traits {
    pub use super::action::AtkActionExt;
    pub use super::component::ComponentExt;
    pub use super::document::DocumentExt;
    pub use super::editable_text::EditableTextExt;
    pub use super::gobject_accessible::GObjectAccessibleExt;
    pub use super::hyperlink::HyperlinkExt;
    pub use super::hyperlink_impl::HyperlinkImplExt;
    pub use super::hypertext::HypertextExt;
    pub use super::image::AtkImageExt;
    pub use super::misc::AtkMiscExt;
    pub use super::object::AtkObjectExt;
    pub use super::object_factory::ObjectFactoryExt;
    pub use super::plug::AtkPlugExt;
    pub use super::registry::RegistryExt;
    pub use super::relation::RelationExt;
    pub use super::relation_set::RelationSetExt;
    pub use super::selection::SelectionExt;
    pub use super::socket::AtkSocketExt;
    pub use super::state_set::StateSetExt;
    pub use super::streamable_content::StreamableContentExt;
    pub use super::table::TableExt;
    pub use super::table_cell::TableCellExt;
    pub use super::text::TextExt;
    pub use super::value::ValueExt;
    pub use super::window::AtkWindowExt;
}
