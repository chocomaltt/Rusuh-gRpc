pub mod application {
    pub mod auth_use_case;
}

pub mod config {
    pub mod db;
    pub mod env;
    pub mod redis;
}

pub mod core {
    pub mod server;
}

pub mod domain {
    pub mod entity {
        pub mod user;
        pub mod user_info;
        pub mod user_security;
        pub mod user_sessions;
    }
    pub mod port {
        pub mod db {
            pub mod user_info_port;
            pub mod user_port;
            pub mod user_security_port;
            pub mod user_session_port;
        }
        pub mod db_port;
        pub mod redis_port;
    }
    pub mod service {
        pub mod jwt_service;
        pub mod totp_service;
    }
}

pub mod infrastructure {
    pub mod db {
        pub mod user_adapter;
        pub mod user_info_adapter;
        pub mod user_security_adapter;
        pub mod user_session_adapter;
    }
    pub mod notification {
        pub mod email_adapter;
    }
    pub mod redis {
        pub mod redis_adapter;
    }
}

pub mod interface {
    pub mod grpc {
        pub mod auth_handler;
    }
    pub mod common {
        pub mod client_info;
    }
    pub mod interceptor {
        pub mod auth_interceptor;
    }
}

pub mod pb {
    pub mod auth;
}

pub use config::env::cfg;
pub use domain::service::totp_service::otp;
pub use infrastructure::notification::email_adapter::email;
