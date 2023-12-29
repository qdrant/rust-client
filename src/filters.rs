use crate::qdrant;
use crate::qdrant::condition::ConditionOneOf;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::r#match::MatchValue;
use crate::qdrant::{
    Condition, FieldCondition, Filter, GeoBoundingBox, GeoPolygon, GeoRadius, HasIdCondition,
    IsEmptyCondition, IsNullCondition, NestedCondition, PointId, PointsSelector, Range,
    ValuesCount,
};

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

    /// create a Condition to initiate full text match
    ///
    /// # Examples:
    /// ```
    /// qdrant_client::qdrant::Condition::matches_text("description", "good cheap");
    /// ```
    pub fn matches_text(field: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                r#match: Some(qdrant::Match {
                    match_value: Some(MatchValue::Text(query.into())),
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

    /// create a Condition that checks geo fields against a geo polygons
    ///
    /// # Examples:
    ///
    /// ```
    /// use qdrant_client::qdrant::{GeoLineString, GeoPoint, GeoPolygon};
    /// let polygon = GeoPolygon {
    ///  exterior: Some(GeoLineString { points: vec![GeoPoint { lon: 42., lat: 42. }]}),
    ///  interiors: vec![],
    /// };
    /// qdrant_client::qdrant::Condition::geo_polygon("location", polygon);
    pub fn geo_polygon(field: impl Into<String>, geo_polygon: GeoPolygon) -> Self {
        Self {
            condition_one_of: Some(ConditionOneOf::Field(qdrant::FieldCondition {
                key: field.into(),
                geo_polygon: Some(geo_polygon),
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
        Self::Keyword(value)
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
