use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};

use crate::qdrant;
use crate::qdrant::condition::ConditionOneOf;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::r#match::MatchValue;
use crate::qdrant::{
    Condition, FieldCondition, Filter, GeoBoundingBox, GeoRadius, HasIdCondition, IsEmptyCondition,
    IsNullCondition, NestedCondition, PointId, PointsSelector, Range, ValuesCount,
};

macro_rules! impl_from_std_range {
    ($this:ty; $($from:ty),+) => {
        $(
            impl From<$from> for $this {
                fn from(value: $from) -> Self {
                    Self::from_bounds(value.start_bound(), value.end_bound())
                }
            }
        )+
    };
}

impl_from_std_range!(
    Range;
    std::ops::Range<f64>,
    std::ops::RangeFrom<f64>,
    std::ops::RangeTo<f64>,
    std::ops::RangeInclusive<f64>,
    std::ops::RangeToInclusive<f64>,
    std::ops::RangeFull
);

impl_from_std_range!(
    ValuesCount;
    std::ops::Range<u64>,
    std::ops::RangeFrom<u64>,
    std::ops::RangeTo<u64>,
    std::ops::RangeInclusive<u64>,
    std::ops::RangeToInclusive<u64>,
    std::ops::RangeFull
);

macro_rules! impl_common {
    ($this:ty, $num:ty) => {
        impl $this {
            #[doc=concat!("Returns an empty `", stringify!($this),"` with unsatisfiable bounds. \
            An empty `", stringify!($this), "` will match nothing.")]
            #[doc=""]
            #[doc=concat!("Note that there are many representations of empty ranges. Do not use `some_range == ",
            stringify!($this), "::empty()` to check if a range is empty; use `is_empty` instead."
            )]
            pub fn empty() -> Self {
                Self::from_bounds(Bound::Excluded(&Default::default()), Bound::Excluded(&Default::default()))
            }

            /// Returns a range that matches only the specified value.
            pub fn only(val: $num) -> Self {
                Self::from_bounds(Bound::Included(&val), Bound::Included(&val))
            }

            /// Returns a range that matches any value.
            pub fn any() -> Self {
                Self::default()
            }

            /// Construct a range by explicitly specifying both of its bounds.
            pub fn from_bounds(lo: Bound<&$num>, hi: Bound<&$num>) -> Self {
                let mut range = Self::default();

                match lo {
                    Bound::Included(gte) => {
                        range.gte = Some(*gte);
                    }
                    Bound::Excluded(gt) => {
                        range.gt = Some(*gt);
                    }
                    Bound::Unbounded => {}
                }

                match hi {
                    Bound::Included(lte) => {
                        range.lte = Some(*lte);
                    }
                    Bound::Excluded(lt) => {
                        range.lt = Some(*lt);
                    }
                    Bound::Unbounded => {}
                }

                range
            }

            /// Normalizes the representation of a given range
            /// by removing redundant constraints.
            /// 
            /// Redundant constraints occur when both `gt` and `gte`,
            /// or both `lt` and `lte` fields are specified. In this case,
            /// at least one of them will never be satisfied, so it can safely
            /// be removed, leaving the stricter constraint.
            ///
            /// This is mostly useful for debugging purposes. At the moment,
            /// filters do not require that they are passed collapsed ranges.
            ///
            /// You don't need to call this function if you construct
            /// ranges by calling the library functions,
            /// as they never produce ranges with redundant constraints.
            pub fn collapse(&mut self) {
                *self = Self::from_bounds(self.start_bound(), self.end_bound())
            }

            /// Consumes a given range and returns its collapsed version.
            ///
            /// See `collapse` for details.
            pub fn collapsed(mut self) -> Self {
                self.collapse();
                self
            }

            /// Consumes a given range and returns its intersection with `other`.
            pub fn intersection(mut self, other: &Self) -> Self {
                self.intersect(other);
                self
            }

            /// Returns `true` if `val` is contained in the range.
            pub fn contains(&self, val: $num) -> bool {
                <Self as RangeBounds<$num>>::contains(self, &val)
            }

            /// A convenience method to exclude the lower bound
            /// for ranges constructed `From` standard library range types.
            pub fn exclude_lower(&mut self) {
                self.collapse();
                self.gt = self.gte.or(self.gt);
                self.gte = None;
            }

            /// A convenience function to exclude the lower bound
            /// for ranges constructed `From` standard library range types.
            pub fn excluding_lower(mut self) -> Self {
                self.exclude_lower();
                self
            }
        }

        impl RangeBounds<$num> for $this {
            fn start_bound(&self) -> Bound<&$num> {
                match (&self.gt, &self.gte) {
                    (None, None) => Bound::Unbounded,
                    (None, Some(b)) => Bound::Included(b),
                    (Some(b), None) => Bound::Excluded(b),
                    (Some(gt), Some(gte)) => {
                        if gt < gte {
                            Bound::Included(gte)
                        } else {
                            Bound::Excluded(gt)
                        }
                    }
                }
            }

            fn end_bound(&self) -> std::ops::Bound<&$num> {
                match (&self.lt, &self.lte) {
                    (None, None) => Bound::Unbounded,
                    (None, Some(b)) => Bound::Included(b),
                    (Some(b), None) => Bound::Excluded(b),
                    (Some(lt), Some(lte)) => {
                        if lt > lte {
                            Bound::Included(lte)
                        } else {
                            Bound::Excluded(lt)
                        }
                    }
                }
            }
        }
    };
}

impl_common!(Range, f64);
impl_common!(ValuesCount, u64);

impl Range {
    /// A range is empty if no possible `f64` value can match against it.
    ///
    /// You may use this to avoid passing unsatisfiable ranges to filters and conditions.
    pub fn is_empty(&self) -> bool {
        match (self.start_bound(), self.end_bound()) {
            (Bound::Excluded(lo), Bound::Excluded(hi)) => {
                lo.partial_cmp(hi).map_or(true, |o| o != Ordering::Less)
            }

            (Bound::Included(lo), Bound::Excluded(hi))
            | (Bound::Excluded(lo), Bound::Included(hi))
            | (Bound::Included(lo), Bound::Included(hi)) => {
                lo.partial_cmp(hi).map_or(true, |o| o == Ordering::Greater)
            }

            (Bound::Excluded(b), Bound::Unbounded) => b.is_nan() || *b == f64::INFINITY,
            (Bound::Unbounded, Bound::Excluded(b)) => b.is_nan() || *b == f64::NEG_INFINITY,

            (Bound::Included(b), Bound::Unbounded) | (Bound::Unbounded, Bound::Included(b)) => {
                b.is_nan()
            }

            (Bound::Unbounded, Bound::Unbounded) => false,
        }
    }

    /// Intersect a given range with another range.
    ///
    /// The result is a range that contains `a` if and only if
    /// `self` contains `a` and `other` contains `a`.
    pub fn intersect(&mut self, other: &Self) {
        fn intersect_one(
            bound1: Option<f64>,
            bound2: Option<f64>,
            cmp: impl Fn(f64, f64) -> f64,
        ) -> Option<f64> {
            match (bound1, bound2) {
                (Some(b1), Some(b2)) => {
                    // We have to specially handle NaNs because
                    // the normal comparators return the other operand
                    // if one of them is NaN.
                    if b1.is_nan() || b2.is_nan() {
                        Some(f64::NAN)
                    } else {
                        Some(cmp(b1, b2))
                    }
                }
                _ => bound1.or(bound2),
            }
        }

        self.gt = intersect_one(self.gt, other.gt, f64::max);
        self.gte = intersect_one(self.gte, other.gte, f64::max);
        self.lt = intersect_one(self.lt, other.lt, f64::min);
        self.lte = intersect_one(self.lte, other.lte, f64::min);

        self.collapse();
    }
}

impl ValuesCount {
    /// A range is empty if no possible `u64` value can match against it.
    ///
    /// You may use this to avoid creating unsatisfiable filters and conditions.
    pub fn is_empty(&self) -> bool {
        match (self.start_bound(), self.end_bound()) {
            (Bound::Excluded(lo), Bound::Excluded(hi)) => lo >= hi || lo.abs_diff(*hi) <= 1,

            (Bound::Included(lo), Bound::Excluded(hi))
            | (Bound::Excluded(lo), Bound::Included(hi))
            | (Bound::Included(lo), Bound::Included(hi)) => lo > hi,

            _ => false,
        }
    }

    /// Intersect a given range with another range.
    ///
    /// The result is a range that contains `a` if and only if
    /// `self` contains `a` and `other` contains `a`.
    pub fn intersect(&mut self, other: &Self) {
        fn intersect_one(
            bound1: Option<u64>,
            bound2: Option<u64>,
            cmp: impl Fn(u64, u64) -> u64,
        ) -> Option<u64> {
            match (bound1, bound2) {
                (Some(b1), Some(b2)) => Some(cmp(b1, b2)),
                _ => bound1.or(bound2),
            }
        }

        self.gt = intersect_one(self.gt, other.gt, u64::max);
        self.gte = intersect_one(self.gte, other.gte, u64::max);
        self.lt = intersect_one(self.lt, other.lt, u64::min);
        self.lte = intersect_one(self.lte, other.lte, u64::min);

        self.collapse();
    }
}

impl From<Filter> for PointsSelector {
    fn from(filter: Filter) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Filter(filter)),
        }
    }
}

impl From<FieldCondition> for Condition {
    fn from(field_condition: FieldCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::Field(field_condition)),
        }
    }
}

impl From<IsEmptyCondition> for Condition {
    fn from(is_empty_condition: IsEmptyCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::IsEmpty(is_empty_condition)),
        }
    }
}

impl From<IsNullCondition> for Condition {
    fn from(is_null_condition: IsNullCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::IsNull(is_null_condition)),
        }
    }
}

impl From<HasIdCondition> for Condition {
    fn from(has_id_condition: HasIdCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::HasId(has_id_condition)),
        }
    }
}

impl From<Filter> for Condition {
    fn from(filter: Filter) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::Filter(filter)),
        }
    }
}

impl From<NestedCondition> for Condition {
    fn from(nested_condition: NestedCondition) -> Self {
        debug_assert!(
            !&nested_condition
                .filter
                .as_ref()
                .map_or(false, |f| f.check_has_id()),
            "Filters containing a `has_id` condition are not supported for nested filtering."
        );

        Condition {
            condition_one_of: Some(ConditionOneOf::Nested(nested_condition)),
        }
    }
}

impl qdrant::Filter {
    /// Checks if the filter, or any of its nested conditions containing filters,
    /// have a `has_id` condition, which is not allowed for nested object filters.
    fn check_has_id(&self) -> bool {
        self.should
            .iter()
            .chain(self.must.iter())
            .chain(self.must_not.iter())
            .any(|cond| match &cond.condition_one_of {
                Some(ConditionOneOf::HasId(_)) => true,
                Some(ConditionOneOf::Nested(nested)) => nested
                    .filter
                    .as_ref()
                    .map_or(false, |filter| filter.check_has_id()),
                Some(ConditionOneOf::Filter(filter)) => filter.check_has_id(),
                _ => false,
            })
    }

    /// create a Filter where all of the conditions must be satisfied
    pub fn must(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self {
            must: conds.into_iter().collect(),
            ..Default::default()
        }
    }

    /// create a Filter where at least one of the conditions should be satisfied
    pub fn should(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self {
            should: conds.into_iter().collect(),
            ..Default::default()
        }
    }

    /// create a Filter where none of the conditions must be satisfied
    pub fn must_not(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self {
            must_not: conds.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Alias for [`should`](Self::should)
    /// create a Filter that matches if any of the conditions match
    pub fn any(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self::should(conds)
    }

    /// Alias for [`must`](Self::must)
    /// create a Filter that matches if all of the conditions match
    pub fn all(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self::must(conds)
    }

    /// Alias for [`must_not`](Self::must_not)
    /// create a Filter that matches if none of the conditions match
    pub fn none(conds: impl IntoIterator<Item = qdrant::Condition>) -> Self {
        Self::must_not(conds)
    }
}

impl qdrant::Condition {
    /// create a Condition to check if a field is empty
    ///
    /// # Examples:
    /// ```
    /// qdrant_client::qdrant::Condition::is_empty("field");
    /// ```
    pub fn is_empty(key: impl Into<String>) -> Self {
        Self::from(qdrant::IsEmptyCondition { key: key.into() })
    }

    /// create a Condition to check if the point has a null key
    ///
    /// # Examples:
    /// ```
    /// qdrant_client::qdrant::Condition::is_empty("remark");
    /// ```
    pub fn is_null(key: impl Into<String>) -> Self {
        Self::from(qdrant::IsNullCondition { key: key.into() })
    }

    /// create a Condition to check if the point has one of the given ids
    ///
    /// # Examples:
    /// ```
    /// qdrant_client::qdrant::Condition::has_id([0, 8, 15]);
    /// ```
    pub fn has_id(ids: impl IntoIterator<Item = impl Into<PointId>>) -> Self {
        Self::from(qdrant::HasIdCondition {
            has_id: ids.into_iter().map(Into::into).collect(),
        })
    }

    /// create a Condition that matches a field against a certain value
    ///
    /// # Examples:
    /// ```
    /// qdrant_client::qdrant::Condition::matches("number", 42);
    /// qdrant_client::qdrant::Condition::matches("tag", vec!["i".to_string(), "em".into()]);
    /// ```
    pub fn matches(field: impl Into<String>, r#match: impl Into<MatchValue>) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                r#match: Some(qdrant::Match {
                    match_value: Some(r#match.into()),
                }),
                ..Default::default()
            })),
        }
    }

    /// create a Condition that checks numeric fields against a range
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::Range;
    /// qdrant_client::qdrant::Condition::range("number", Range {
    ///     gte: Some(42.),
    ///     ..Default::default()
    /// });
    /// ```
    pub fn range(field: impl Into<String>, range: Range) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                range: Some(range),
                ..Default::default()
            })),
        }
    }

    /// create a Condition that checks geo fields against a radius
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::{GeoPoint, GeoRadius};
    /// qdrant_client::qdrant::Condition::geo_radius("location", GeoRadius {
    ///   center: Some(GeoPoint { lon: 42., lat: 42. }),
    ///   radius: 42.,
    /// });
    pub fn geo_radius(field: impl Into<String>, geo_radius: GeoRadius) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                geo_radius: Some(geo_radius),
                ..Default::default()
            })),
        }
    }

    /// create a Condition that checks geo fields against a bounding box
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::{GeoPoint, GeoBoundingBox};
    /// qdrant_client::qdrant::Condition::geo_bounding_box("location", GeoBoundingBox {
    ///   top_left: Some(GeoPoint { lon: 42., lat: 42. }),
    ///   bottom_right: Some(GeoPoint { lon: 42., lat: 42. }),
    /// });
    pub fn geo_bounding_box(field: impl Into<String>, geo_bounding_box: GeoBoundingBox) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                geo_bounding_box: Some(geo_bounding_box),
                ..Default::default()
            })),
        }
    }

    /// create a Condition that checks count of values in a field
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::ValuesCount;
    /// qdrant_client::qdrant::Condition::values_count("tags", ValuesCount {
    ///  gte: Some(42),
    ///  ..Default::default()
    /// });
    pub fn values_count(field: impl Into<String>, values_count: ValuesCount) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                values_count: Some(values_count),
                ..Default::default()
            })),
        }
    }

    /// create a Condition that applies a per-element filter to a nested array
    ///
    /// The `field` parameter should be a key-path to a nested array of objects.
    /// You may specify it as both `array_field` or `array_field[]`.
    ///
    /// For motivation and further examples,
    /// see [API documentation](https://qdrant.tech/documentation/concepts/filtering/#nested-object-filter).
    ///
    /// # Panics:
    ///
    /// If debug assertions are enabled, this will panic if the filter, or any its subfilters,
    /// contain a `HasIdCondition` (equivalently, a condition created with `Self::has_id`),
    /// as these are unsupported for nested object filters.
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::Filter;
    /// qdrant_client::qdrant::Condition::nested("array_field[]", Filter::any([
    ///   qdrant_client::qdrant::Condition::is_null("element_field")
    /// ]));
    pub fn nested(field: impl Into<String>, filter: Filter) -> Self {
        Self::from(NestedCondition {
            key: field.into(),
            filter: Some(filter),
        })
    }
}

impl From<bool> for MatchValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for MatchValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for MatchValue {
    fn from(value: String) -> Self {
        if value.contains(char::is_whitespace) {
            Self::Text(value)
        } else {
            Self::Keyword(value)
        }
    }
}

impl From<Vec<i64>> for MatchValue {
    fn from(integers: Vec<i64>) -> Self {
        Self::Integers(qdrant::RepeatedIntegers { integers })
    }
}

impl From<Vec<String>> for MatchValue {
    fn from(strings: Vec<String>) -> Self {
        Self::Keywords(qdrant::RepeatedStrings { strings })
    }
}

impl std::ops::Not for MatchValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Keyword(s) => Self::ExceptKeywords(qdrant::RepeatedStrings { strings: vec![s] }),
            Self::Integer(i) => {
                Self::ExceptIntegers(qdrant::RepeatedIntegers { integers: vec![i] })
            }
            Self::Boolean(b) => Self::Boolean(!b),
            Self::Keywords(ks) => Self::ExceptKeywords(ks),
            Self::Integers(is) => Self::ExceptIntegers(is),
            Self::ExceptKeywords(ks) => Self::Keywords(ks),
            Self::ExceptIntegers(is) => Self::Integers(is),
            Self::Text(_) => panic!("cannot negate a MatchValue::Text"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::qdrant::{Condition, Filter, NestedCondition};

    #[test]
    fn test_nested_has_id() {
        assert!(!Filter::any([]).check_has_id());
        assert!(Filter::any([Condition::has_id([0])]).check_has_id());

        // nested filter
        assert!(Filter::any([Filter::any([Condition::has_id([0])]).into()]).check_has_id());

        // nested filter where only the innermost has a `has_id`
        assert!(
            Filter::any([Filter::any([Filter::any([Condition::has_id([0])]).into()]).into()])
                .check_has_id()
        );

        // `has_id` itself nested in a nested condition
        assert!(Filter::any([Condition {
            condition_one_of: Some(crate::qdrant::condition::ConditionOneOf::Nested(
                NestedCondition {
                    key: "test".to_string(),
                    filter: Some(Filter::any([Condition::has_id([0])]))
                }
            ))
        }])
        .check_has_id());
    }

    #[test]
    #[should_panic]
    fn test_nested_condition_validation() {
        let _ = Filter::any([Condition::nested(
            "test",
            Filter::any([Condition::has_id([0])]),
        )]);
    }
}

#[cfg(test)]
mod range_tests {
    use super::Range;

    #[test]
    fn test_empty() {
        assert!(Range::empty().is_empty());
        assert!(Range::from(0.0..0.0).excluding_lower().is_empty());
        assert!(Range::from(1.0..0.0).excluding_lower().is_empty());
        assert!(Range::from(1.0..-1.0).excluding_lower().is_empty());
    }

    #[test]
    fn test_empty_edge_cases() {
        assert!(Range::from(f64::NAN..).is_empty());
        assert!(Range::only(f64::NAN).is_empty());
        assert!(Range::from(..f64::NEG_INFINITY).is_empty());
        assert!(Range::from(f64::INFINITY..).excluding_lower().is_empty());

        // These are non-empty because they still can match the infinities.
        assert!(!Range::from(..=f64::INFINITY).is_empty());
        assert!(!Range::from(f64::INFINITY..).is_empty());
    }

    #[test]
    fn test_contains() {
        assert!(Range::any().contains(0.0));
        assert!(Range::only(0.0).contains(0.0));
        assert!(!Range::only(0.0).contains(1.0));
    }

    #[test]
    fn test_contains_edge_cases() {
        assert!(Range::from(..=f64::INFINITY).contains(f64::INFINITY));
        assert!(Range::from(f64::INFINITY..).contains(f64::INFINITY));

        assert!(!Range::from(f64::INFINITY..)
            .excluding_lower()
            .contains(f64::INFINITY));
        assert!(!Range::from(..f64::INFINITY).contains(f64::INFINITY));

        let test_vals = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, 1.0, -1.0];
        assert!(test_vals
            .iter()
            .all(|&val| !Range::only(f64::NAN).contains(val)));
        assert!(test_vals
            .iter()
            .all(|&val| !Range::from(f64::NAN..).contains(val)));
        assert!(test_vals
            .iter()
            .all(|&val| !Range::from(f64::INFINITY..).excluding_lower().contains(val)));
        assert!(test_vals
            .iter()
            .all(|&val| !Range::from(..f64::NEG_INFINITY).contains(val)));
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            Range::from(0.0..1.0)
                .excluding_lower()
                .intersection(&Range::from(0.0..2.0).excluding_lower()),
            Range::from(0.0..1.0).excluding_lower()
        );

        assert_eq!(
            Range::from(0.0..5.0)
                .excluding_lower()
                .intersection(&Range::from(1.0..2.0).excluding_lower()),
            Range::from(1.0..2.0).excluding_lower()
        );

        assert_eq!(
            Range::from(..5.0).intersection(&Range::from(1.0..)),
            Range::from(1.0..5.0)
        );

        assert!(Range::from(..=f64::NAN)
            .intersection(&Range::from(0.0..=1.0))
            .lte
            .unwrap()
            .is_nan());

        assert!(Range::from(..f64::NAN)
            .intersection(&Range::from(0.0..=1.0))
            .lt
            .unwrap()
            .is_nan());
    }

    #[test]
    fn test_collapse() {
        {
            let mut range = Range {
                gt: Some(0.0),
                gte: Some(0.0),
                ..Default::default()
            };
            range.collapse();
            assert_eq!(range, Range::from(0.0..).excluding_lower());
        }

        {
            let mut range = Range {
                lt: Some(0.0),
                lte: Some(0.0),
                ..Default::default()
            };
            range.collapse();
            assert_eq!(range, Range::from(..0.0));
        }

        {
            let mut range = Range {
                gt: Some(0.0),
                gte: Some(-1.0),
                lt: Some(2.0),
                lte: Some(1.0),
            };
            range.collapse();
            assert_eq!(range, Range::from(0.0..=1.0).excluding_lower());
        }
    }
}

#[cfg(test)]
mod values_count_tests {
    use super::ValuesCount;

    #[test]
    fn test_empty() {
        assert!(ValuesCount::empty().is_empty());

        assert!(ValuesCount::from(0..0).excluding_lower().is_empty());
        assert!(ValuesCount::from(1..0).excluding_lower().is_empty());
        assert!(ValuesCount::from(0..1).excluding_lower().is_empty());

        assert!(!ValuesCount::from(0..=1).excluding_lower().is_empty());
        assert!(!ValuesCount::from(0..1).is_empty());
        assert!(!ValuesCount::from(0..=1).is_empty());
    }

    #[test]
    fn test_contains() {
        assert!(ValuesCount::any().contains(0));
        assert!(ValuesCount::only(0).contains(0));

        assert!(ValuesCount::from(0..=1).excluding_lower().contains(1));
        assert!(ValuesCount::from(0..=1).contains(0));

        assert!(!ValuesCount::from(0..0).excluding_lower().contains(0));
        assert!(!ValuesCount::from(1..0).excluding_lower().contains(0));
        assert!(!ValuesCount::from(0..1).excluding_lower().contains(0));
        assert!(!ValuesCount::from(0..1).excluding_lower().contains(1));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            ValuesCount::from(0..1)
                .excluding_lower()
                .intersection(&ValuesCount::from(0..2).excluding_lower()),
            ValuesCount::from(0..1).excluding_lower()
        );

        assert_eq!(
            ValuesCount::from(0..5).intersection(&ValuesCount::from(1..2)),
            ValuesCount::from(1..2)
        );

        assert_eq!(
            ValuesCount::from(..5).intersection(&ValuesCount::from(1..)),
            ValuesCount::from(1..5)
        );
    }
}
