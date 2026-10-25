use crate::stub::{ParamSet, Query, ResMut};

pub struct A;
pub struct B;

pub fn within_limits(_resource: ResMut<A>, _q: Query<(&mut A, &mut B)>) {}

pub fn set_within_limits(_s: ParamSet<(Query<&mut A>, Query<&B>)>) {}
