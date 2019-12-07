error_chain! {
    errors {
        LogCreateError(dir: String, msg: String) {
            description("failed to create log")
            display("creating the log failed at {}: '{}'", dir, msg)
        }

        LogOpenError(dir: String, msg: String) {
            description("failed to open log")
            display("opening log failed at {}: '{}'", dir, msg)
        }
    }
}
