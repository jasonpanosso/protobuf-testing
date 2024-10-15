pub mod citadel_gcmessages_common {
    include!(concat!(env!("OUT_DIR"), "/citadel_gcmessages_common.rs"));
}

pub mod citadel_gcmessages_server {
    include!(concat!(env!("OUT_DIR"), "/citadel_gcmessages_server.rs"));
}

pub mod gcsdk_gcmessages {
    include!(concat!(env!("OUT_DIR"), "/gcsdk_gcmessages.rs"));
}

pub mod steammessages {
    include!(concat!(env!("OUT_DIR"), "/steammessages.rs"));
}

pub mod steammessages_unified_base {
    pub mod steamworkssdk {
        include!(concat!(
            env!("OUT_DIR"),
            "/steammessages_unified_base.steamworkssdk.rs"
        ));
    }
}

pub mod steammessages_steamlearn {
    pub mod steamworkssdk {
        include!(concat!(
            env!("OUT_DIR"),
            "/steammessages_steamlearn.steamworkssdk.rs"
        ));
    }
}
