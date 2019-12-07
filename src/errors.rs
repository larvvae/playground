use super::kf_log;

error_chain! {
    links {
        KfLog(kf_log::errors::Error, kf_log::errors::ErrorKind);
    }
}
