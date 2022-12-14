use rstest::*;

use crate::constants::{PAGE_INPUT_MAX_LIMIT, PAGE_INPUT_MAX_OFFSET};
use crate::dto::*;
use crate::errors::CommonError;
use crate::test_common::test::init_test;

#[fixture]
pub fn setup() {
    init_test();
}

mod get_page_input {
    use super::*;

    #[rstest]
    fn test_get_page_input(_setup: ()) {
        let input = GetPageInput {
            limit: 10,
            offset: 0,
        };
        assert_eq!(input.validate(), Ok(()));
    }

    #[rstest]
    fn test_get_page_input_limit_overflow(_setup: ()) {
        let input = GetPageInput {
            limit: PAGE_INPUT_MAX_LIMIT + 1,
            offset: 0,
        };
        assert_eq!(
            input.validate(),
            Err(CommonError::ValueShouldBeInRangeError {
                field: "limit".to_string(),
                min: 1,
                max: PAGE_INPUT_MAX_LIMIT,
            })
        );
    }

    #[rstest]
    fn test_get_page_input_offset_overflow(_setup: ()) {
        let input = GetPageInput {
            limit: 100,
            offset: PAGE_INPUT_MAX_OFFSET + 1,
        };
        assert_eq!(
            input.validate(),
            Err(CommonError::ValueShouldBeInRangeError {
                field: "offset".to_string(),
                min: 0,
                max: PAGE_INPUT_MAX_OFFSET,
            })
        );
    }
}
