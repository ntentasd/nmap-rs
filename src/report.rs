use crate::scanner::{ScanResult, ScanStatus};

impl std::fmt::Display for ScanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanStatus::Open => write!(f, "open"),
            ScanStatus::Closed => write!(f, "closed"),
        }
    }
}

impl std::fmt::Display for ScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.tries() > 1 { "ies" } else { "y" };
        write!(
            f,
            "{}:{}\t{}\t({} tr{})",
            self.address(),
            self.port(),
            self.status(),
            self.tries(),
            suffix
        )
    }
}
