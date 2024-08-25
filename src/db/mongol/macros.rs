#[macro_export]
macro_rules! transaction_error {
    ($mongo_err:expr) => {
        $crate::server_error!(
            $crate::model::error::Kind::Unexpected,
            $crate::model::error::OnType::Transaction
        )
        .add_debug_info(
            "mongo transaction error",
            $mongo_err.to_string(),
        )
    };
}
