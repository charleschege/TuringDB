use turingdb_helpers::TuringOp;
use custom_codes::DbOps;

pub (crate) async fn format_error(op: &TuringOp, error: &anyhow::Error) -> DbOps {
    let unhandled_error = format!("[TuringDB::<{:?}>::(ERROR)-{:?}]", op, custom_codes::try_downcast(error));
    DbOps::EncounteredErrors(unhandled_error)
}