//! Compound queries wrap other compound or leaf queries, either to combine
//! their results and scores, to change their behavior, or to switch from query
//! to filter context.

mod bool_query;
mod boosting_query;
mod constant_score_query;
mod dis_max_query;
mod function_score_query;

pub use self::{
    bool_query::*, boosting_query::*, constant_score_query::*, dis_max_query::*,
    function_score_query::*,
};
