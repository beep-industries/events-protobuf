// This includes the generated structs to make them available to the code
pub mod test_svc_merchandise {
    include!(concat!(env!("OUT_DIR"), "/test_svc.merchandise.rs"));
} // This module (and the generated structs) are defined here to be part of the lib's public API

pub mod communities_events {
    include!(concat!(env!("OUT_DIR"), "/communities.events.rs"));
}

pub mod messages_events {
    include!(concat!(env!("OUT_DIR"), "/messages.events.rs"));
}