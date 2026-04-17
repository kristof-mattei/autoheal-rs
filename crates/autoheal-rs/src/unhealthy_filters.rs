use hashbrown::{HashMap, HashSet};
use twistlock::filters::{Filters, Health};

pub fn build(autoheal_container_label_filter: Option<&str>) -> Filters {
    let rewritten = match autoheal_container_label_filter {
        Some("all") | None => None,
        Some(v) => {
            if let Some((left, right)) = v.split_once('=') {
                Some(HashMap::from_iter([(left.into(), Some(right.into()))]))
            } else {
                Some(HashMap::from_iter([(v.into(), Some("true".into()))]))
            }
        },
    };

    Filters {
        health: Some(HashSet::from_iter([Health::Unhealthy])),
        label: rewritten,
        ..Filters::default()
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::{HashMap, HashSet};
    use pretty_assertions::assert_eq;
    use twistlock::filters::Health;

    use crate::unhealthy_filters::build;

    #[test]
    fn build_filters_all() {
        let all_unhealthy = build(Some("all"));

        assert_eq!(
            all_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(all_unhealthy.label, None);
    }

    #[test]
    fn build_filters_autoheal() {
        let autoheal_and_unhealthy = build(Some("autoheal"));

        assert_eq!(
            autoheal_and_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(
            autoheal_and_unhealthy.label,
            Some(HashMap::from([("autoheal".into(), Some("true".into()))]))
        );
    }

    #[test]
    fn build_filters_custom() {
        let custom_and_unhealthy = build(Some("custom"));

        assert_eq!(
            custom_and_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(
            custom_and_unhealthy.label,
            Some(HashMap::from([("custom".into(), Some("true".into()))]))
        );
    }

    #[test]
    fn build_filters_custom_and_value_1() {
        let custom_and_unhealthy = build(Some("custom=true"));

        assert_eq!(
            custom_and_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(
            custom_and_unhealthy.label,
            Some(HashMap::from([("custom".into(), Some("true".into()))]))
        );
    }

    #[test]
    fn build_filters_custom_and_value_2() {
        let custom_and_unhealthy = build(Some("custom=false"));

        assert_eq!(
            custom_and_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(
            custom_and_unhealthy.label,
            Some(HashMap::from([("custom".into(), Some("false".into()))]))
        );
    }

    #[test]
    fn build_filters_custom_and_value_3() {
        let custom_and_unhealthy = build(Some("custom=foobar"));

        assert_eq!(
            custom_and_unhealthy.health,
            Some(HashSet::from([Health::Unhealthy]))
        );
        assert_eq!(
            custom_and_unhealthy.label,
            Some(HashMap::from([("custom".into(), Some("foobar".into()))]))
        );
    }
}
