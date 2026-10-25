use crate::stub::{ParamSet, Query, ResMut, SystemParam, World};

pub struct A;
pub struct B;
pub struct C;

pub fn two_resources(_a: ResMut<A>, _b: ResMut<B>) {}

pub fn three_writes(_q: Query<(&mut A, &mut B, &mut C)>) {}

pub fn hidden_in_set(_s: ParamSet<(ResMut<A>, ResMut<B>)>) {}

pub struct Bundle(pub ResMut<A>, pub ResMut<B>);

impl SystemParam for Bundle {}

pub fn hidden_in_bundle(_b: Bundle) {}

pub fn exclusive(_w: &mut World) {}
