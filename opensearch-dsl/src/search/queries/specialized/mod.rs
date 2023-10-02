//! This group contains queries which do not fit into the other groups

mod distance_feature_query;
mod more_like_this_query;
mod percolate_lookup_query;
mod percolate_query;
mod pinned_query;
mod rank_feature_query;
mod script_query;
mod script_score_query;
mod wrapper_query;

pub use self::{
  distance_feature_query::*, more_like_this_query::*, percolate_lookup_query::*, percolate_query::*, pinned_query::*,
  rank_feature_query::*, script_query::*, script_score_query::*, wrapper_query::*,
};
