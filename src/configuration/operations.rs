pub struct OperationsRange {
    pub start: u8,
    pub end: u8,
}

pub const DEFAULT_ADDITION_RANGE: OperationsRange = OperationsRange {
    start: 1u8,
    end: 20u8,
};

pub const DEFAULT_SUBTRACTION_RANGE: OperationsRange = OperationsRange {
    start: 1u8,
    end: 20u8,
};

pub const DEFAULT_MULTIPLICATION_RANGE: OperationsRange = OperationsRange {
    start: 1u8,
    end: 10u8,
};

pub const DEFAULT_DIVISION_RANGE: OperationsRange = OperationsRange {
    start: 1u8,
    end: 10u8,
};

pub struct OperationsConfig {
    pub addition_range: OperationsRange,
    pub subtraction_range: OperationsRange,
    pub multiplication_range: OperationsRange,
    pub division_range: OperationsRange,
}

impl Default for OperationsConfig {
    fn default() -> Self {
        OperationsConfig {
            addition_range: DEFAULT_ADDITION_RANGE,
            subtraction_range: DEFAULT_SUBTRACTION_RANGE,
            multiplication_range: DEFAULT_MULTIPLICATION_RANGE,
            division_range: DEFAULT_DIVISION_RANGE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OperationsConfig, DEFAULT_ADDITION_RANGE, DEFAULT_DIVISION_RANGE,
        DEFAULT_MULTIPLICATION_RANGE, DEFAULT_SUBTRACTION_RANGE,
    };

    #[test]
    fn test_default_config() {
        let default_config = OperationsConfig::default();
        assert_eq!(
            default_config.addition_range.start,
            DEFAULT_ADDITION_RANGE.start
        );
        assert_eq!(
            default_config.addition_range.end,
            DEFAULT_ADDITION_RANGE.end
        );
        assert_eq!(
            default_config.subtraction_range.start,
            DEFAULT_SUBTRACTION_RANGE.start
        );
        assert_eq!(
            default_config.subtraction_range.end,
            DEFAULT_SUBTRACTION_RANGE.end
        );
        assert_eq!(
            default_config.multiplication_range.start,
            DEFAULT_MULTIPLICATION_RANGE.start
        );
        assert_eq!(
            default_config.multiplication_range.end,
            DEFAULT_MULTIPLICATION_RANGE.end
        );
        assert_eq!(
            default_config.division_range.start,
            DEFAULT_DIVISION_RANGE.start
        );
        assert_eq!(
            default_config.division_range.end,
            DEFAULT_DIVISION_RANGE.end
        );
    }
}
