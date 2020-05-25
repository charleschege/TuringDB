/*use tai64::TAI64N;
use anyhow::Result;

#[derive(Debug)]
pub struct ErrorLogger {
    error_type: anyhow::Error,
    time: TAI64N,
}

impl ErrorLogger {
    pub async fn new(data: anyhow::Error) -> Self {
        Self {
            error_type: data,
            time: TAI64N::now(),
        }
    }

    pub async fn log(self) -> Result<()> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push("TuringFeedsDB");
        log_file_path.push("errors.log");

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .await {
                Ok(mut file) => {
                    writeln!(file, "{:?}", self).await?;
                    Ok(())
                },
                Err(error) => {
                    match error.kind() {
                        ErrorKind::NotFound | ErrorKind::Interrupted | ErrorKind::UnexpectedEof | ErrorKind::InvalidInput | ErrorKind::InvalidData => {
                            let mut log_file_path = PathBuf::new();
                            log_file_path.push("TuringFeedsRepo");
                            log_file_path.push("errors.log");

                            let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(log_file_path)
                                .await?;
                            writeln!(file, "{:?}", self).await?;

                            Ok(())
                        },
                        _ => Err(anyhow::Error::new(error))
                    }
                }
            }
    }
}
*/