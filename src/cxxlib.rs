use commandor::prelude::*;

pub mod commands;
pub mod lib;

#[cxx::bridge(namespace = "moe::launcher::agcli")]
mod agcli {
    pub struct PatchLocal {
        pub version: String,
    }

    pub struct PatchUpstream {
        //server: String,
        pub version: String,
    }

    pub struct PatchData {
        pub applied: PatchLocal,
        pub upstream: PatchUpstream,
    }

    pub struct PatchStatus {
        pub is_applied: bool,
        pub failed_check: bool,

        pub state: PatchData,
    }

    extern "Rust" {
        fn get_local_patch_state(path: &String) -> PatchLocal;
        fn get_upstream_patch_state() -> PatchUpstream;
    }
}


pub fn get_local_patch_state(path: &String) -> agcli::PatchLocal {
    agcli::PatchLocal {
        version: String::from(path) + " - hello there"
    }
}

pub fn get_upstream_patch_state() -> agcli::PatchUpstream {
    let patch_up = commands::Patch::new();

    agcli::PatchUpstream {
        version: String::from("test")
    }
}

pub fn can_talk() -> bool {
    return true;
}