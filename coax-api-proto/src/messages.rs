// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct GenericMessage {
    // message fields
    message_id: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    content: ::std::option::Option<GenericMessage_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GenericMessage {}

#[derive(Clone,PartialEq)]
pub enum GenericMessage_oneof_content {
    text(Text),
    image(ImageAsset),
    knock(Knock),
    lastRead(LastRead),
    cleared(Cleared),
    external(External),
    clientAction(ClientAction),
    calling(Calling),
    asset(Asset),
    hidden(MessageHide),
    location(Location),
    deleted(MessageDelete),
    edited(MessageEdit),
    confirmation(Confirmation),
    reaction(Reaction),
    ephemeral(Ephemeral),
}

impl GenericMessage {
    pub fn new() -> GenericMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GenericMessage {
        static mut instance: ::protobuf::lazy::Lazy<GenericMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenericMessage,
        };
        unsafe {
            instance.get(GenericMessage::new)
        }
    }

    // required string message_id = 1;

    pub fn clear_message_id(&mut self) {
        self.message_id.clear();
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: ::std::string::String) {
        self.message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&mut self) -> &mut ::std::string::String {
        if self.message_id.is_none() {
            self.message_id.set_default();
        };
        self.message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_id(&mut self) -> ::std::string::String {
        self.message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_id(&self) -> &str {
        match self.message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message_id
    }

    fn mut_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message_id
    }

    // optional .Text text = 2;

    pub fn clear_text(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_text(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: Text) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut Text {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::text(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::text(Text::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text(&mut self) -> Text {
        if self.has_text() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::text(v)) => v,
                _ => panic!(),
            }
        } else {
            Text::new()
        }
    }

    pub fn get_text(&self) -> &Text {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::text(ref v)) => v,
            _ => Text::default_instance(),
        }
    }

    // optional .ImageAsset image = 3;

    pub fn clear_image(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_image(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::image(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ImageAsset) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::image(v))
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut ImageAsset {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::image(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::image(ImageAsset::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::image(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_image(&mut self) -> ImageAsset {
        if self.has_image() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::image(v)) => v,
                _ => panic!(),
            }
        } else {
            ImageAsset::new()
        }
    }

    pub fn get_image(&self) -> &ImageAsset {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::image(ref v)) => v,
            _ => ImageAsset::default_instance(),
        }
    }

    // optional .Knock knock = 4;

    pub fn clear_knock(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_knock(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::knock(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_knock(&mut self, v: Knock) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::knock(v))
    }

    // Mutable pointer to the field.
    pub fn mut_knock(&mut self) -> &mut Knock {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::knock(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::knock(Knock::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::knock(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_knock(&mut self) -> Knock {
        if self.has_knock() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::knock(v)) => v,
                _ => panic!(),
            }
        } else {
            Knock::new()
        }
    }

    pub fn get_knock(&self) -> &Knock {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::knock(ref v)) => v,
            _ => Knock::default_instance(),
        }
    }

    // optional .LastRead lastRead = 6;

    pub fn clear_lastRead(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_lastRead(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_lastRead(&mut self, v: LastRead) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(v))
    }

    // Mutable pointer to the field.
    pub fn mut_lastRead(&mut self) -> &mut LastRead {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(LastRead::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_lastRead(&mut self) -> LastRead {
        if self.has_lastRead() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(v)) => v,
                _ => panic!(),
            }
        } else {
            LastRead::new()
        }
    }

    pub fn get_lastRead(&self) -> &LastRead {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(ref v)) => v,
            _ => LastRead::default_instance(),
        }
    }

    // optional .Cleared cleared = 7;

    pub fn clear_cleared(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_cleared(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::cleared(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cleared(&mut self, v: Cleared) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::cleared(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cleared(&mut self) -> &mut Cleared {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::cleared(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::cleared(Cleared::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::cleared(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cleared(&mut self) -> Cleared {
        if self.has_cleared() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::cleared(v)) => v,
                _ => panic!(),
            }
        } else {
            Cleared::new()
        }
    }

    pub fn get_cleared(&self) -> &Cleared {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::cleared(ref v)) => v,
            _ => Cleared::default_instance(),
        }
    }

    // optional .External external = 8;

    pub fn clear_external(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_external(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::external(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_external(&mut self, v: External) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::external(v))
    }

    // Mutable pointer to the field.
    pub fn mut_external(&mut self) -> &mut External {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::external(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::external(External::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::external(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_external(&mut self) -> External {
        if self.has_external() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::external(v)) => v,
                _ => panic!(),
            }
        } else {
            External::new()
        }
    }

    pub fn get_external(&self) -> &External {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::external(ref v)) => v,
            _ => External::default_instance(),
        }
    }

    // optional .ClientAction clientAction = 9;

    pub fn clear_clientAction(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_clientAction(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::clientAction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clientAction(&mut self, v: ClientAction) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::clientAction(v))
    }

    pub fn get_clientAction(&self) -> ClientAction {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::clientAction(v)) => v,
            _ => ClientAction::RESET_SESSION,
        }
    }

    // optional .Calling calling = 10;

    pub fn clear_calling(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_calling(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::calling(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_calling(&mut self, v: Calling) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::calling(v))
    }

    // Mutable pointer to the field.
    pub fn mut_calling(&mut self) -> &mut Calling {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::calling(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::calling(Calling::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::calling(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_calling(&mut self) -> Calling {
        if self.has_calling() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::calling(v)) => v,
                _ => panic!(),
            }
        } else {
            Calling::new()
        }
    }

    pub fn get_calling(&self) -> &Calling {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::calling(ref v)) => v,
            _ => Calling::default_instance(),
        }
    }

    // optional .Asset asset = 11;

    pub fn clear_asset(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_asset(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::asset(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_asset(&mut self, v: Asset) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::asset(v))
    }

    // Mutable pointer to the field.
    pub fn mut_asset(&mut self) -> &mut Asset {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::asset(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::asset(Asset::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::asset(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_asset(&mut self) -> Asset {
        if self.has_asset() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::asset(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset::new()
        }
    }

    pub fn get_asset(&self) -> &Asset {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::asset(ref v)) => v,
            _ => Asset::default_instance(),
        }
    }

    // optional .MessageHide hidden = 12;

    pub fn clear_hidden(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_hidden(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::hidden(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hidden(&mut self, v: MessageHide) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::hidden(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hidden(&mut self) -> &mut MessageHide {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::hidden(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::hidden(MessageHide::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::hidden(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hidden(&mut self) -> MessageHide {
        if self.has_hidden() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::hidden(v)) => v,
                _ => panic!(),
            }
        } else {
            MessageHide::new()
        }
    }

    pub fn get_hidden(&self) -> &MessageHide {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::hidden(ref v)) => v,
            _ => MessageHide::default_instance(),
        }
    }

    // optional .Location location = 13;

    pub fn clear_location(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_location(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::location(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Location) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::location(v))
    }

    // Mutable pointer to the field.
    pub fn mut_location(&mut self) -> &mut Location {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::location(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::location(Location::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::location(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_location(&mut self) -> Location {
        if self.has_location() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::location(v)) => v,
                _ => panic!(),
            }
        } else {
            Location::new()
        }
    }

    pub fn get_location(&self) -> &Location {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::location(ref v)) => v,
            _ => Location::default_instance(),
        }
    }

    // optional .MessageDelete deleted = 14;

    pub fn clear_deleted(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_deleted(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::deleted(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: MessageDelete) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::deleted(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deleted(&mut self) -> &mut MessageDelete {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::deleted(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::deleted(MessageDelete::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::deleted(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deleted(&mut self) -> MessageDelete {
        if self.has_deleted() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::deleted(v)) => v,
                _ => panic!(),
            }
        } else {
            MessageDelete::new()
        }
    }

    pub fn get_deleted(&self) -> &MessageDelete {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::deleted(ref v)) => v,
            _ => MessageDelete::default_instance(),
        }
    }

    // optional .MessageEdit edited = 15;

    pub fn clear_edited(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_edited(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::edited(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_edited(&mut self, v: MessageEdit) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::edited(v))
    }

    // Mutable pointer to the field.
    pub fn mut_edited(&mut self) -> &mut MessageEdit {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::edited(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::edited(MessageEdit::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::edited(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_edited(&mut self) -> MessageEdit {
        if self.has_edited() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::edited(v)) => v,
                _ => panic!(),
            }
        } else {
            MessageEdit::new()
        }
    }

    pub fn get_edited(&self) -> &MessageEdit {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::edited(ref v)) => v,
            _ => MessageEdit::default_instance(),
        }
    }

    // optional .Confirmation confirmation = 16;

    pub fn clear_confirmation(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_confirmation(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_confirmation(&mut self, v: Confirmation) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(v))
    }

    // Mutable pointer to the field.
    pub fn mut_confirmation(&mut self) -> &mut Confirmation {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(Confirmation::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_confirmation(&mut self) -> Confirmation {
        if self.has_confirmation() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(v)) => v,
                _ => panic!(),
            }
        } else {
            Confirmation::new()
        }
    }

    pub fn get_confirmation(&self) -> &Confirmation {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(ref v)) => v,
            _ => Confirmation::default_instance(),
        }
    }

    // optional .Reaction reaction = 17;

    pub fn clear_reaction(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_reaction(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::reaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reaction(&mut self, v: Reaction) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::reaction(v))
    }

    // Mutable pointer to the field.
    pub fn mut_reaction(&mut self) -> &mut Reaction {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::reaction(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::reaction(Reaction::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::reaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reaction(&mut self) -> Reaction {
        if self.has_reaction() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::reaction(v)) => v,
                _ => panic!(),
            }
        } else {
            Reaction::new()
        }
    }

    pub fn get_reaction(&self) -> &Reaction {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::reaction(ref v)) => v,
            _ => Reaction::default_instance(),
        }
    }

    // optional .Ephemeral ephemeral = 18;

    pub fn clear_ephemeral(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_ephemeral(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ephemeral(&mut self, v: Ephemeral) {
        self.content = ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ephemeral(&mut self) -> &mut Ephemeral {
        if let ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(Ephemeral::new()));
        }
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ephemeral(&mut self) -> Ephemeral {
        if self.has_ephemeral() {
            match self.content.take() {
                ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(v)) => v,
                _ => panic!(),
            }
        } else {
            Ephemeral::new()
        }
    }

    pub fn get_ephemeral(&self) -> &Ephemeral {
        match self.content {
            ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(ref v)) => v,
            _ => Ephemeral::default_instance(),
        }
    }
}

impl ::protobuf::Message for GenericMessage {
    fn is_initialized(&self) -> bool {
        if self.message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::text(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::image(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::knock(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::lastRead(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::cleared(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::external(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::clientAction(is.read_enum()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::calling(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::asset(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::hidden(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::location(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::deleted(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::edited(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::confirmation(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::reaction(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(GenericMessage_oneof_content::ephemeral(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &GenericMessage_oneof_content::text(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::image(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::knock(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::lastRead(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::cleared(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::external(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::clientAction(v) => {
                    my_size += ::protobuf::rt::enum_size(9, v);
                },
                &GenericMessage_oneof_content::calling(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::asset(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::hidden(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::location(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::deleted(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::edited(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::confirmation(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::reaction(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &GenericMessage_oneof_content::ephemeral(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &GenericMessage_oneof_content::text(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::image(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::knock(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::lastRead(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::cleared(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::external(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::clientAction(v) => {
                    os.write_enum(9, v.value())?;
                },
                &GenericMessage_oneof_content::calling(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::asset(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::hidden(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::location(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::deleted(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::edited(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::confirmation(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::reaction(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &GenericMessage_oneof_content::ephemeral(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GenericMessage {
    fn new() -> GenericMessage {
        GenericMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GenericMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    GenericMessage::get_message_id_for_reflect,
                    GenericMessage::mut_message_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Text>(
                    "text",
                    GenericMessage::has_text,
                    GenericMessage::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ImageAsset>(
                    "image",
                    GenericMessage::has_image,
                    GenericMessage::get_image,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Knock>(
                    "knock",
                    GenericMessage::has_knock,
                    GenericMessage::get_knock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LastRead>(
                    "lastRead",
                    GenericMessage::has_lastRead,
                    GenericMessage::get_lastRead,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Cleared>(
                    "cleared",
                    GenericMessage::has_cleared,
                    GenericMessage::get_cleared,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, External>(
                    "external",
                    GenericMessage::has_external,
                    GenericMessage::get_external,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, ClientAction>(
                    "clientAction",
                    GenericMessage::has_clientAction,
                    GenericMessage::get_clientAction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Calling>(
                    "calling",
                    GenericMessage::has_calling,
                    GenericMessage::get_calling,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset>(
                    "asset",
                    GenericMessage::has_asset,
                    GenericMessage::get_asset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessageHide>(
                    "hidden",
                    GenericMessage::has_hidden,
                    GenericMessage::get_hidden,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Location>(
                    "location",
                    GenericMessage::has_location,
                    GenericMessage::get_location,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessageDelete>(
                    "deleted",
                    GenericMessage::has_deleted,
                    GenericMessage::get_deleted,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessageEdit>(
                    "edited",
                    GenericMessage::has_edited,
                    GenericMessage::get_edited,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Confirmation>(
                    "confirmation",
                    GenericMessage::has_confirmation,
                    GenericMessage::get_confirmation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Reaction>(
                    "reaction",
                    GenericMessage::has_reaction,
                    GenericMessage::get_reaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Ephemeral>(
                    "ephemeral",
                    GenericMessage::has_ephemeral,
                    GenericMessage::get_ephemeral,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenericMessage>(
                    "GenericMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GenericMessage {
    fn clear(&mut self) {
        self.clear_message_id();
        self.clear_text();
        self.clear_image();
        self.clear_knock();
        self.clear_lastRead();
        self.clear_cleared();
        self.clear_external();
        self.clear_clientAction();
        self.clear_calling();
        self.clear_asset();
        self.clear_hidden();
        self.clear_location();
        self.clear_deleted();
        self.clear_edited();
        self.clear_confirmation();
        self.clear_reaction();
        self.clear_ephemeral();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenericMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenericMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ephemeral {
    // message fields
    expire_after_millis: ::std::option::Option<i64>,
    // message oneof groups
    content: ::std::option::Option<Ephemeral_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ephemeral {}

#[derive(Clone,PartialEq)]
pub enum Ephemeral_oneof_content {
    text(Text),
    image(ImageAsset),
    knock(Knock),
    asset(Asset),
    location(Location),
}

impl Ephemeral {
    pub fn new() -> Ephemeral {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ephemeral {
        static mut instance: ::protobuf::lazy::Lazy<Ephemeral> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ephemeral,
        };
        unsafe {
            instance.get(Ephemeral::new)
        }
    }

    // required int64 expire_after_millis = 1;

    pub fn clear_expire_after_millis(&mut self) {
        self.expire_after_millis = ::std::option::Option::None;
    }

    pub fn has_expire_after_millis(&self) -> bool {
        self.expire_after_millis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expire_after_millis(&mut self, v: i64) {
        self.expire_after_millis = ::std::option::Option::Some(v);
    }

    pub fn get_expire_after_millis(&self) -> i64 {
        self.expire_after_millis.unwrap_or(0)
    }

    fn get_expire_after_millis_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.expire_after_millis
    }

    fn mut_expire_after_millis_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.expire_after_millis
    }

    // optional .Text text = 2;

    pub fn clear_text(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_text(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: Text) {
        self.content = ::std::option::Option::Some(Ephemeral_oneof_content::text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut Text {
        if let ::std::option::Option::Some(Ephemeral_oneof_content::text(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Ephemeral_oneof_content::text(Text::new()));
        }
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text(&mut self) -> Text {
        if self.has_text() {
            match self.content.take() {
                ::std::option::Option::Some(Ephemeral_oneof_content::text(v)) => v,
                _ => panic!(),
            }
        } else {
            Text::new()
        }
    }

    pub fn get_text(&self) -> &Text {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::text(ref v)) => v,
            _ => Text::default_instance(),
        }
    }

    // optional .ImageAsset image = 3;

    pub fn clear_image(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_image(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::image(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ImageAsset) {
        self.content = ::std::option::Option::Some(Ephemeral_oneof_content::image(v))
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut ImageAsset {
        if let ::std::option::Option::Some(Ephemeral_oneof_content::image(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Ephemeral_oneof_content::image(ImageAsset::new()));
        }
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::image(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_image(&mut self) -> ImageAsset {
        if self.has_image() {
            match self.content.take() {
                ::std::option::Option::Some(Ephemeral_oneof_content::image(v)) => v,
                _ => panic!(),
            }
        } else {
            ImageAsset::new()
        }
    }

    pub fn get_image(&self) -> &ImageAsset {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::image(ref v)) => v,
            _ => ImageAsset::default_instance(),
        }
    }

    // optional .Knock knock = 4;

    pub fn clear_knock(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_knock(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::knock(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_knock(&mut self, v: Knock) {
        self.content = ::std::option::Option::Some(Ephemeral_oneof_content::knock(v))
    }

    // Mutable pointer to the field.
    pub fn mut_knock(&mut self) -> &mut Knock {
        if let ::std::option::Option::Some(Ephemeral_oneof_content::knock(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Ephemeral_oneof_content::knock(Knock::new()));
        }
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::knock(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_knock(&mut self) -> Knock {
        if self.has_knock() {
            match self.content.take() {
                ::std::option::Option::Some(Ephemeral_oneof_content::knock(v)) => v,
                _ => panic!(),
            }
        } else {
            Knock::new()
        }
    }

    pub fn get_knock(&self) -> &Knock {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::knock(ref v)) => v,
            _ => Knock::default_instance(),
        }
    }

    // optional .Asset asset = 5;

    pub fn clear_asset(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_asset(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::asset(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_asset(&mut self, v: Asset) {
        self.content = ::std::option::Option::Some(Ephemeral_oneof_content::asset(v))
    }

    // Mutable pointer to the field.
    pub fn mut_asset(&mut self) -> &mut Asset {
        if let ::std::option::Option::Some(Ephemeral_oneof_content::asset(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Ephemeral_oneof_content::asset(Asset::new()));
        }
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::asset(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_asset(&mut self) -> Asset {
        if self.has_asset() {
            match self.content.take() {
                ::std::option::Option::Some(Ephemeral_oneof_content::asset(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset::new()
        }
    }

    pub fn get_asset(&self) -> &Asset {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::asset(ref v)) => v,
            _ => Asset::default_instance(),
        }
    }

    // optional .Location location = 6;

    pub fn clear_location(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_location(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::location(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Location) {
        self.content = ::std::option::Option::Some(Ephemeral_oneof_content::location(v))
    }

    // Mutable pointer to the field.
    pub fn mut_location(&mut self) -> &mut Location {
        if let ::std::option::Option::Some(Ephemeral_oneof_content::location(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Ephemeral_oneof_content::location(Location::new()));
        }
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::location(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_location(&mut self) -> Location {
        if self.has_location() {
            match self.content.take() {
                ::std::option::Option::Some(Ephemeral_oneof_content::location(v)) => v,
                _ => panic!(),
            }
        } else {
            Location::new()
        }
    }

    pub fn get_location(&self) -> &Location {
        match self.content {
            ::std::option::Option::Some(Ephemeral_oneof_content::location(ref v)) => v,
            _ => Location::default_instance(),
        }
    }
}

impl ::protobuf::Message for Ephemeral {
    fn is_initialized(&self) -> bool {
        if self.expire_after_millis.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.expire_after_millis = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(Ephemeral_oneof_content::text(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(Ephemeral_oneof_content::image(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(Ephemeral_oneof_content::knock(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(Ephemeral_oneof_content::asset(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(Ephemeral_oneof_content::location(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.expire_after_millis {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &Ephemeral_oneof_content::text(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Ephemeral_oneof_content::image(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Ephemeral_oneof_content::knock(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Ephemeral_oneof_content::asset(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Ephemeral_oneof_content::location(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.expire_after_millis {
            os.write_int64(1, v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &Ephemeral_oneof_content::text(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Ephemeral_oneof_content::image(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Ephemeral_oneof_content::knock(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Ephemeral_oneof_content::asset(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Ephemeral_oneof_content::location(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ephemeral {
    fn new() -> Ephemeral {
        Ephemeral::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ephemeral>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expire_after_millis",
                    Ephemeral::get_expire_after_millis_for_reflect,
                    Ephemeral::mut_expire_after_millis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Text>(
                    "text",
                    Ephemeral::has_text,
                    Ephemeral::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ImageAsset>(
                    "image",
                    Ephemeral::has_image,
                    Ephemeral::get_image,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Knock>(
                    "knock",
                    Ephemeral::has_knock,
                    Ephemeral::get_knock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset>(
                    "asset",
                    Ephemeral::has_asset,
                    Ephemeral::get_asset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Location>(
                    "location",
                    Ephemeral::has_location,
                    Ephemeral::get_location,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ephemeral>(
                    "Ephemeral",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ephemeral {
    fn clear(&mut self) {
        self.clear_expire_after_millis();
        self.clear_text();
        self.clear_image();
        self.clear_knock();
        self.clear_asset();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ephemeral {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ephemeral {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Text {
    // message fields
    content: ::protobuf::SingularField<::std::string::String>,
    mention: ::protobuf::RepeatedField<Mention>,
    link_preview: ::protobuf::RepeatedField<LinkPreview>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Text {}

impl Text {
    pub fn new() -> Text {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Text {
        static mut instance: ::protobuf::lazy::Lazy<Text> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Text,
        };
        unsafe {
            instance.get(Text::new)
        }
    }

    // required string content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        self.content.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        match self.content.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content
    }

    // repeated .Mention mention = 2;

    pub fn clear_mention(&mut self) {
        self.mention.clear();
    }

    // Param is passed by value, moved
    pub fn set_mention(&mut self, v: ::protobuf::RepeatedField<Mention>) {
        self.mention = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mention(&mut self) -> &mut ::protobuf::RepeatedField<Mention> {
        &mut self.mention
    }

    // Take field
    pub fn take_mention(&mut self) -> ::protobuf::RepeatedField<Mention> {
        ::std::mem::replace(&mut self.mention, ::protobuf::RepeatedField::new())
    }

    pub fn get_mention(&self) -> &[Mention] {
        &self.mention
    }

    fn get_mention_for_reflect(&self) -> &::protobuf::RepeatedField<Mention> {
        &self.mention
    }

    fn mut_mention_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mention> {
        &mut self.mention
    }

    // repeated .LinkPreview link_preview = 3;

    pub fn clear_link_preview(&mut self) {
        self.link_preview.clear();
    }

    // Param is passed by value, moved
    pub fn set_link_preview(&mut self, v: ::protobuf::RepeatedField<LinkPreview>) {
        self.link_preview = v;
    }

    // Mutable pointer to the field.
    pub fn mut_link_preview(&mut self) -> &mut ::protobuf::RepeatedField<LinkPreview> {
        &mut self.link_preview
    }

    // Take field
    pub fn take_link_preview(&mut self) -> ::protobuf::RepeatedField<LinkPreview> {
        ::std::mem::replace(&mut self.link_preview, ::protobuf::RepeatedField::new())
    }

    pub fn get_link_preview(&self) -> &[LinkPreview] {
        &self.link_preview
    }

    fn get_link_preview_for_reflect(&self) -> &::protobuf::RepeatedField<LinkPreview> {
        &self.link_preview
    }

    fn mut_link_preview_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LinkPreview> {
        &mut self.link_preview
    }
}

impl ::protobuf::Message for Text {
    fn is_initialized(&self) -> bool {
        if self.content.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mention)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.link_preview)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.content.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.mention {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.link_preview {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.content.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.mention {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.link_preview {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Text {
    fn new() -> Text {
        Text::new()
    }

    fn descriptor_static(_: ::std::option::Option<Text>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    Text::get_content_for_reflect,
                    Text::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mention>>(
                    "mention",
                    Text::get_mention_for_reflect,
                    Text::mut_mention_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LinkPreview>>(
                    "link_preview",
                    Text::get_link_preview_for_reflect,
                    Text::mut_link_preview_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Text>(
                    "Text",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Text {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_mention();
        self.clear_link_preview();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Text {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Knock {
    // message fields
    hot_knock: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Knock {}

impl Knock {
    pub fn new() -> Knock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Knock {
        static mut instance: ::protobuf::lazy::Lazy<Knock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Knock,
        };
        unsafe {
            instance.get(Knock::new)
        }
    }

    // required bool hot_knock = 1;

    pub fn clear_hot_knock(&mut self) {
        self.hot_knock = ::std::option::Option::None;
    }

    pub fn has_hot_knock(&self) -> bool {
        self.hot_knock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hot_knock(&mut self, v: bool) {
        self.hot_knock = ::std::option::Option::Some(v);
    }

    pub fn get_hot_knock(&self) -> bool {
        self.hot_knock.unwrap_or(false)
    }

    fn get_hot_knock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hot_knock
    }

    fn mut_hot_knock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hot_knock
    }
}

impl ::protobuf::Message for Knock {
    fn is_initialized(&self) -> bool {
        if self.hot_knock.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.hot_knock = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.hot_knock {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hot_knock {
            os.write_bool(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Knock {
    fn new() -> Knock {
        Knock::new()
    }

    fn descriptor_static(_: ::std::option::Option<Knock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hot_knock",
                    Knock::get_hot_knock_for_reflect,
                    Knock::mut_hot_knock_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Knock>(
                    "Knock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Knock {
    fn clear(&mut self) {
        self.clear_hot_knock();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Knock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Knock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LinkPreview {
    // message fields
    url: ::protobuf::SingularField<::std::string::String>,
    url_offset: ::std::option::Option<i32>,
    permanent_url: ::protobuf::SingularField<::std::string::String>,
    title: ::protobuf::SingularField<::std::string::String>,
    summary: ::protobuf::SingularField<::std::string::String>,
    image: ::protobuf::SingularPtrField<Asset>,
    // message oneof groups
    preview: ::std::option::Option<LinkPreview_oneof_preview>,
    meta_data: ::std::option::Option<LinkPreview_oneof_meta_data>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinkPreview {}

#[derive(Clone,PartialEq)]
pub enum LinkPreview_oneof_preview {
    article(Article),
}

#[derive(Clone,PartialEq)]
pub enum LinkPreview_oneof_meta_data {
    tweet(Tweet),
}

impl LinkPreview {
    pub fn new() -> LinkPreview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinkPreview {
        static mut instance: ::protobuf::lazy::Lazy<LinkPreview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinkPreview,
        };
        unsafe {
            instance.get(LinkPreview::new)
        }
    }

    // required string url = 1;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // required int32 url_offset = 2;

    pub fn clear_url_offset(&mut self) {
        self.url_offset = ::std::option::Option::None;
    }

    pub fn has_url_offset(&self) -> bool {
        self.url_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url_offset(&mut self, v: i32) {
        self.url_offset = ::std::option::Option::Some(v);
    }

    pub fn get_url_offset(&self) -> i32 {
        self.url_offset.unwrap_or(0)
    }

    fn get_url_offset_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.url_offset
    }

    fn mut_url_offset_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.url_offset
    }

    // optional .Article article = 3;

    pub fn clear_article(&mut self) {
        self.preview = ::std::option::Option::None;
    }

    pub fn has_article(&self) -> bool {
        match self.preview {
            ::std::option::Option::Some(LinkPreview_oneof_preview::article(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_article(&mut self, v: Article) {
        self.preview = ::std::option::Option::Some(LinkPreview_oneof_preview::article(v))
    }

    // Mutable pointer to the field.
    pub fn mut_article(&mut self) -> &mut Article {
        if let ::std::option::Option::Some(LinkPreview_oneof_preview::article(_)) = self.preview {
        } else {
            self.preview = ::std::option::Option::Some(LinkPreview_oneof_preview::article(Article::new()));
        }
        match self.preview {
            ::std::option::Option::Some(LinkPreview_oneof_preview::article(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_article(&mut self) -> Article {
        if self.has_article() {
            match self.preview.take() {
                ::std::option::Option::Some(LinkPreview_oneof_preview::article(v)) => v,
                _ => panic!(),
            }
        } else {
            Article::new()
        }
    }

    pub fn get_article(&self) -> &Article {
        match self.preview {
            ::std::option::Option::Some(LinkPreview_oneof_preview::article(ref v)) => v,
            _ => Article::default_instance(),
        }
    }

    // optional string permanent_url = 5;

    pub fn clear_permanent_url(&mut self) {
        self.permanent_url.clear();
    }

    pub fn has_permanent_url(&self) -> bool {
        self.permanent_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permanent_url(&mut self, v: ::std::string::String) {
        self.permanent_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permanent_url(&mut self) -> &mut ::std::string::String {
        if self.permanent_url.is_none() {
            self.permanent_url.set_default();
        };
        self.permanent_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_permanent_url(&mut self) -> ::std::string::String {
        self.permanent_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_permanent_url(&self) -> &str {
        match self.permanent_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_permanent_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.permanent_url
    }

    fn mut_permanent_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.permanent_url
    }

    // optional string title = 6;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        };
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string summary = 7;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        if self.summary.is_none() {
            self.summary.set_default();
        };
        self.summary.as_mut().unwrap()
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        self.summary.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        match self.summary.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_summary_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.summary
    }

    // optional .Asset image = 8;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: Asset) {
        self.image = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut Asset {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> Asset {
        self.image.take().unwrap_or_else(|| Asset::new())
    }

    pub fn get_image(&self) -> &Asset {
        self.image.as_ref().unwrap_or_else(|| Asset::default_instance())
    }

    fn get_image_for_reflect(&self) -> &::protobuf::SingularPtrField<Asset> {
        &self.image
    }

    fn mut_image_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Asset> {
        &mut self.image
    }

    // optional .Tweet tweet = 9;

    pub fn clear_tweet(&mut self) {
        self.meta_data = ::std::option::Option::None;
    }

    pub fn has_tweet(&self) -> bool {
        match self.meta_data {
            ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tweet(&mut self, v: Tweet) {
        self.meta_data = ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tweet(&mut self) -> &mut Tweet {
        if let ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(_)) = self.meta_data {
        } else {
            self.meta_data = ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(Tweet::new()));
        }
        match self.meta_data {
            ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tweet(&mut self) -> Tweet {
        if self.has_tweet() {
            match self.meta_data.take() {
                ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(v)) => v,
                _ => panic!(),
            }
        } else {
            Tweet::new()
        }
    }

    pub fn get_tweet(&self) -> &Tweet {
        match self.meta_data {
            ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(ref v)) => v,
            _ => Tweet::default_instance(),
        }
    }
}

impl ::protobuf::Message for LinkPreview {
    fn is_initialized(&self) -> bool {
        if self.url.is_none() {
            return false;
        };
        if self.url_offset.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.url_offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.preview = ::std::option::Option::Some(LinkPreview_oneof_preview::article(is.read_message()?));
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.permanent_url)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.summary)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.image)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.meta_data = ::std::option::Option::Some(LinkPreview_oneof_meta_data::tweet(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.url_offset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.permanent_url.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.summary.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.image.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.preview {
            match v {
                &LinkPreview_oneof_preview::article(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &LinkPreview_oneof_meta_data::tweet(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.url.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.url_offset {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.permanent_url.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.title.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.summary.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.image.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.preview {
            match v {
                &LinkPreview_oneof_preview::article(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &LinkPreview_oneof_meta_data::tweet(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinkPreview {
    fn new() -> LinkPreview {
        LinkPreview::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinkPreview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    LinkPreview::get_url_for_reflect,
                    LinkPreview::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "url_offset",
                    LinkPreview::get_url_offset_for_reflect,
                    LinkPreview::mut_url_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Article>(
                    "article",
                    LinkPreview::has_article,
                    LinkPreview::get_article,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "permanent_url",
                    LinkPreview::get_permanent_url_for_reflect,
                    LinkPreview::mut_permanent_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    LinkPreview::get_title_for_reflect,
                    LinkPreview::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    LinkPreview::get_summary_for_reflect,
                    LinkPreview::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Asset>>(
                    "image",
                    LinkPreview::get_image_for_reflect,
                    LinkPreview::mut_image_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Tweet>(
                    "tweet",
                    LinkPreview::has_tweet,
                    LinkPreview::get_tweet,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinkPreview>(
                    "LinkPreview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinkPreview {
    fn clear(&mut self) {
        self.clear_url();
        self.clear_url_offset();
        self.clear_article();
        self.clear_permanent_url();
        self.clear_title();
        self.clear_summary();
        self.clear_image();
        self.clear_tweet();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LinkPreview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LinkPreview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tweet {
    // message fields
    author: ::protobuf::SingularField<::std::string::String>,
    username: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tweet {}

impl Tweet {
    pub fn new() -> Tweet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tweet {
        static mut instance: ::protobuf::lazy::Lazy<Tweet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tweet,
        };
        unsafe {
            instance.get(Tweet::new)
        }
    }

    // optional string author = 1;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    pub fn has_author(&self) -> bool {
        self.author.is_some()
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author(&mut self) -> &mut ::std::string::String {
        if self.author.is_none() {
            self.author.set_default();
        };
        self.author.as_mut().unwrap()
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        self.author.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_author(&self) -> &str {
        match self.author.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_author_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.author
    }

    fn mut_author_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.author
    }

    // optional string username = 2;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        };
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.username
    }
}

impl ::protobuf::Message for Tweet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.author)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.author.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.username.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.author.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.username.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tweet {
    fn new() -> Tweet {
        Tweet::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tweet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "author",
                    Tweet::get_author_for_reflect,
                    Tweet::mut_author_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    Tweet::get_username_for_reflect,
                    Tweet::mut_username_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tweet>(
                    "Tweet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tweet {
    fn clear(&mut self) {
        self.clear_author();
        self.clear_username();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tweet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tweet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Article {
    // message fields
    permanent_url: ::protobuf::SingularField<::std::string::String>,
    title: ::protobuf::SingularField<::std::string::String>,
    summary: ::protobuf::SingularField<::std::string::String>,
    image: ::protobuf::SingularPtrField<Asset>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Article {}

impl Article {
    pub fn new() -> Article {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Article {
        static mut instance: ::protobuf::lazy::Lazy<Article> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Article,
        };
        unsafe {
            instance.get(Article::new)
        }
    }

    // required string permanent_url = 1;

    pub fn clear_permanent_url(&mut self) {
        self.permanent_url.clear();
    }

    pub fn has_permanent_url(&self) -> bool {
        self.permanent_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permanent_url(&mut self, v: ::std::string::String) {
        self.permanent_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permanent_url(&mut self) -> &mut ::std::string::String {
        if self.permanent_url.is_none() {
            self.permanent_url.set_default();
        };
        self.permanent_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_permanent_url(&mut self) -> ::std::string::String {
        self.permanent_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_permanent_url(&self) -> &str {
        match self.permanent_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_permanent_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.permanent_url
    }

    fn mut_permanent_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.permanent_url
    }

    // optional string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        };
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string summary = 3;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        if self.summary.is_none() {
            self.summary.set_default();
        };
        self.summary.as_mut().unwrap()
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        self.summary.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        match self.summary.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_summary_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.summary
    }

    // optional .Asset image = 4;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: Asset) {
        self.image = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut Asset {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> Asset {
        self.image.take().unwrap_or_else(|| Asset::new())
    }

    pub fn get_image(&self) -> &Asset {
        self.image.as_ref().unwrap_or_else(|| Asset::default_instance())
    }

    fn get_image_for_reflect(&self) -> &::protobuf::SingularPtrField<Asset> {
        &self.image
    }

    fn mut_image_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Asset> {
        &mut self.image
    }
}

impl ::protobuf::Message for Article {
    fn is_initialized(&self) -> bool {
        if self.permanent_url.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.permanent_url)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.summary)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.image)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.permanent_url.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.summary.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.image.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.permanent_url.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.title.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.summary.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.image.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Article {
    fn new() -> Article {
        Article::new()
    }

    fn descriptor_static(_: ::std::option::Option<Article>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "permanent_url",
                    Article::get_permanent_url_for_reflect,
                    Article::mut_permanent_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    Article::get_title_for_reflect,
                    Article::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    Article::get_summary_for_reflect,
                    Article::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Asset>>(
                    "image",
                    Article::get_image_for_reflect,
                    Article::mut_image_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Article>(
                    "Article",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Article {
    fn clear(&mut self) {
        self.clear_permanent_url();
        self.clear_title();
        self.clear_summary();
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Article {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Article {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mention {
    // message fields
    user_id: ::protobuf::SingularField<::std::string::String>,
    user_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mention {}

impl Mention {
    pub fn new() -> Mention {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mention {
        static mut instance: ::protobuf::lazy::Lazy<Mention> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mention,
        };
        unsafe {
            instance.get(Mention::new)
        }
    }

    // required string user_id = 1;

    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        if self.user_id.is_none() {
            self.user_id.set_default();
        };
        self.user_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        self.user_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user_id(&self) -> &str {
        match self.user_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_user_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.user_id
    }

    // required string user_name = 2;

    pub fn clear_user_name(&mut self) {
        self.user_name.clear();
    }

    pub fn has_user_name(&self) -> bool {
        self.user_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_name(&mut self, v: ::std::string::String) {
        self.user_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_name(&mut self) -> &mut ::std::string::String {
        if self.user_name.is_none() {
            self.user_name.set_default();
        };
        self.user_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_user_name(&mut self) -> ::std::string::String {
        self.user_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user_name(&self) -> &str {
        match self.user_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_user_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.user_name
    }

    fn mut_user_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.user_name
    }
}

impl ::protobuf::Message for Mention {
    fn is_initialized(&self) -> bool {
        if self.user_id.is_none() {
            return false;
        };
        if self.user_name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.user_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.user_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.user_name.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mention {
    fn new() -> Mention {
        Mention::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mention>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user_id",
                    Mention::get_user_id_for_reflect,
                    Mention::mut_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user_name",
                    Mention::get_user_name_for_reflect,
                    Mention::mut_user_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mention>(
                    "Mention",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mention {
    fn clear(&mut self) {
        self.clear_user_id();
        self.clear_user_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mention {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LastRead {
    // message fields
    conversation_id: ::protobuf::SingularField<::std::string::String>,
    last_read_timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LastRead {}

impl LastRead {
    pub fn new() -> LastRead {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LastRead {
        static mut instance: ::protobuf::lazy::Lazy<LastRead> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LastRead,
        };
        unsafe {
            instance.get(LastRead::new)
        }
    }

    // required string conversation_id = 1;

    pub fn clear_conversation_id(&mut self) {
        self.conversation_id.clear();
    }

    pub fn has_conversation_id(&self) -> bool {
        self.conversation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conversation_id(&mut self, v: ::std::string::String) {
        self.conversation_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conversation_id(&mut self) -> &mut ::std::string::String {
        if self.conversation_id.is_none() {
            self.conversation_id.set_default();
        };
        self.conversation_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_conversation_id(&mut self) -> ::std::string::String {
        self.conversation_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_conversation_id(&self) -> &str {
        match self.conversation_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_conversation_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.conversation_id
    }

    fn mut_conversation_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.conversation_id
    }

    // required int64 last_read_timestamp = 2;

    pub fn clear_last_read_timestamp(&mut self) {
        self.last_read_timestamp = ::std::option::Option::None;
    }

    pub fn has_last_read_timestamp(&self) -> bool {
        self.last_read_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_read_timestamp(&mut self, v: i64) {
        self.last_read_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_last_read_timestamp(&self) -> i64 {
        self.last_read_timestamp.unwrap_or(0)
    }

    fn get_last_read_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_read_timestamp
    }

    fn mut_last_read_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_read_timestamp
    }
}

impl ::protobuf::Message for LastRead {
    fn is_initialized(&self) -> bool {
        if self.conversation_id.is_none() {
            return false;
        };
        if self.last_read_timestamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.conversation_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.last_read_timestamp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.conversation_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.last_read_timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conversation_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.last_read_timestamp {
            os.write_int64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LastRead {
    fn new() -> LastRead {
        LastRead::new()
    }

    fn descriptor_static(_: ::std::option::Option<LastRead>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "conversation_id",
                    LastRead::get_conversation_id_for_reflect,
                    LastRead::mut_conversation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_read_timestamp",
                    LastRead::get_last_read_timestamp_for_reflect,
                    LastRead::mut_last_read_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LastRead>(
                    "LastRead",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LastRead {
    fn clear(&mut self) {
        self.clear_conversation_id();
        self.clear_last_read_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LastRead {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LastRead {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cleared {
    // message fields
    conversation_id: ::protobuf::SingularField<::std::string::String>,
    cleared_timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cleared {}

impl Cleared {
    pub fn new() -> Cleared {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cleared {
        static mut instance: ::protobuf::lazy::Lazy<Cleared> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cleared,
        };
        unsafe {
            instance.get(Cleared::new)
        }
    }

    // required string conversation_id = 1;

    pub fn clear_conversation_id(&mut self) {
        self.conversation_id.clear();
    }

    pub fn has_conversation_id(&self) -> bool {
        self.conversation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conversation_id(&mut self, v: ::std::string::String) {
        self.conversation_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conversation_id(&mut self) -> &mut ::std::string::String {
        if self.conversation_id.is_none() {
            self.conversation_id.set_default();
        };
        self.conversation_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_conversation_id(&mut self) -> ::std::string::String {
        self.conversation_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_conversation_id(&self) -> &str {
        match self.conversation_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_conversation_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.conversation_id
    }

    fn mut_conversation_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.conversation_id
    }

    // required int64 cleared_timestamp = 2;

    pub fn clear_cleared_timestamp(&mut self) {
        self.cleared_timestamp = ::std::option::Option::None;
    }

    pub fn has_cleared_timestamp(&self) -> bool {
        self.cleared_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cleared_timestamp(&mut self, v: i64) {
        self.cleared_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_cleared_timestamp(&self) -> i64 {
        self.cleared_timestamp.unwrap_or(0)
    }

    fn get_cleared_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.cleared_timestamp
    }

    fn mut_cleared_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.cleared_timestamp
    }
}

impl ::protobuf::Message for Cleared {
    fn is_initialized(&self) -> bool {
        if self.conversation_id.is_none() {
            return false;
        };
        if self.cleared_timestamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.conversation_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.cleared_timestamp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.conversation_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.cleared_timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conversation_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.cleared_timestamp {
            os.write_int64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Cleared {
    fn new() -> Cleared {
        Cleared::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cleared>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "conversation_id",
                    Cleared::get_conversation_id_for_reflect,
                    Cleared::mut_conversation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "cleared_timestamp",
                    Cleared::get_cleared_timestamp_for_reflect,
                    Cleared::mut_cleared_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cleared>(
                    "Cleared",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cleared {
    fn clear(&mut self) {
        self.clear_conversation_id();
        self.clear_cleared_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cleared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cleared {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageHide {
    // message fields
    conversation_id: ::protobuf::SingularField<::std::string::String>,
    message_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageHide {}

impl MessageHide {
    pub fn new() -> MessageHide {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageHide {
        static mut instance: ::protobuf::lazy::Lazy<MessageHide> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageHide,
        };
        unsafe {
            instance.get(MessageHide::new)
        }
    }

    // required string conversation_id = 1;

    pub fn clear_conversation_id(&mut self) {
        self.conversation_id.clear();
    }

    pub fn has_conversation_id(&self) -> bool {
        self.conversation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conversation_id(&mut self, v: ::std::string::String) {
        self.conversation_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conversation_id(&mut self) -> &mut ::std::string::String {
        if self.conversation_id.is_none() {
            self.conversation_id.set_default();
        };
        self.conversation_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_conversation_id(&mut self) -> ::std::string::String {
        self.conversation_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_conversation_id(&self) -> &str {
        match self.conversation_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_conversation_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.conversation_id
    }

    fn mut_conversation_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.conversation_id
    }

    // required string message_id = 2;

    pub fn clear_message_id(&mut self) {
        self.message_id.clear();
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: ::std::string::String) {
        self.message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&mut self) -> &mut ::std::string::String {
        if self.message_id.is_none() {
            self.message_id.set_default();
        };
        self.message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_id(&mut self) -> ::std::string::String {
        self.message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_id(&self) -> &str {
        match self.message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message_id
    }

    fn mut_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message_id
    }
}

impl ::protobuf::Message for MessageHide {
    fn is_initialized(&self) -> bool {
        if self.conversation_id.is_none() {
            return false;
        };
        if self.message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.conversation_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.conversation_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conversation_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.message_id.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageHide {
    fn new() -> MessageHide {
        MessageHide::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageHide>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "conversation_id",
                    MessageHide::get_conversation_id_for_reflect,
                    MessageHide::mut_conversation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    MessageHide::get_message_id_for_reflect,
                    MessageHide::mut_message_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageHide>(
                    "MessageHide",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageHide {
    fn clear(&mut self) {
        self.clear_conversation_id();
        self.clear_message_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageHide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageHide {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageDelete {
    // message fields
    message_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageDelete {}

impl MessageDelete {
    pub fn new() -> MessageDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageDelete {
        static mut instance: ::protobuf::lazy::Lazy<MessageDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageDelete,
        };
        unsafe {
            instance.get(MessageDelete::new)
        }
    }

    // required string message_id = 1;

    pub fn clear_message_id(&mut self) {
        self.message_id.clear();
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: ::std::string::String) {
        self.message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&mut self) -> &mut ::std::string::String {
        if self.message_id.is_none() {
            self.message_id.set_default();
        };
        self.message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_id(&mut self) -> ::std::string::String {
        self.message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_id(&self) -> &str {
        match self.message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message_id
    }

    fn mut_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message_id
    }
}

impl ::protobuf::Message for MessageDelete {
    fn is_initialized(&self) -> bool {
        if self.message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_id.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageDelete {
    fn new() -> MessageDelete {
        MessageDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    MessageDelete::get_message_id_for_reflect,
                    MessageDelete::mut_message_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageDelete>(
                    "MessageDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageDelete {
    fn clear(&mut self) {
        self.clear_message_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageEdit {
    // message fields
    replacing_message_id: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    content: ::std::option::Option<MessageEdit_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageEdit {}

#[derive(Clone,PartialEq)]
pub enum MessageEdit_oneof_content {
    text(Text),
}

impl MessageEdit {
    pub fn new() -> MessageEdit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageEdit {
        static mut instance: ::protobuf::lazy::Lazy<MessageEdit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageEdit,
        };
        unsafe {
            instance.get(MessageEdit::new)
        }
    }

    // required string replacing_message_id = 1;

    pub fn clear_replacing_message_id(&mut self) {
        self.replacing_message_id.clear();
    }

    pub fn has_replacing_message_id(&self) -> bool {
        self.replacing_message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replacing_message_id(&mut self, v: ::std::string::String) {
        self.replacing_message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replacing_message_id(&mut self) -> &mut ::std::string::String {
        if self.replacing_message_id.is_none() {
            self.replacing_message_id.set_default();
        };
        self.replacing_message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_replacing_message_id(&mut self) -> ::std::string::String {
        self.replacing_message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_replacing_message_id(&self) -> &str {
        match self.replacing_message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_replacing_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.replacing_message_id
    }

    fn mut_replacing_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.replacing_message_id
    }

    // optional .Text text = 2;

    pub fn clear_text(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_text(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(MessageEdit_oneof_content::text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: Text) {
        self.content = ::std::option::Option::Some(MessageEdit_oneof_content::text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut Text {
        if let ::std::option::Option::Some(MessageEdit_oneof_content::text(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(MessageEdit_oneof_content::text(Text::new()));
        }
        match self.content {
            ::std::option::Option::Some(MessageEdit_oneof_content::text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text(&mut self) -> Text {
        if self.has_text() {
            match self.content.take() {
                ::std::option::Option::Some(MessageEdit_oneof_content::text(v)) => v,
                _ => panic!(),
            }
        } else {
            Text::new()
        }
    }

    pub fn get_text(&self) -> &Text {
        match self.content {
            ::std::option::Option::Some(MessageEdit_oneof_content::text(ref v)) => v,
            _ => Text::default_instance(),
        }
    }
}

impl ::protobuf::Message for MessageEdit {
    fn is_initialized(&self) -> bool {
        if self.replacing_message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.replacing_message_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(MessageEdit_oneof_content::text(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.replacing_message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &MessageEdit_oneof_content::text(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replacing_message_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &MessageEdit_oneof_content::text(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageEdit {
    fn new() -> MessageEdit {
        MessageEdit::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageEdit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "replacing_message_id",
                    MessageEdit::get_replacing_message_id_for_reflect,
                    MessageEdit::mut_replacing_message_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Text>(
                    "text",
                    MessageEdit::has_text,
                    MessageEdit::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageEdit>(
                    "MessageEdit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageEdit {
    fn clear(&mut self) {
        self.clear_replacing_message_id();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageEdit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageEdit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Confirmation {
    // message fields
    field_type: ::std::option::Option<Confirmation_Type>,
    first_message_id: ::protobuf::SingularField<::std::string::String>,
    more_message_ids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Confirmation {}

impl Confirmation {
    pub fn new() -> Confirmation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Confirmation {
        static mut instance: ::protobuf::lazy::Lazy<Confirmation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Confirmation,
        };
        unsafe {
            instance.get(Confirmation::new)
        }
    }

    // required .Confirmation.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Confirmation_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Confirmation_Type {
        self.field_type.unwrap_or(Confirmation_Type::DELIVERED)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Confirmation_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Confirmation_Type> {
        &mut self.field_type
    }

    // required string first_message_id = 1;

    pub fn clear_first_message_id(&mut self) {
        self.first_message_id.clear();
    }

    pub fn has_first_message_id(&self) -> bool {
        self.first_message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_message_id(&mut self, v: ::std::string::String) {
        self.first_message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first_message_id(&mut self) -> &mut ::std::string::String {
        if self.first_message_id.is_none() {
            self.first_message_id.set_default();
        };
        self.first_message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_first_message_id(&mut self) -> ::std::string::String {
        self.first_message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_first_message_id(&self) -> &str {
        match self.first_message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_first_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.first_message_id
    }

    fn mut_first_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.first_message_id
    }

    // repeated string more_message_ids = 3;

    pub fn clear_more_message_ids(&mut self) {
        self.more_message_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_more_message_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.more_message_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_more_message_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.more_message_ids
    }

    // Take field
    pub fn take_more_message_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.more_message_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_more_message_ids(&self) -> &[::std::string::String] {
        &self.more_message_ids
    }

    fn get_more_message_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.more_message_ids
    }

    fn mut_more_message_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.more_message_ids
    }
}

impl ::protobuf::Message for Confirmation {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.first_message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.first_message_id)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.more_message_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.first_message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.more_message_ids {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.first_message_id.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.more_message_ids {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Confirmation {
    fn new() -> Confirmation {
        Confirmation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Confirmation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Confirmation_Type>>(
                    "type",
                    Confirmation::get_field_type_for_reflect,
                    Confirmation::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "first_message_id",
                    Confirmation::get_first_message_id_for_reflect,
                    Confirmation::mut_first_message_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "more_message_ids",
                    Confirmation::get_more_message_ids_for_reflect,
                    Confirmation::mut_more_message_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Confirmation>(
                    "Confirmation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Confirmation {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_first_message_id();
        self.clear_more_message_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Confirmation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Confirmation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Confirmation_Type {
    DELIVERED = 0,
    READ = 1,
}

impl ::protobuf::ProtobufEnum for Confirmation_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Confirmation_Type> {
        match value {
            0 => ::std::option::Option::Some(Confirmation_Type::DELIVERED),
            1 => ::std::option::Option::Some(Confirmation_Type::READ),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Confirmation_Type] = &[
            Confirmation_Type::DELIVERED,
            Confirmation_Type::READ,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Confirmation_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Confirmation_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Confirmation_Type {
}

impl ::protobuf::reflect::ProtobufValue for Confirmation_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Location {
    // message fields
    longitude: ::std::option::Option<f32>,
    latitude: ::std::option::Option<f32>,
    name: ::protobuf::SingularField<::std::string::String>,
    zoom: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Location {}

impl Location {
    pub fn new() -> Location {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Location {
        static mut instance: ::protobuf::lazy::Lazy<Location> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Location,
        };
        unsafe {
            instance.get(Location::new)
        }
    }

    // required float longitude = 1;

    pub fn clear_longitude(&mut self) {
        self.longitude = ::std::option::Option::None;
    }

    pub fn has_longitude(&self) -> bool {
        self.longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f32) {
        self.longitude = ::std::option::Option::Some(v);
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude.unwrap_or(0.)
    }

    fn get_longitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.longitude
    }

    fn mut_longitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.longitude
    }

    // required float latitude = 2;

    pub fn clear_latitude(&mut self) {
        self.latitude = ::std::option::Option::None;
    }

    pub fn has_latitude(&self) -> bool {
        self.latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f32) {
        self.latitude = ::std::option::Option::Some(v);
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude.unwrap_or(0.)
    }

    fn get_latitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.latitude
    }

    fn mut_latitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.latitude
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 zoom = 4;

    pub fn clear_zoom(&mut self) {
        self.zoom = ::std::option::Option::None;
    }

    pub fn has_zoom(&self) -> bool {
        self.zoom.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zoom(&mut self, v: i32) {
        self.zoom = ::std::option::Option::Some(v);
    }

    pub fn get_zoom(&self) -> i32 {
        self.zoom.unwrap_or(0)
    }

    fn get_zoom_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.zoom
    }

    fn mut_zoom_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.zoom
    }
}

impl ::protobuf::Message for Location {
    fn is_initialized(&self) -> bool {
        if self.longitude.is_none() {
            return false;
        };
        if self.latitude.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.zoom = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.longitude {
            my_size += 5;
        };
        if let Some(v) = self.latitude {
            my_size += 5;
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.zoom {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.longitude {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.latitude {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.zoom {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Location {
    fn new() -> Location {
        Location::new()
    }

    fn descriptor_static(_: ::std::option::Option<Location>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "longitude",
                    Location::get_longitude_for_reflect,
                    Location::mut_longitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "latitude",
                    Location::get_latitude_for_reflect,
                    Location::mut_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Location::get_name_for_reflect,
                    Location::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "zoom",
                    Location::get_zoom_for_reflect,
                    Location::mut_zoom_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Location>(
                    "Location",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Location {
    fn clear(&mut self) {
        self.clear_longitude();
        self.clear_latitude();
        self.clear_name();
        self.clear_zoom();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Location {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageAsset {
    // message fields
    tag: ::protobuf::SingularField<::std::string::String>,
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    original_width: ::std::option::Option<i32>,
    original_height: ::std::option::Option<i32>,
    mime_type: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<i32>,
    otr_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    mac_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    mac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sha256: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageAsset {}

impl ImageAsset {
    pub fn new() -> ImageAsset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageAsset {
        static mut instance: ::protobuf::lazy::Lazy<ImageAsset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageAsset,
        };
        unsafe {
            instance.get(ImageAsset::new)
        }
    }

    // required string tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }

    // required int32 width = 2;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.width
    }

    // required int32 height = 3;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    fn get_height_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.height
    }

    // required int32 original_width = 4;

    pub fn clear_original_width(&mut self) {
        self.original_width = ::std::option::Option::None;
    }

    pub fn has_original_width(&self) -> bool {
        self.original_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_width(&mut self, v: i32) {
        self.original_width = ::std::option::Option::Some(v);
    }

    pub fn get_original_width(&self) -> i32 {
        self.original_width.unwrap_or(0)
    }

    fn get_original_width_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.original_width
    }

    fn mut_original_width_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.original_width
    }

    // required int32 original_height = 5;

    pub fn clear_original_height(&mut self) {
        self.original_height = ::std::option::Option::None;
    }

    pub fn has_original_height(&self) -> bool {
        self.original_height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_height(&mut self, v: i32) {
        self.original_height = ::std::option::Option::Some(v);
    }

    pub fn get_original_height(&self) -> i32 {
        self.original_height.unwrap_or(0)
    }

    fn get_original_height_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.original_height
    }

    fn mut_original_height_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.original_height
    }

    // required string mime_type = 6;

    pub fn clear_mime_type(&mut self) {
        self.mime_type.clear();
    }

    pub fn has_mime_type(&self) -> bool {
        self.mime_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mime_type(&mut self, v: ::std::string::String) {
        self.mime_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mime_type(&mut self) -> &mut ::std::string::String {
        if self.mime_type.is_none() {
            self.mime_type.set_default();
        };
        self.mime_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_mime_type(&mut self) -> ::std::string::String {
        self.mime_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mime_type(&self) -> &str {
        match self.mime_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mime_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mime_type
    }

    fn mut_mime_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mime_type
    }

    // required int32 size = 7;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> i32 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.size
    }

    // optional bytes otr_key = 8;

    pub fn clear_otr_key(&mut self) {
        self.otr_key.clear();
    }

    pub fn has_otr_key(&self) -> bool {
        self.otr_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_otr_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.otr_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_otr_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.otr_key.is_none() {
            self.otr_key.set_default();
        };
        self.otr_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_otr_key(&mut self) -> ::std::vec::Vec<u8> {
        self.otr_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_otr_key(&self) -> &[u8] {
        match self.otr_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_otr_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.otr_key
    }

    fn mut_otr_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.otr_key
    }

    // optional bytes mac_key = 9;

    pub fn clear_mac_key(&mut self) {
        self.mac_key.clear();
    }

    pub fn has_mac_key(&self) -> bool {
        self.mac_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mac_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.mac_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.mac_key.is_none() {
            self.mac_key.set_default();
        };
        self.mac_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_mac_key(&mut self) -> ::std::vec::Vec<u8> {
        self.mac_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_mac_key(&self) -> &[u8] {
        match self.mac_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_mac_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.mac_key
    }

    fn mut_mac_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.mac_key
    }

    // optional bytes mac = 10;

    pub fn clear_mac(&mut self) {
        self.mac.clear();
    }

    pub fn has_mac(&self) -> bool {
        self.mac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mac(&mut self, v: ::std::vec::Vec<u8>) {
        self.mac = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.mac.is_none() {
            self.mac.set_default();
        };
        self.mac.as_mut().unwrap()
    }

    // Take field
    pub fn take_mac(&mut self) -> ::std::vec::Vec<u8> {
        self.mac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_mac(&self) -> &[u8] {
        match self.mac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_mac_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.mac
    }

    fn mut_mac_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.mac
    }

    // optional bytes sha256 = 11;

    pub fn clear_sha256(&mut self) {
        self.sha256.clear();
    }

    pub fn has_sha256(&self) -> bool {
        self.sha256.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha256(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha256 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha256(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha256.is_none() {
            self.sha256.set_default();
        };
        self.sha256.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha256(&mut self) -> ::std::vec::Vec<u8> {
        self.sha256.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sha256(&self) -> &[u8] {
        match self.sha256.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_sha256_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.sha256
    }

    fn mut_sha256_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.sha256
    }
}

impl ::protobuf::Message for ImageAsset {
    fn is_initialized(&self) -> bool {
        if self.tag.is_none() {
            return false;
        };
        if self.width.is_none() {
            return false;
        };
        if self.height.is_none() {
            return false;
        };
        if self.original_width.is_none() {
            return false;
        };
        if self.original_height.is_none() {
            return false;
        };
        if self.mime_type.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.original_width = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.original_height = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mime_type)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.otr_key)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.mac_key)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.mac)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha256)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.width {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.height {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.original_width {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.original_height {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.mime_type.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.otr_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        };
        if let Some(v) = self.mac_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        };
        if let Some(v) = self.mac.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.sha256.as_ref() {
            my_size += ::protobuf::rt::bytes_size(11, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tag.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.width {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.height {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.original_width {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.original_height {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.mime_type.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.size {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.otr_key.as_ref() {
            os.write_bytes(8, &v)?;
        };
        if let Some(v) = self.mac_key.as_ref() {
            os.write_bytes(9, &v)?;
        };
        if let Some(v) = self.mac.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.sha256.as_ref() {
            os.write_bytes(11, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ImageAsset {
    fn new() -> ImageAsset {
        ImageAsset::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageAsset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    ImageAsset::get_tag_for_reflect,
                    ImageAsset::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "width",
                    ImageAsset::get_width_for_reflect,
                    ImageAsset::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "height",
                    ImageAsset::get_height_for_reflect,
                    ImageAsset::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "original_width",
                    ImageAsset::get_original_width_for_reflect,
                    ImageAsset::mut_original_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "original_height",
                    ImageAsset::get_original_height_for_reflect,
                    ImageAsset::mut_original_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mime_type",
                    ImageAsset::get_mime_type_for_reflect,
                    ImageAsset::mut_mime_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "size",
                    ImageAsset::get_size_for_reflect,
                    ImageAsset::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "otr_key",
                    ImageAsset::get_otr_key_for_reflect,
                    ImageAsset::mut_otr_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "mac_key",
                    ImageAsset::get_mac_key_for_reflect,
                    ImageAsset::mut_mac_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "mac",
                    ImageAsset::get_mac_for_reflect,
                    ImageAsset::mut_mac_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha256",
                    ImageAsset::get_sha256_for_reflect,
                    ImageAsset::mut_sha256_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageAsset>(
                    "ImageAsset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageAsset {
    fn clear(&mut self) {
        self.clear_tag();
        self.clear_width();
        self.clear_height();
        self.clear_original_width();
        self.clear_original_height();
        self.clear_mime_type();
        self.clear_size();
        self.clear_otr_key();
        self.clear_mac_key();
        self.clear_mac();
        self.clear_sha256();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageAsset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageAsset {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset {
    // message fields
    original: ::protobuf::SingularPtrField<Asset_Original>,
    preview: ::protobuf::SingularPtrField<Asset_Preview>,
    // message oneof groups
    status: ::std::option::Option<Asset_oneof_status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset {}

#[derive(Clone,PartialEq)]
pub enum Asset_oneof_status {
    not_uploaded(Asset_NotUploaded),
    uploaded(Asset_RemoteData),
}

impl Asset {
    pub fn new() -> Asset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset {
        static mut instance: ::protobuf::lazy::Lazy<Asset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset,
        };
        unsafe {
            instance.get(Asset::new)
        }
    }

    // optional .Asset.Original original = 1;

    pub fn clear_original(&mut self) {
        self.original.clear();
    }

    pub fn has_original(&self) -> bool {
        self.original.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original(&mut self, v: Asset_Original) {
        self.original = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_original(&mut self) -> &mut Asset_Original {
        if self.original.is_none() {
            self.original.set_default();
        };
        self.original.as_mut().unwrap()
    }

    // Take field
    pub fn take_original(&mut self) -> Asset_Original {
        self.original.take().unwrap_or_else(|| Asset_Original::new())
    }

    pub fn get_original(&self) -> &Asset_Original {
        self.original.as_ref().unwrap_or_else(|| Asset_Original::default_instance())
    }

    fn get_original_for_reflect(&self) -> &::protobuf::SingularPtrField<Asset_Original> {
        &self.original
    }

    fn mut_original_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Asset_Original> {
        &mut self.original
    }

    // optional .Asset.NotUploaded not_uploaded = 3;

    pub fn clear_not_uploaded(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_not_uploaded(&self) -> bool {
        match self.status {
            ::std::option::Option::Some(Asset_oneof_status::not_uploaded(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_uploaded(&mut self, v: Asset_NotUploaded) {
        self.status = ::std::option::Option::Some(Asset_oneof_status::not_uploaded(v))
    }

    pub fn get_not_uploaded(&self) -> Asset_NotUploaded {
        match self.status {
            ::std::option::Option::Some(Asset_oneof_status::not_uploaded(v)) => v,
            _ => Asset_NotUploaded::CANCELLED,
        }
    }

    // optional .Asset.RemoteData uploaded = 4;

    pub fn clear_uploaded(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_uploaded(&self) -> bool {
        match self.status {
            ::std::option::Option::Some(Asset_oneof_status::uploaded(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uploaded(&mut self, v: Asset_RemoteData) {
        self.status = ::std::option::Option::Some(Asset_oneof_status::uploaded(v))
    }

    // Mutable pointer to the field.
    pub fn mut_uploaded(&mut self) -> &mut Asset_RemoteData {
        if let ::std::option::Option::Some(Asset_oneof_status::uploaded(_)) = self.status {
        } else {
            self.status = ::std::option::Option::Some(Asset_oneof_status::uploaded(Asset_RemoteData::new()));
        }
        match self.status {
            ::std::option::Option::Some(Asset_oneof_status::uploaded(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_uploaded(&mut self) -> Asset_RemoteData {
        if self.has_uploaded() {
            match self.status.take() {
                ::std::option::Option::Some(Asset_oneof_status::uploaded(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset_RemoteData::new()
        }
    }

    pub fn get_uploaded(&self) -> &Asset_RemoteData {
        match self.status {
            ::std::option::Option::Some(Asset_oneof_status::uploaded(ref v)) => v,
            _ => Asset_RemoteData::default_instance(),
        }
    }

    // optional .Asset.Preview preview = 5;

    pub fn clear_preview(&mut self) {
        self.preview.clear();
    }

    pub fn has_preview(&self) -> bool {
        self.preview.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preview(&mut self, v: Asset_Preview) {
        self.preview = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preview(&mut self) -> &mut Asset_Preview {
        if self.preview.is_none() {
            self.preview.set_default();
        };
        self.preview.as_mut().unwrap()
    }

    // Take field
    pub fn take_preview(&mut self) -> Asset_Preview {
        self.preview.take().unwrap_or_else(|| Asset_Preview::new())
    }

    pub fn get_preview(&self) -> &Asset_Preview {
        self.preview.as_ref().unwrap_or_else(|| Asset_Preview::default_instance())
    }

    fn get_preview_for_reflect(&self) -> &::protobuf::SingularPtrField<Asset_Preview> {
        &self.preview
    }

    fn mut_preview_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Asset_Preview> {
        &mut self.preview
    }
}

impl ::protobuf::Message for Asset {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.original)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.status = ::std::option::Option::Some(Asset_oneof_status::not_uploaded(is.read_enum()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.status = ::std::option::Option::Some(Asset_oneof_status::uploaded(is.read_message()?));
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.preview)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.original.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.preview.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.status {
            match v {
                &Asset_oneof_status::not_uploaded(v) => {
                    my_size += ::protobuf::rt::enum_size(3, v);
                },
                &Asset_oneof_status::uploaded(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.original.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.preview.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.status {
            match v {
                &Asset_oneof_status::not_uploaded(v) => {
                    os.write_enum(3, v.value())?;
                },
                &Asset_oneof_status::uploaded(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset {
    fn new() -> Asset {
        Asset::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Asset_Original>>(
                    "original",
                    Asset::get_original_for_reflect,
                    Asset::mut_original_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, Asset_NotUploaded>(
                    "not_uploaded",
                    Asset::has_not_uploaded,
                    Asset::get_not_uploaded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset_RemoteData>(
                    "uploaded",
                    Asset::has_uploaded,
                    Asset::get_uploaded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Asset_Preview>>(
                    "preview",
                    Asset::get_preview_for_reflect,
                    Asset::mut_preview_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset>(
                    "Asset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset {
    fn clear(&mut self) {
        self.clear_original();
        self.clear_not_uploaded();
        self.clear_uploaded();
        self.clear_preview();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_Original {
    // message fields
    mime_type: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularField<::std::string::String>,
    caption: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    meta_data: ::std::option::Option<Asset_Original_oneof_meta_data>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_Original {}

#[derive(Clone,PartialEq)]
pub enum Asset_Original_oneof_meta_data {
    image(Asset_ImageMetaData),
    video(Asset_VideoMetaData),
    audio(Asset_AudioMetaData),
}

impl Asset_Original {
    pub fn new() -> Asset_Original {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_Original {
        static mut instance: ::protobuf::lazy::Lazy<Asset_Original> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_Original,
        };
        unsafe {
            instance.get(Asset_Original::new)
        }
    }

    // required string mime_type = 1;

    pub fn clear_mime_type(&mut self) {
        self.mime_type.clear();
    }

    pub fn has_mime_type(&self) -> bool {
        self.mime_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mime_type(&mut self, v: ::std::string::String) {
        self.mime_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mime_type(&mut self) -> &mut ::std::string::String {
        if self.mime_type.is_none() {
            self.mime_type.set_default();
        };
        self.mime_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_mime_type(&mut self) -> ::std::string::String {
        self.mime_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mime_type(&self) -> &str {
        match self.mime_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mime_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mime_type
    }

    fn mut_mime_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mime_type
    }

    // required uint64 size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.size
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .Asset.ImageMetaData image = 4;

    pub fn clear_image(&mut self) {
        self.meta_data = ::std::option::Option::None;
    }

    pub fn has_image(&self) -> bool {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: Asset_ImageMetaData) {
        self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(v))
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut Asset_ImageMetaData {
        if let ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(_)) = self.meta_data {
        } else {
            self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(Asset_ImageMetaData::new()));
        }
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_image(&mut self) -> Asset_ImageMetaData {
        if self.has_image() {
            match self.meta_data.take() {
                ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset_ImageMetaData::new()
        }
    }

    pub fn get_image(&self) -> &Asset_ImageMetaData {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(ref v)) => v,
            _ => Asset_ImageMetaData::default_instance(),
        }
    }

    // optional .Asset.VideoMetaData video = 5;

    pub fn clear_video(&mut self) {
        self.meta_data = ::std::option::Option::None;
    }

    pub fn has_video(&self) -> bool {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_video(&mut self, v: Asset_VideoMetaData) {
        self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(v))
    }

    // Mutable pointer to the field.
    pub fn mut_video(&mut self) -> &mut Asset_VideoMetaData {
        if let ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(_)) = self.meta_data {
        } else {
            self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(Asset_VideoMetaData::new()));
        }
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_video(&mut self) -> Asset_VideoMetaData {
        if self.has_video() {
            match self.meta_data.take() {
                ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset_VideoMetaData::new()
        }
    }

    pub fn get_video(&self) -> &Asset_VideoMetaData {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(ref v)) => v,
            _ => Asset_VideoMetaData::default_instance(),
        }
    }

    // optional .Asset.AudioMetaData audio = 6;

    pub fn clear_audio(&mut self) {
        self.meta_data = ::std::option::Option::None;
    }

    pub fn has_audio(&self) -> bool {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_audio(&mut self, v: Asset_AudioMetaData) {
        self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(v))
    }

    // Mutable pointer to the field.
    pub fn mut_audio(&mut self) -> &mut Asset_AudioMetaData {
        if let ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(_)) = self.meta_data {
        } else {
            self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(Asset_AudioMetaData::new()));
        }
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_audio(&mut self) -> Asset_AudioMetaData {
        if self.has_audio() {
            match self.meta_data.take() {
                ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset_AudioMetaData::new()
        }
    }

    pub fn get_audio(&self) -> &Asset_AudioMetaData {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(ref v)) => v,
            _ => Asset_AudioMetaData::default_instance(),
        }
    }

    // optional string source = 7;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        self.source.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source(&self) -> &str {
        match self.source.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.source
    }

    // optional string caption = 8;

    pub fn clear_caption(&mut self) {
        self.caption.clear();
    }

    pub fn has_caption(&self) -> bool {
        self.caption.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caption(&mut self, v: ::std::string::String) {
        self.caption = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_caption(&mut self) -> &mut ::std::string::String {
        if self.caption.is_none() {
            self.caption.set_default();
        };
        self.caption.as_mut().unwrap()
    }

    // Take field
    pub fn take_caption(&mut self) -> ::std::string::String {
        self.caption.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_caption(&self) -> &str {
        match self.caption.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_caption_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.caption
    }

    fn mut_caption_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.caption
    }
}

impl ::protobuf::Message for Asset_Original {
    fn is_initialized(&self) -> bool {
        if self.mime_type.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mime_type)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::image(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::video(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.meta_data = ::std::option::Option::Some(Asset_Original_oneof_meta_data::audio(is.read_message()?));
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.caption)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.mime_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.source.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.caption.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &Asset_Original_oneof_meta_data::image(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Asset_Original_oneof_meta_data::video(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Asset_Original_oneof_meta_data::audio(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mime_type.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.size {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.source.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.caption.as_ref() {
            os.write_string(8, &v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &Asset_Original_oneof_meta_data::image(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Asset_Original_oneof_meta_data::video(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Asset_Original_oneof_meta_data::audio(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_Original {
    fn new() -> Asset_Original {
        Asset_Original::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_Original>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mime_type",
                    Asset_Original::get_mime_type_for_reflect,
                    Asset_Original::mut_mime_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    Asset_Original::get_size_for_reflect,
                    Asset_Original::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Asset_Original::get_name_for_reflect,
                    Asset_Original::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset_ImageMetaData>(
                    "image",
                    Asset_Original::has_image,
                    Asset_Original::get_image,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset_VideoMetaData>(
                    "video",
                    Asset_Original::has_video,
                    Asset_Original::get_video,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset_AudioMetaData>(
                    "audio",
                    Asset_Original::has_audio,
                    Asset_Original::get_audio,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    Asset_Original::get_source_for_reflect,
                    Asset_Original::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "caption",
                    Asset_Original::get_caption_for_reflect,
                    Asset_Original::mut_caption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_Original>(
                    "Asset_Original",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_Original {
    fn clear(&mut self) {
        self.clear_mime_type();
        self.clear_size();
        self.clear_name();
        self.clear_image();
        self.clear_video();
        self.clear_audio();
        self.clear_source();
        self.clear_caption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_Original {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_Original {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_Preview {
    // message fields
    mime_type: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<u64>,
    remote: ::protobuf::SingularPtrField<Asset_RemoteData>,
    // message oneof groups
    meta_data: ::std::option::Option<Asset_Preview_oneof_meta_data>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_Preview {}

#[derive(Clone,PartialEq)]
pub enum Asset_Preview_oneof_meta_data {
    image(Asset_ImageMetaData),
}

impl Asset_Preview {
    pub fn new() -> Asset_Preview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_Preview {
        static mut instance: ::protobuf::lazy::Lazy<Asset_Preview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_Preview,
        };
        unsafe {
            instance.get(Asset_Preview::new)
        }
    }

    // required string mime_type = 1;

    pub fn clear_mime_type(&mut self) {
        self.mime_type.clear();
    }

    pub fn has_mime_type(&self) -> bool {
        self.mime_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mime_type(&mut self, v: ::std::string::String) {
        self.mime_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mime_type(&mut self) -> &mut ::std::string::String {
        if self.mime_type.is_none() {
            self.mime_type.set_default();
        };
        self.mime_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_mime_type(&mut self) -> ::std::string::String {
        self.mime_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mime_type(&self) -> &str {
        match self.mime_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mime_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mime_type
    }

    fn mut_mime_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mime_type
    }

    // required uint64 size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.size
    }

    // optional .Asset.RemoteData remote = 3;

    pub fn clear_remote(&mut self) {
        self.remote.clear();
    }

    pub fn has_remote(&self) -> bool {
        self.remote.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remote(&mut self, v: Asset_RemoteData) {
        self.remote = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote(&mut self) -> &mut Asset_RemoteData {
        if self.remote.is_none() {
            self.remote.set_default();
        };
        self.remote.as_mut().unwrap()
    }

    // Take field
    pub fn take_remote(&mut self) -> Asset_RemoteData {
        self.remote.take().unwrap_or_else(|| Asset_RemoteData::new())
    }

    pub fn get_remote(&self) -> &Asset_RemoteData {
        self.remote.as_ref().unwrap_or_else(|| Asset_RemoteData::default_instance())
    }

    fn get_remote_for_reflect(&self) -> &::protobuf::SingularPtrField<Asset_RemoteData> {
        &self.remote
    }

    fn mut_remote_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Asset_RemoteData> {
        &mut self.remote
    }

    // optional .Asset.ImageMetaData image = 4;

    pub fn clear_image(&mut self) {
        self.meta_data = ::std::option::Option::None;
    }

    pub fn has_image(&self) -> bool {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: Asset_ImageMetaData) {
        self.meta_data = ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(v))
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut Asset_ImageMetaData {
        if let ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(_)) = self.meta_data {
        } else {
            self.meta_data = ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(Asset_ImageMetaData::new()));
        }
        match self.meta_data {
            ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_image(&mut self) -> Asset_ImageMetaData {
        if self.has_image() {
            match self.meta_data.take() {
                ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(v)) => v,
                _ => panic!(),
            }
        } else {
            Asset_ImageMetaData::new()
        }
    }

    pub fn get_image(&self) -> &Asset_ImageMetaData {
        match self.meta_data {
            ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(ref v)) => v,
            _ => Asset_ImageMetaData::default_instance(),
        }
    }
}

impl ::protobuf::Message for Asset_Preview {
    fn is_initialized(&self) -> bool {
        if self.mime_type.is_none() {
            return false;
        };
        if self.size.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mime_type)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remote)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.meta_data = ::std::option::Option::Some(Asset_Preview_oneof_meta_data::image(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.mime_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.remote.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &Asset_Preview_oneof_meta_data::image(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mime_type.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.size {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.remote.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.meta_data {
            match v {
                &Asset_Preview_oneof_meta_data::image(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_Preview {
    fn new() -> Asset_Preview {
        Asset_Preview::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_Preview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mime_type",
                    Asset_Preview::get_mime_type_for_reflect,
                    Asset_Preview::mut_mime_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    Asset_Preview::get_size_for_reflect,
                    Asset_Preview::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Asset_RemoteData>>(
                    "remote",
                    Asset_Preview::get_remote_for_reflect,
                    Asset_Preview::mut_remote_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Asset_ImageMetaData>(
                    "image",
                    Asset_Preview::has_image,
                    Asset_Preview::get_image,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_Preview>(
                    "Asset_Preview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_Preview {
    fn clear(&mut self) {
        self.clear_mime_type();
        self.clear_size();
        self.clear_remote();
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_Preview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_Preview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_ImageMetaData {
    // message fields
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    tag: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_ImageMetaData {}

impl Asset_ImageMetaData {
    pub fn new() -> Asset_ImageMetaData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_ImageMetaData {
        static mut instance: ::protobuf::lazy::Lazy<Asset_ImageMetaData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_ImageMetaData,
        };
        unsafe {
            instance.get(Asset_ImageMetaData::new)
        }
    }

    // required int32 width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.width
    }

    // required int32 height = 2;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    fn get_height_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.height
    }

    // optional string tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }
}

impl ::protobuf::Message for Asset_ImageMetaData {
    fn is_initialized(&self) -> bool {
        if self.width.is_none() {
            return false;
        };
        if self.height.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.width {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.height {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.height {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_ImageMetaData {
    fn new() -> Asset_ImageMetaData {
        Asset_ImageMetaData::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_ImageMetaData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "width",
                    Asset_ImageMetaData::get_width_for_reflect,
                    Asset_ImageMetaData::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "height",
                    Asset_ImageMetaData::get_height_for_reflect,
                    Asset_ImageMetaData::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    Asset_ImageMetaData::get_tag_for_reflect,
                    Asset_ImageMetaData::mut_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_ImageMetaData>(
                    "Asset_ImageMetaData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_ImageMetaData {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_ImageMetaData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_ImageMetaData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_VideoMetaData {
    // message fields
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    duration_in_millis: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_VideoMetaData {}

impl Asset_VideoMetaData {
    pub fn new() -> Asset_VideoMetaData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_VideoMetaData {
        static mut instance: ::protobuf::lazy::Lazy<Asset_VideoMetaData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_VideoMetaData,
        };
        unsafe {
            instance.get(Asset_VideoMetaData::new)
        }
    }

    // optional int32 width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.width
    }

    // optional int32 height = 2;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    fn get_height_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.height
    }

    // optional uint64 duration_in_millis = 3;

    pub fn clear_duration_in_millis(&mut self) {
        self.duration_in_millis = ::std::option::Option::None;
    }

    pub fn has_duration_in_millis(&self) -> bool {
        self.duration_in_millis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_in_millis(&mut self, v: u64) {
        self.duration_in_millis = ::std::option::Option::Some(v);
    }

    pub fn get_duration_in_millis(&self) -> u64 {
        self.duration_in_millis.unwrap_or(0)
    }

    fn get_duration_in_millis_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.duration_in_millis
    }

    fn mut_duration_in_millis_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.duration_in_millis
    }
}

impl ::protobuf::Message for Asset_VideoMetaData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.duration_in_millis = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.width {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.height {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.duration_in_millis {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.height {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.duration_in_millis {
            os.write_uint64(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_VideoMetaData {
    fn new() -> Asset_VideoMetaData {
        Asset_VideoMetaData::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_VideoMetaData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "width",
                    Asset_VideoMetaData::get_width_for_reflect,
                    Asset_VideoMetaData::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "height",
                    Asset_VideoMetaData::get_height_for_reflect,
                    Asset_VideoMetaData::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "duration_in_millis",
                    Asset_VideoMetaData::get_duration_in_millis_for_reflect,
                    Asset_VideoMetaData::mut_duration_in_millis_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_VideoMetaData>(
                    "Asset_VideoMetaData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_VideoMetaData {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_duration_in_millis();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_VideoMetaData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_VideoMetaData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_AudioMetaData {
    // message fields
    duration_in_millis: ::std::option::Option<u64>,
    normalized_loudness: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_AudioMetaData {}

impl Asset_AudioMetaData {
    pub fn new() -> Asset_AudioMetaData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_AudioMetaData {
        static mut instance: ::protobuf::lazy::Lazy<Asset_AudioMetaData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_AudioMetaData,
        };
        unsafe {
            instance.get(Asset_AudioMetaData::new)
        }
    }

    // optional uint64 duration_in_millis = 1;

    pub fn clear_duration_in_millis(&mut self) {
        self.duration_in_millis = ::std::option::Option::None;
    }

    pub fn has_duration_in_millis(&self) -> bool {
        self.duration_in_millis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_in_millis(&mut self, v: u64) {
        self.duration_in_millis = ::std::option::Option::Some(v);
    }

    pub fn get_duration_in_millis(&self) -> u64 {
        self.duration_in_millis.unwrap_or(0)
    }

    fn get_duration_in_millis_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.duration_in_millis
    }

    fn mut_duration_in_millis_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.duration_in_millis
    }

    // optional bytes normalized_loudness = 3;

    pub fn clear_normalized_loudness(&mut self) {
        self.normalized_loudness.clear();
    }

    pub fn has_normalized_loudness(&self) -> bool {
        self.normalized_loudness.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normalized_loudness(&mut self, v: ::std::vec::Vec<u8>) {
        self.normalized_loudness = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normalized_loudness(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.normalized_loudness.is_none() {
            self.normalized_loudness.set_default();
        };
        self.normalized_loudness.as_mut().unwrap()
    }

    // Take field
    pub fn take_normalized_loudness(&mut self) -> ::std::vec::Vec<u8> {
        self.normalized_loudness.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_normalized_loudness(&self) -> &[u8] {
        match self.normalized_loudness.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_normalized_loudness_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.normalized_loudness
    }

    fn mut_normalized_loudness_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.normalized_loudness
    }
}

impl ::protobuf::Message for Asset_AudioMetaData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.duration_in_millis = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.normalized_loudness)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.duration_in_millis {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.normalized_loudness.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.duration_in_millis {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.normalized_loudness.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_AudioMetaData {
    fn new() -> Asset_AudioMetaData {
        Asset_AudioMetaData::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_AudioMetaData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "duration_in_millis",
                    Asset_AudioMetaData::get_duration_in_millis_for_reflect,
                    Asset_AudioMetaData::mut_duration_in_millis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "normalized_loudness",
                    Asset_AudioMetaData::get_normalized_loudness_for_reflect,
                    Asset_AudioMetaData::mut_normalized_loudness_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_AudioMetaData>(
                    "Asset_AudioMetaData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_AudioMetaData {
    fn clear(&mut self) {
        self.clear_duration_in_millis();
        self.clear_normalized_loudness();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_AudioMetaData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_AudioMetaData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Asset_RemoteData {
    // message fields
    otr_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sha256: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    asset_id: ::protobuf::SingularField<::std::string::String>,
    asset_token: ::protobuf::SingularField<::std::string::String>,
    encryption: ::std::option::Option<EncryptionAlgorithm>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Asset_RemoteData {}

impl Asset_RemoteData {
    pub fn new() -> Asset_RemoteData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Asset_RemoteData {
        static mut instance: ::protobuf::lazy::Lazy<Asset_RemoteData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Asset_RemoteData,
        };
        unsafe {
            instance.get(Asset_RemoteData::new)
        }
    }

    // required bytes otr_key = 1;

    pub fn clear_otr_key(&mut self) {
        self.otr_key.clear();
    }

    pub fn has_otr_key(&self) -> bool {
        self.otr_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_otr_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.otr_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_otr_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.otr_key.is_none() {
            self.otr_key.set_default();
        };
        self.otr_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_otr_key(&mut self) -> ::std::vec::Vec<u8> {
        self.otr_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_otr_key(&self) -> &[u8] {
        match self.otr_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_otr_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.otr_key
    }

    fn mut_otr_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.otr_key
    }

    // required bytes sha256 = 2;

    pub fn clear_sha256(&mut self) {
        self.sha256.clear();
    }

    pub fn has_sha256(&self) -> bool {
        self.sha256.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha256(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha256 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha256(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha256.is_none() {
            self.sha256.set_default();
        };
        self.sha256.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha256(&mut self) -> ::std::vec::Vec<u8> {
        self.sha256.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sha256(&self) -> &[u8] {
        match self.sha256.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_sha256_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.sha256
    }

    fn mut_sha256_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.sha256
    }

    // optional string asset_id = 3;

    pub fn clear_asset_id(&mut self) {
        self.asset_id.clear();
    }

    pub fn has_asset_id(&self) -> bool {
        self.asset_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asset_id(&mut self, v: ::std::string::String) {
        self.asset_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_asset_id(&mut self) -> &mut ::std::string::String {
        if self.asset_id.is_none() {
            self.asset_id.set_default();
        };
        self.asset_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_asset_id(&mut self) -> ::std::string::String {
        self.asset_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_asset_id(&self) -> &str {
        match self.asset_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_asset_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.asset_id
    }

    fn mut_asset_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.asset_id
    }

    // optional string asset_token = 5;

    pub fn clear_asset_token(&mut self) {
        self.asset_token.clear();
    }

    pub fn has_asset_token(&self) -> bool {
        self.asset_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asset_token(&mut self, v: ::std::string::String) {
        self.asset_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_asset_token(&mut self) -> &mut ::std::string::String {
        if self.asset_token.is_none() {
            self.asset_token.set_default();
        };
        self.asset_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_asset_token(&mut self) -> ::std::string::String {
        self.asset_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_asset_token(&self) -> &str {
        match self.asset_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_asset_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.asset_token
    }

    fn mut_asset_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.asset_token
    }

    // optional .EncryptionAlgorithm encryption = 6;

    pub fn clear_encryption(&mut self) {
        self.encryption = ::std::option::Option::None;
    }

    pub fn has_encryption(&self) -> bool {
        self.encryption.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryption(&mut self, v: EncryptionAlgorithm) {
        self.encryption = ::std::option::Option::Some(v);
    }

    pub fn get_encryption(&self) -> EncryptionAlgorithm {
        self.encryption.unwrap_or(EncryptionAlgorithm::AES_CBC)
    }

    fn get_encryption_for_reflect(&self) -> &::std::option::Option<EncryptionAlgorithm> {
        &self.encryption
    }

    fn mut_encryption_for_reflect(&mut self) -> &mut ::std::option::Option<EncryptionAlgorithm> {
        &mut self.encryption
    }
}

impl ::protobuf::Message for Asset_RemoteData {
    fn is_initialized(&self) -> bool {
        if self.otr_key.is_none() {
            return false;
        };
        if self.sha256.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.otr_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha256)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.asset_id)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.asset_token)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.encryption = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.otr_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.sha256.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.asset_id.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.asset_token.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.encryption {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.otr_key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.sha256.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.asset_id.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.asset_token.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.encryption {
            os.write_enum(6, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Asset_RemoteData {
    fn new() -> Asset_RemoteData {
        Asset_RemoteData::new()
    }

    fn descriptor_static(_: ::std::option::Option<Asset_RemoteData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "otr_key",
                    Asset_RemoteData::get_otr_key_for_reflect,
                    Asset_RemoteData::mut_otr_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha256",
                    Asset_RemoteData::get_sha256_for_reflect,
                    Asset_RemoteData::mut_sha256_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "asset_id",
                    Asset_RemoteData::get_asset_id_for_reflect,
                    Asset_RemoteData::mut_asset_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "asset_token",
                    Asset_RemoteData::get_asset_token_for_reflect,
                    Asset_RemoteData::mut_asset_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EncryptionAlgorithm>>(
                    "encryption",
                    Asset_RemoteData::get_encryption_for_reflect,
                    Asset_RemoteData::mut_encryption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Asset_RemoteData>(
                    "Asset_RemoteData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Asset_RemoteData {
    fn clear(&mut self) {
        self.clear_otr_key();
        self.clear_sha256();
        self.clear_asset_id();
        self.clear_asset_token();
        self.clear_encryption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Asset_RemoteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Asset_RemoteData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Asset_NotUploaded {
    CANCELLED = 0,
    FAILED = 1,
}

impl ::protobuf::ProtobufEnum for Asset_NotUploaded {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Asset_NotUploaded> {
        match value {
            0 => ::std::option::Option::Some(Asset_NotUploaded::CANCELLED),
            1 => ::std::option::Option::Some(Asset_NotUploaded::FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Asset_NotUploaded] = &[
            Asset_NotUploaded::CANCELLED,
            Asset_NotUploaded::FAILED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Asset_NotUploaded>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Asset_NotUploaded", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Asset_NotUploaded {
}

impl ::protobuf::reflect::ProtobufValue for Asset_NotUploaded {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct External {
    // message fields
    otr_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sha256: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    encryption: ::std::option::Option<EncryptionAlgorithm>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for External {}

impl External {
    pub fn new() -> External {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static External {
        static mut instance: ::protobuf::lazy::Lazy<External> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const External,
        };
        unsafe {
            instance.get(External::new)
        }
    }

    // required bytes otr_key = 1;

    pub fn clear_otr_key(&mut self) {
        self.otr_key.clear();
    }

    pub fn has_otr_key(&self) -> bool {
        self.otr_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_otr_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.otr_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_otr_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.otr_key.is_none() {
            self.otr_key.set_default();
        };
        self.otr_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_otr_key(&mut self) -> ::std::vec::Vec<u8> {
        self.otr_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_otr_key(&self) -> &[u8] {
        match self.otr_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_otr_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.otr_key
    }

    fn mut_otr_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.otr_key
    }

    // optional bytes sha256 = 2;

    pub fn clear_sha256(&mut self) {
        self.sha256.clear();
    }

    pub fn has_sha256(&self) -> bool {
        self.sha256.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha256(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha256 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha256(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha256.is_none() {
            self.sha256.set_default();
        };
        self.sha256.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha256(&mut self) -> ::std::vec::Vec<u8> {
        self.sha256.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sha256(&self) -> &[u8] {
        match self.sha256.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_sha256_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.sha256
    }

    fn mut_sha256_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.sha256
    }

    // optional .EncryptionAlgorithm encryption = 3;

    pub fn clear_encryption(&mut self) {
        self.encryption = ::std::option::Option::None;
    }

    pub fn has_encryption(&self) -> bool {
        self.encryption.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryption(&mut self, v: EncryptionAlgorithm) {
        self.encryption = ::std::option::Option::Some(v);
    }

    pub fn get_encryption(&self) -> EncryptionAlgorithm {
        self.encryption.unwrap_or(EncryptionAlgorithm::AES_CBC)
    }

    fn get_encryption_for_reflect(&self) -> &::std::option::Option<EncryptionAlgorithm> {
        &self.encryption
    }

    fn mut_encryption_for_reflect(&mut self) -> &mut ::std::option::Option<EncryptionAlgorithm> {
        &mut self.encryption
    }
}

impl ::protobuf::Message for External {
    fn is_initialized(&self) -> bool {
        if self.otr_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.otr_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha256)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.encryption = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.otr_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.sha256.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.encryption {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.otr_key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.sha256.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.encryption {
            os.write_enum(3, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for External {
    fn new() -> External {
        External::new()
    }

    fn descriptor_static(_: ::std::option::Option<External>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "otr_key",
                    External::get_otr_key_for_reflect,
                    External::mut_otr_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha256",
                    External::get_sha256_for_reflect,
                    External::mut_sha256_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EncryptionAlgorithm>>(
                    "encryption",
                    External::get_encryption_for_reflect,
                    External::mut_encryption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<External>(
                    "External",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for External {
    fn clear(&mut self) {
        self.clear_otr_key();
        self.clear_sha256();
        self.clear_encryption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for External {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for External {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Reaction {
    // message fields
    emoji: ::protobuf::SingularField<::std::string::String>,
    message_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Reaction {}

impl Reaction {
    pub fn new() -> Reaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Reaction {
        static mut instance: ::protobuf::lazy::Lazy<Reaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Reaction,
        };
        unsafe {
            instance.get(Reaction::new)
        }
    }

    // optional string emoji = 1;

    pub fn clear_emoji(&mut self) {
        self.emoji.clear();
    }

    pub fn has_emoji(&self) -> bool {
        self.emoji.is_some()
    }

    // Param is passed by value, moved
    pub fn set_emoji(&mut self, v: ::std::string::String) {
        self.emoji = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_emoji(&mut self) -> &mut ::std::string::String {
        if self.emoji.is_none() {
            self.emoji.set_default();
        };
        self.emoji.as_mut().unwrap()
    }

    // Take field
    pub fn take_emoji(&mut self) -> ::std::string::String {
        self.emoji.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_emoji(&self) -> &str {
        match self.emoji.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_emoji_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.emoji
    }

    fn mut_emoji_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.emoji
    }

    // required string message_id = 2;

    pub fn clear_message_id(&mut self) {
        self.message_id.clear();
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: ::std::string::String) {
        self.message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&mut self) -> &mut ::std::string::String {
        if self.message_id.is_none() {
            self.message_id.set_default();
        };
        self.message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_id(&mut self) -> ::std::string::String {
        self.message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_id(&self) -> &str {
        match self.message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message_id
    }

    fn mut_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message_id
    }
}

impl ::protobuf::Message for Reaction {
    fn is_initialized(&self) -> bool {
        if self.message_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.emoji)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.emoji.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.emoji.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.message_id.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Reaction {
    fn new() -> Reaction {
        Reaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Reaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "emoji",
                    Reaction::get_emoji_for_reflect,
                    Reaction::mut_emoji_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    Reaction::get_message_id_for_reflect,
                    Reaction::mut_message_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Reaction>(
                    "Reaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Reaction {
    fn clear(&mut self) {
        self.clear_emoji();
        self.clear_message_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Reaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Reaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Calling {
    // message fields
    content: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Calling {}

impl Calling {
    pub fn new() -> Calling {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Calling {
        static mut instance: ::protobuf::lazy::Lazy<Calling> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Calling,
        };
        unsafe {
            instance.get(Calling::new)
        }
    }

    // required string content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        self.content.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        match self.content.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content
    }
}

impl ::protobuf::Message for Calling {
    fn is_initialized(&self) -> bool {
        if self.content.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.content.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.content.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Calling {
    fn new() -> Calling {
        Calling::new()
    }

    fn descriptor_static(_: ::std::option::Option<Calling>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    Calling::get_content_for_reflect,
                    Calling::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Calling>(
                    "Calling",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Calling {
    fn clear(&mut self) {
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Calling {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Calling {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ClientAction {
    RESET_SESSION = 0,
}

impl ::protobuf::ProtobufEnum for ClientAction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientAction> {
        match value {
            0 => ::std::option::Option::Some(ClientAction::RESET_SESSION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ClientAction] = &[
            ClientAction::RESET_SESSION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ClientAction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ClientAction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ClientAction {
}

impl ::protobuf::reflect::ProtobufValue for ClientAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncryptionAlgorithm {
    AES_CBC = 0,
    AES_GCM = 1,
}

impl ::protobuf::ProtobufEnum for EncryptionAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncryptionAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(EncryptionAlgorithm::AES_CBC),
            1 => ::std::option::Option::Some(EncryptionAlgorithm::AES_GCM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncryptionAlgorithm] = &[
            EncryptionAlgorithm::AES_CBC,
            EncryptionAlgorithm::AES_GCM,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EncryptionAlgorithm>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncryptionAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncryptionAlgorithm {
}

impl ::protobuf::reflect::ProtobufValue for EncryptionAlgorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xbe, 0x05, 0x0a, 0x0e, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x49, 0x64, 0x12, 0x1b, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x05, 0x2e, 0x54, 0x65, 0x78, 0x74, 0x48, 0x00, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12,
    0x23, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x48, 0x00, 0x52, 0x05, 0x69,
    0x6d, 0x61, 0x67, 0x65, 0x12, 0x1e, 0x0a, 0x05, 0x6b, 0x6e, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x4b, 0x6e, 0x6f, 0x63, 0x6b, 0x48, 0x00, 0x52, 0x05, 0x6b,
    0x6e, 0x6f, 0x63, 0x6b, 0x12, 0x27, 0x0a, 0x08, 0x6c, 0x61, 0x73, 0x74, 0x52, 0x65, 0x61, 0x64,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4c, 0x61, 0x73, 0x74, 0x52, 0x65, 0x61,
    0x64, 0x48, 0x00, 0x52, 0x08, 0x6c, 0x61, 0x73, 0x74, 0x52, 0x65, 0x61, 0x64, 0x12, 0x24, 0x0a,
    0x07, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x65, 0x64, 0x48, 0x00, 0x52, 0x07, 0x63, 0x6c, 0x65, 0x61,
    0x72, 0x65, 0x64, 0x12, 0x27, 0x0a, 0x08, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x48, 0x00, 0x52, 0x08, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x12, 0x33, 0x0a, 0x0c,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x48, 0x00, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x24, 0x0a, 0x07, 0x63, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x08, 0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x48, 0x00, 0x52, 0x07,
    0x63, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x12, 0x1e, 0x0a, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x48, 0x00,
    0x52, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x69, 0x64, 0x64, 0x65,
    0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x48, 0x69, 0x64, 0x65, 0x48, 0x00, 0x52, 0x06, 0x68, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x12,
    0x27, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x09, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x08,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x07, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x48, 0x00, 0x52, 0x07, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x64, 0x12, 0x26, 0x0a, 0x06, 0x65, 0x64, 0x69, 0x74, 0x65, 0x64, 0x18, 0x0f,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x45, 0x64,
    0x69, 0x74, 0x48, 0x00, 0x52, 0x06, 0x65, 0x64, 0x69, 0x74, 0x65, 0x64, 0x12, 0x33, 0x0a, 0x0c,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x10, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x48, 0x00, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x27, 0x0a, 0x08, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00,
    0x52, 0x08, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x09, 0x65, 0x70,
    0x68, 0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e,
    0x45, 0x70, 0x68, 0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x09, 0x65, 0x70, 0x68,
    0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x22, 0xf1, 0x01, 0x0a, 0x09, 0x45, 0x70, 0x68, 0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x12,
    0x2e, 0x0a, 0x13, 0x65, 0x78, 0x70, 0x69, 0x72, 0x65, 0x5f, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f,
    0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x52, 0x11, 0x65, 0x78,
    0x70, 0x69, 0x72, 0x65, 0x41, 0x66, 0x74, 0x65, 0x72, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x12,
    0x1b, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e,
    0x54, 0x65, 0x78, 0x74, 0x48, 0x00, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x23, 0x0a, 0x05,
    0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x49, 0x6d,
    0x61, 0x67, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x12, 0x1e, 0x0a, 0x05, 0x6b, 0x6e, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x06, 0x2e, 0x4b, 0x6e, 0x6f, 0x63, 0x6b, 0x48, 0x00, 0x52, 0x05, 0x6b, 0x6e, 0x6f, 0x63,
    0x6b, 0x12, 0x1e, 0x0a, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x06, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x48, 0x00, 0x52, 0x05, 0x61, 0x73, 0x73, 0x65,
    0x74, 0x12, 0x27, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00,
    0x52, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0x75, 0x0a, 0x04, 0x54, 0x65, 0x78, 0x74, 0x12, 0x18, 0x0a,
    0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x07,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x22, 0x0a, 0x07, 0x6d, 0x65, 0x6e, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4d, 0x65, 0x6e, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x07, 0x6d, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x0a, 0x0c, 0x6c,
    0x69, 0x6e, 0x6b, 0x5f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x4c, 0x69, 0x6e, 0x6b, 0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x52,
    0x0b, 0x6c, 0x69, 0x6e, 0x6b, 0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x22, 0x2b, 0x0a, 0x05,
    0x4b, 0x6e, 0x6f, 0x63, 0x6b, 0x12, 0x22, 0x0a, 0x09, 0x68, 0x6f, 0x74, 0x5f, 0x6b, 0x6e, 0x6f,
    0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x52,
    0x08, 0x68, 0x6f, 0x74, 0x4b, 0x6e, 0x6f, 0x63, 0x6b, 0x22, 0x8f, 0x02, 0x0a, 0x0b, 0x4c, 0x69,
    0x6e, 0x6b, 0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x75,
    0x72, 0x6c, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x52,
    0x09, 0x75, 0x72, 0x6c, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x12, 0x24, 0x0a, 0x07, 0x61, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x41, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x48, 0x00, 0x52, 0x07, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x12, 0x23, 0x0a, 0x0d, 0x70, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x5f, 0x75, 0x72,
    0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x70, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65,
    0x6e, 0x74, 0x55, 0x72, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73,
    0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75,
    0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x05, 0x69, 0x6d,
    0x61, 0x67, 0x65, 0x12, 0x1e, 0x0a, 0x05, 0x74, 0x77, 0x65, 0x65, 0x74, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54, 0x77, 0x65, 0x65, 0x74, 0x48, 0x01, 0x52, 0x05, 0x74, 0x77,
    0x65, 0x65, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x70, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x42, 0x0b,
    0x0a, 0x09, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x22, 0x3b, 0x0a, 0x05, 0x54,
    0x77, 0x65, 0x65, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x12, 0x1a, 0x0a, 0x08,
    0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x7c, 0x0a, 0x07, 0x41, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x70, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74,
    0x5f, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x0c, 0x70, 0x65, 0x72, 0x6d,
    0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52,
    0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x22, 0x3f, 0x0a, 0x07, 0x4d, 0x65, 0x6e, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x17, 0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x06, 0x75, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x75, 0x73,
    0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x08, 0x75,
    0x73, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x63, 0x0a, 0x08, 0x4c, 0x61, 0x73, 0x74, 0x52,
    0x65, 0x61, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x0e, 0x63, 0x6f,
    0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x13,
    0x6c, 0x61, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x52, 0x11, 0x6c, 0x61, 0x73, 0x74, 0x52,
    0x65, 0x61, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x5f, 0x0a, 0x07,
    0x43, 0x6c, 0x65, 0x61, 0x72, 0x65, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x76, 0x65,
    0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x0e, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x12, 0x2b, 0x0a, 0x11, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x52, 0x10, 0x63, 0x6c, 0x65,
    0x61, 0x72, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x55, 0x0a,
    0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x48, 0x69, 0x64, 0x65, 0x12, 0x27, 0x0a, 0x0f,
    0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x0e, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x49, 0x64, 0x22, 0x2e, 0x0a, 0x0d, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x44,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x49, 0x64, 0x22, 0x67, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x45,
    0x64, 0x69, 0x74, 0x12, 0x30, 0x0a, 0x14, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x69, 0x6e, 0x67,
    0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x52, 0x12, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x69, 0x6e, 0x67, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x54, 0x65, 0x78, 0x74, 0x48, 0x00, 0x52, 0x04, 0x74, 0x65,
    0x78, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0xab, 0x01,
    0x0a, 0x0c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x26,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x0e, 0x66, 0x69, 0x72, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x49, 0x64,
    0x12, 0x28, 0x0a, 0x10, 0x6d, 0x6f, 0x72, 0x65, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0e, 0x6d, 0x6f, 0x72, 0x65,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x49, 0x64, 0x73, 0x22, 0x1f, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x44, 0x45, 0x4c, 0x49, 0x56, 0x45, 0x52, 0x45, 0x44, 0x10,
    0x00, 0x12, 0x08, 0x0a, 0x04, 0x52, 0x45, 0x41, 0x44, 0x10, 0x01, 0x22, 0x6c, 0x0a, 0x08, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x02, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x7a, 0x6f, 0x6f, 0x6d, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x04, 0x7a, 0x6f, 0x6f, 0x6d, 0x22, 0xa9, 0x02, 0x0a, 0x0a, 0x49, 0x6d,
    0x61, 0x67, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67, 0x12, 0x14, 0x0a, 0x05, 0x77, 0x69,
    0x64, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x52, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05,
    0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05,
    0x52, 0x0d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x57, 0x69, 0x64, 0x74, 0x68, 0x12,
    0x27, 0x0a, 0x0f, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x05, 0x20, 0x02, 0x28, 0x05, 0x52, 0x0e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x69, 0x6d, 0x65,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x69, 0x6d,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x07, 0x20,
    0x02, 0x28, 0x05, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x6f, 0x74, 0x72,
    0x5f, 0x6b, 0x65, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6f, 0x74, 0x72, 0x4b,
    0x65, 0x79, 0x12, 0x17, 0x0a, 0x07, 0x6d, 0x61, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x06, 0x6d, 0x61, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6d,
    0x61, 0x63, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6d, 0x61, 0x63, 0x12, 0x16, 0x0a,
    0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x73,
    0x68, 0x61, 0x32, 0x35, 0x36, 0x22, 0x9a, 0x09, 0x0a, 0x05, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12,
    0x2b, 0x0a, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x52, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x37, 0x0a, 0x0c,
    0x6e, 0x6f, 0x74, 0x5f, 0x75, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x12, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x4e, 0x6f, 0x74, 0x55, 0x70,
    0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0b, 0x6e, 0x6f, 0x74, 0x55, 0x70, 0x6c,
    0x6f, 0x61, 0x64, 0x65, 0x64, 0x12, 0x2f, 0x0a, 0x08, 0x75, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x65,
    0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e,
    0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x08, 0x75, 0x70,
    0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x12, 0x28, 0x0a, 0x07, 0x70, 0x72, 0x65, 0x76, 0x69, 0x65,
    0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e,
    0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x52, 0x07, 0x70, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77,
    0x1a, 0x98, 0x02, 0x0a, 0x08, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x1b, 0x0a,
    0x09, 0x6d, 0x69, 0x6d, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x08, 0x6d, 0x69, 0x6d, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69,
    0x7a, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x4d,
    0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65,
    0x12, 0x2c, 0x0a, 0x05, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x4d, 0x65, 0x74,
    0x61, 0x44, 0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x05, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x12, 0x2c,
    0x0a, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e,
    0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x4d, 0x65, 0x74, 0x61, 0x44,
    0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x16, 0x0a, 0x06,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x0b,
    0x0a, 0x09, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x1a, 0xa0, 0x01, 0x0a, 0x07,
    0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x69, 0x6d, 0x65, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x69, 0x6d, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x04, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x29, 0x0a, 0x06, 0x72, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74,
    0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x52, 0x06, 0x72, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x12, 0x2c, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x14, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65,
    0x4d, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x42, 0x0b, 0x0a, 0x09, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x1a, 0x4f,
    0x0a, 0x0d, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x14, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x52, 0x05,
    0x77, 0x69, 0x64, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x05, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x10, 0x0a,
    0x03, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67, 0x1a,
    0x6b, 0x0a, 0x0d, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x4d, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61,
    0x12, 0x14, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x2c,
    0x0a, 0x12, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x5f, 0x6d, 0x69,
    0x6c, 0x6c, 0x69, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x10, 0x64, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x1a, 0x6e, 0x0a, 0x0d,
    0x41, 0x75, 0x64, 0x69, 0x6f, 0x4d, 0x65, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2c, 0x0a,
    0x12, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x5f, 0x6d, 0x69, 0x6c,
    0x6c, 0x69, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x10, 0x64, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x12, 0x2f, 0x0a, 0x13, 0x6e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x75, 0x64, 0x6e, 0x65,
    0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x12, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x4c, 0x6f, 0x75, 0x64, 0x6e, 0x65, 0x73, 0x73, 0x1a, 0xaf, 0x01, 0x0a,
    0x0a, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x17, 0x0a, 0x07, 0x6f,
    0x74, 0x72, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x06, 0x6f, 0x74,
    0x72, 0x4b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x52, 0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x12, 0x19, 0x0a, 0x08,
    0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07,
    0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x73, 0x73, 0x65, 0x74,
    0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x61, 0x73,
    0x73, 0x65, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x34, 0x0a, 0x0a, 0x65, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x45,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74,
    0x68, 0x6d, 0x52, 0x0a, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x28,
    0x0a, 0x0b, 0x4e, 0x6f, 0x74, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x12, 0x0d, 0x0a,
    0x09, 0x43, 0x41, 0x4e, 0x43, 0x45, 0x4c, 0x4c, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06,
    0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x01, 0x42, 0x08, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x22, 0x71, 0x0a, 0x08, 0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x12, 0x17,
    0x0a, 0x07, 0x6f, 0x74, 0x72, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52,
    0x06, 0x6f, 0x74, 0x72, 0x4b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x68, 0x61, 0x32, 0x35,
    0x36, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x12,
    0x34, 0x0a, 0x0a, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x52, 0x0a, 0x65, 0x6e, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x3f, 0x0a, 0x08, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x6f, 0x6a, 0x69, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x65, 0x6d, 0x6f, 0x6a, 0x69, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x49, 0x64, 0x22, 0x23, 0x0a, 0x07, 0x43, 0x61, 0x6c, 0x6c, 0x69, 0x6e,
    0x67, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2a, 0x21, 0x0a, 0x0c, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x11, 0x0a, 0x0d, 0x52,
    0x45, 0x53, 0x45, 0x54, 0x5f, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x00, 0x2a, 0x2f,
    0x0a, 0x13, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x6f,
    0x72, 0x69, 0x74, 0x68, 0x6d, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x45, 0x53, 0x5f, 0x43, 0x42, 0x43,
    0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x45, 0x53, 0x5f, 0x47, 0x43, 0x4d, 0x10, 0x01, 0x42,
    0x0f, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x2e, 0x77, 0x61, 0x7a, 0x2e, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x4a, 0xc7, 0x4d, 0x0a, 0x07, 0x12, 0x05, 0x01, 0x00, 0xe5, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x01, 0x00, 0x26, 0x0a, 0x21, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x01, 0x00, 0x26, 0x1a, 0x14, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x20, 0x3d, 0x20, 0x22,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x32, 0x22, 0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x01, 0x16, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x17, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x16, 0x0a, 0x3a, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x21, 0x22, 0x2d, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x72, 0x61, 0x6e,
    0x64, 0x6f, 0x6d, 0x20, 0x69, 0x64, 0x2c, 0x20, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x61, 0x62,
    0x6c, 0x79, 0x20, 0x55, 0x55, 0x49, 0x44, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x05, 0x02, 0x16, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x06, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x06, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x06, 0x10, 0x11, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x04, 0x19,
    0x22, 0x1f, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e,
    0x20, 0x66, 0x61, 0x76, 0x6f, 0x75, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x73, 0x73, 0x65, 0x74,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x07, 0x04, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x0f, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x08, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x09, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0a, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x0a, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0a,
    0x0c, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0a, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0b, 0x04, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0b, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x0b, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x0c, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x0c,
    0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0c, 0x11, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0c, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0d, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0d, 0x0c, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x0d, 0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0e,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x0e, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x0e, 0x0a, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0e, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x0f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x06, 0x12, 0x03, 0x0f, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x0f, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x0f, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x10, 0x04, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x10, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x10, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0c, 0x12, 0x03, 0x11, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x06,
    0x12, 0x03, 0x11, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x11, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x11, 0x1c,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x12, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x12, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x12, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0d, 0x03, 0x12, 0x03, 0x12, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e,
    0x12, 0x03, 0x13, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x06, 0x12, 0x03,
    0x13, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x13, 0x11,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x13, 0x20, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x14, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x14, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f,
    0x03, 0x12, 0x03, 0x14, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03,
    0x15, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x06, 0x12, 0x03, 0x15, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x15, 0x0e, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x19, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1a,
    0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01,
    0x08, 0x00, 0x12, 0x04, 0x1b, 0x02, 0x21, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00,
    0x01, 0x12, 0x03, 0x1b, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x1c, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1c, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x1d, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x1d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x04,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1e, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1e, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x1f, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x1f, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1f,
    0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x20, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x20, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x20, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x24, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x24, 0x08,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x25, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x26, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x26, 0x0b, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x13, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x27, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27,
    0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x26, 0x27,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2a, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x2b, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x10, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2b, 0x1e, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x07, 0x12, 0x03, 0x2b, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x2e, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2f, 0x18, 0x19, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x30, 0x02, 0x20, 0x22, 0x2b, 0x20, 0x75, 0x72, 0x6c, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x08, 0x00,
    0x12, 0x04, 0x32, 0x02, 0x34, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x32, 0x08, 0x0f, 0x0a, 0x29, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x33, 0x04,
    0x18, 0x22, 0x1c, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d,
    0x20, 0x75, 0x73, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x33, 0x04, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x0c, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x36, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x36,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x36, 0x12, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x36, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x37, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x37, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x37, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x38, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x38, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x38, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06,
    0x12, 0x03, 0x39, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x06, 0x12, 0x03, 0x39, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x39, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x39, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x08, 0x01, 0x12, 0x04, 0x3b, 0x02, 0x3d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x08, 0x01, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x07, 0x12, 0x03, 0x3c, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x06, 0x12,
    0x03, 0x3c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x3c,
    0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x3c, 0x12, 0x13,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x40, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x40, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x41, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x41, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x12, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x42, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x42, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x42, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x42,
    0x1d, 0x1e, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x46, 0x00, 0x4b, 0x01, 0x1a, 0x37,
    0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x4c, 0x69, 0x6e, 0x6b, 0x50,
    0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x46, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x47, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x48, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x49, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x49, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x49, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49,
    0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x4a, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4d,
    0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x4e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4e, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x4e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4f, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12,
    0x04, 0x52, 0x00, 0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x52, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x53, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x53, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x53, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03,
    0x54, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x11, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x57, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x57, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x58, 0x02, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x58, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x59, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x59, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x59, 0x11, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x59, 0x25, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x5c, 0x00, 0x5f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x5c, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x5d,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x01, 0x12, 0x03, 0x5e, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x5e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5e, 0x12,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x1f, 0x20, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x61, 0x00, 0x63, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0b, 0x01, 0x12, 0x03, 0x61, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12,
    0x03, 0x62, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x62,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x62, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0c, 0x12, 0x04, 0x65, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12,
    0x03, 0x65, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x66, 0x02,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x12, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x08,
    0x00, 0x12, 0x04, 0x67, 0x02, 0x69, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x67, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x68,
    0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x03, 0x68, 0x04, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x68, 0x09, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x68, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x04, 0x6c, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x03, 0x6c, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x04, 0x00, 0x12, 0x04, 0x6d, 0x02,
    0x70, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x04, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x07, 0x0b,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6e, 0x04, 0x12, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e, 0x04, 0x0d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x6e, 0x10, 0x11, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x04, 0x0d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6f, 0x04, 0x08, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x6f, 0x0b, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x72, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x72, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x72, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x72, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x73, 0x02, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x73, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x73, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02,
    0x12, 0x03, 0x74, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x03, 0x74, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x03, 0x74, 0x12, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x74, 0x25, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0e, 0x12, 0x04, 0x77, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01,
    0x12, 0x03, 0x77, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x78,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x78, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x03, 0x79, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x79, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79, 0x11,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x1c, 0x1d, 0x0a,
    0x28, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x02, 0x1b, 0x22, 0x1b, 0x20, 0x6c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x2f, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x7a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x7a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x19,
    0x1a, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x03, 0x7b, 0x02, 0x1a, 0x22, 0x37,
    0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x6d, 0x61, 0x70, 0x73, 0x20, 0x7a, 0x6f, 0x6f,
    0x6d, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x28, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x6d,
    0x61, 0x70, 0x73, 0x20, 0x61, 0x70, 0x69, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x7b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7b, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7b, 0x18, 0x19, 0x0a,
    0x43, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x05, 0x7e, 0x00, 0x8a, 0x01, 0x01, 0x22, 0x36, 0x20, 0x64,
    0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x66, 0x61, 0x76,
    0x6f, 0x75, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x4f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x61, 0x6c, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x44,
    0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x7f, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x7f, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x80,
    0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0x81, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x03, 0x12, 0x04, 0x82, 0x01, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05, 0x12,
    0x04, 0x82, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x82, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04, 0x82,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x04, 0x83, 0x01, 0x11, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04, 0x83, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x04, 0x84, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x05, 0x04, 0x12, 0x04, 0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x05, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x84, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x84, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x06, 0x12,
    0x04, 0x85, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x04, 0x12, 0x04,
    0x85, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x05, 0x12, 0x04, 0x85,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x01, 0x12, 0x04, 0x85, 0x01,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x03, 0x12, 0x04, 0x85, 0x01, 0x18,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0x86, 0x01, 0x02, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0x86, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0x86, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1b, 0x1c, 0x0a, 0x27, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x08, 0x12, 0x04, 0x87, 0x01, 0x02, 0x1d, 0x22, 0x19, 0x20, 0x64, 0x65, 0x70, 0x72,
    0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20, 0x75, 0x73, 0x65, 0x20, 0x73, 0x68, 0x61,
    0x32, 0x35, 0x36, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x04, 0x12, 0x04, 0x87,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x05, 0x12, 0x04, 0x87, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x01, 0x12, 0x04, 0x87, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1b, 0x1c,
    0x0a, 0x27, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x09, 0x12, 0x04, 0x88, 0x01, 0x02, 0x1a, 0x22, 0x19,
    0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x09, 0x04, 0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09,
    0x05, 0x12, 0x04, 0x88, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01,
    0x12, 0x04, 0x88, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12,
    0x04, 0x88, 0x01, 0x17, 0x19, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0a, 0x12, 0x04, 0x89,
    0x01, 0x02, 0x1d, 0x22, 0x16, 0x20, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x20, 0x6f, 0x66, 0x20,
    0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x0a, 0x05, 0x12, 0x04, 0x89, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0a, 0x01, 0x12, 0x04, 0x89, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a,
    0x03, 0x12, 0x04, 0x89, 0x01, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x8c,
    0x01, 0x00, 0xcc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x8c, 0x01,
    0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03, 0x00, 0x12, 0x06, 0x8d, 0x01, 0x02, 0x98,
    0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x0a,
    0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x01, 0x04,
    0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x01,
    0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8e,
    0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8e, 0x01, 0x14, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8e, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x04, 0x8f, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x8f, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x90, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x90, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1b, 0x1c, 0x0a, 0x10, 0x0a,
    0x06, 0x04, 0x10, 0x03, 0x00, 0x08, 0x00, 0x12, 0x06, 0x91, 0x01, 0x04, 0x95, 0x01, 0x05, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x08, 0x00, 0x01, 0x12, 0x04, 0x91, 0x01, 0x0a, 0x13,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0x92, 0x01, 0x06, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x03, 0x06, 0x12, 0x04, 0x92, 0x01, 0x06,
    0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x92, 0x01,
    0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0x93,
    0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x93, 0x01, 0x06, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x93, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x93, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x05,
    0x12, 0x04, 0x94, 0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x05,
    0x06, 0x12, 0x04, 0x94, 0x01, 0x06, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x94, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1c, 0x1d, 0x0a, 0x3d, 0x0a, 0x06, 0x04, 0x10, 0x03,
    0x00, 0x02, 0x06, 0x12, 0x04, 0x96, 0x01, 0x04, 0x1f, 0x22, 0x2d, 0x20, 0x6c, 0x69, 0x6e, 0x6b,
    0x20, 0x74, 0x6f, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x65, 0x2e, 0x67, 0x2e, 0x20,
    0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x70, 0x68, 0x79, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x32, 0x33, 0x34, 0x32, 0x34, 0x35, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x00, 0x02, 0x06, 0x05, 0x12, 0x04, 0x96, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x96, 0x01, 0x14, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0x96, 0x01, 0x1d, 0x1e, 0x0a, 0x52, 0x0a, 0x06,
    0x04, 0x10, 0x03, 0x00, 0x02, 0x07, 0x12, 0x04, 0x97, 0x01, 0x04, 0x20, 0x22, 0x42, 0x20, 0x63,
    0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x73,
    0x73, 0x65, 0x74, 0x2c, 0x20, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x22, 0x64, 0x6f, 0x67, 0x22, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x47, 0x69, 0x70, 0x68, 0x79, 0x20, 0x22, 0x64, 0x6f, 0x67,
    0x22, 0x20, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x0a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x97, 0x01, 0x04,
    0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x04, 0x97, 0x01,
    0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x14, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04,
    0x97, 0x01, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03, 0x01, 0x12, 0x06, 0x9a, 0x01,
    0x02, 0xa1, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03, 0x01, 0x01, 0x12, 0x04, 0x9a,
    0x01, 0x0a, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0x9b,
    0x01, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x9b, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9b, 0x01, 0x14, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x9b, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x01, 0x02,
    0x01, 0x12, 0x04, 0x9c, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x10, 0x03, 0x01, 0x02, 0x02, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x01, 0x02, 0x02, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x0d, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x18, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x21, 0x22, 0x0a,
    0x10, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x01, 0x08, 0x00, 0x12, 0x06, 0x9e, 0x01, 0x04, 0xa0, 0x01,
    0x05, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x08, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x01,
    0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x01, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x01,
    0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x03, 0x06, 0x12, 0x04, 0x9f,
    0x01, 0x06, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x9f, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x9f, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03, 0x02, 0x12, 0x06, 0xa3,
    0x01, 0x02, 0xa7, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03, 0x02, 0x01, 0x12, 0x04,
    0xa3, 0x01, 0x0a, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x02, 0x02, 0x00, 0x12, 0x04,
    0xa4, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa4, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xa4, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa4, 0x01, 0x13, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x13, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x10, 0x03, 0x02, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x13, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x14, 0x17, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x02, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1a, 0x1b,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03, 0x03, 0x12, 0x06, 0xa9, 0x01, 0x02, 0xad, 0x01, 0x03,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x0a, 0x17, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x1d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x0c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0d,
    0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01,
    0x13, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa,
    0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x03, 0x02, 0x01, 0x12, 0x04, 0xab,
    0x01, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xab, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x13, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xab, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x03, 0x02,
    0x02, 0x12, 0x04, 0xac, 0x01, 0x04, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xac, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x03,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xac, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x04, 0xac, 0x01, 0x14, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x03, 0x02, 0x02, 0x03, 0x12, 0x04, 0xac, 0x01, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x10, 0x03, 0x04, 0x12, 0x06, 0xaf, 0x01, 0x02, 0xb5, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x03, 0x04, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x0a, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10,
    0x03, 0x04, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x14, 0x26, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x29, 0x2a, 0x0a, 0xb3,
    0x01, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x04, 0x02, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x2b, 0x1a,
    0x61, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74,
    0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x75, 0x64,
    0x6e, 0x65, 0x73, 0x73, 0x20, 0x3d, 0x20, 0x32, 0x20, 0x5b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64,
    0x3d, 0x74, 0x72, 0x75, 0x65, 0x5d, 0x3b, 0x20, 0x2f, 0x2f, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65,
    0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20, 0x53, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61,
    0x64, 0x0a, 0x22, 0x40, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x72,
    0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6c, 0x6f, 0x75,
    0x64, 0x6e, 0x65, 0x73, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x73, 0x20, 0x61,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x28, 0x63, 0x68, 0x61, 0x72, 0x29, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb2, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x04, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xb2, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb2, 0x01, 0x13, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x04, 0x00,
    0x12, 0x06, 0xb7, 0x01, 0x02, 0xba, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x04, 0x00,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x07, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xb8, 0x01, 0x04, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x04, 0x0d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xb8, 0x01, 0x10, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xb9, 0x01, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x10, 0x03, 0x05, 0x12, 0x06, 0xbc, 0x01, 0x02, 0xc3, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x03, 0x05, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x0a, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10,
    0x03, 0x05, 0x02, 0x00, 0x12, 0x04, 0xbd, 0x01, 0x04, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x05, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbd, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x13, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x1d, 0x1e, 0x0a, 0x42,
    0x0a, 0x06, 0x04, 0x10, 0x03, 0x05, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x04, 0x1e, 0x22, 0x32,
    0x20, 0x6f, 0x62, 0x73, 0x6f, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x62, 0x75, 0x74, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x77,
    0x61, 0x72, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbe,
    0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xbe, 0x01, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xbe, 0x01, 0x13, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xbe, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x05, 0x02, 0x02,
    0x12, 0x04, 0xbf, 0x01, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xbf, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x14, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x05, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x1f, 0x20, 0x0a, 0x5b, 0x0a, 0x06, 0x04, 0x10,
    0x03, 0x05, 0x02, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x24, 0x1a, 0x4b, 0x20, 0x20, 0x20, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x61,
    0x73, 0x73, 0x65, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x3d, 0x20, 0x34, 0x3b, 0x20,
    0x2f, 0x2f, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x05,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x05, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x14, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x05, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x10, 0x03, 0x05, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x04, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x05, 0x02, 0x04, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x05, 0x02, 0x04, 0x06, 0x12, 0x04, 0xc2, 0x01, 0x0d, 0x20, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x21, 0x2b, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x03, 0x05, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x2e, 0x2f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc5, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x1f, 0x20, 0x0a, 0x61, 0x0a, 0x04, 0x04, 0x10, 0x08,
    0x00, 0x12, 0x06, 0xc7, 0x01, 0x02, 0xca, 0x01, 0x03, 0x1a, 0x51, 0x20, 0x20, 0x6f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x50, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x20, 0x70, 0x72,
    0x65, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3d, 0x20, 0x32, 0x3b, 0x20, 0x20, 0x2f, 0x2f, 0x20, 0x64,
    0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x20, 0x70, 0x72, 0x65, 0x76,
    0x69, 0x65, 0x77, 0x20, 0x77, 0x61, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65,
    0x6c, 0x79, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x08, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xc8, 0x01, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xc8, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04,
    0xc9, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x06, 0x12, 0x04, 0xc9,
    0x01, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x01,
    0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x02, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0xcb, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x06, 0x12, 0x04, 0xcb, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x1d, 0x1e, 0x0a, 0x50, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xd3, 0x01, 0x01, 0x1a, 0x42, 0x20, 0x41, 0x63, 0x74, 0x75,
    0x61, 0x6c, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x41, 0x45, 0x53,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73, 0x20, 0x61, 0x64, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd0, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd0, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd0, 0x01, 0x1b, 0x1c, 0x0a, 0x56, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x01,
    0x02, 0x1c, 0x22, 0x48, 0x20, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x20, 0x6f, 0x66, 0x20, 0x63,
    0x69, 0x70, 0x68, 0x65, 0x72, 0x74, 0x65, 0x78, 0x74, 0x2c, 0x20, 0x6f, 0x62, 0x73, 0x6f, 0x6c,
    0x65, 0x74, 0x65, 0x20, 0x62, 0x75, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x77, 0x61, 0x72, 0x64, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02,
    0x12, 0x04, 0xd2, 0x01, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xd2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xd2, 0x01, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd2,
    0x01, 0x1f, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd2, 0x01,
    0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xd5, 0x01, 0x00, 0xd8, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x10, 0x0a, 0x56, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x01, 0x02, 0x1c, 0x22, 0x48, 0x20, 0x73, 0x6f,
    0x6d, 0x65, 0x20, 0x65, 0x6d, 0x6f, 0x6a, 0x69, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x73,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20,
    0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x28, 0x73, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xd6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01,
    0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x05,
    0x00, 0x12, 0x06, 0xda, 0x01, 0x00, 0xdc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x04, 0xda, 0x01, 0x05, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x04,
    0xdb, 0x01, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdb,
    0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xdb, 0x01,
    0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xde, 0x01, 0x00, 0xe0, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xde, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xdf, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xdf, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x06, 0xe2,
    0x01, 0x00, 0xe5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x01,
    0x05, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x0c, 0x0d, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0xe4, 0x01, 0x0c, 0x0d,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
