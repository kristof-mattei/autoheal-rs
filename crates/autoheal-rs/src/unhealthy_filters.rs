pub fn build(autoheal_container_label_filter: Option<&str>) -> serde_json::Value {
    let rewritten: Option<String> = match autoheal_container_label_filter {
        Some("all") | None => None,
        Some(v) => {
            if v.contains('=') {
                Some(v.into())
            } else {
                Some(format!("{}=true", v))
            }
        },
    };

    let json = serde_json::Map::from_iter([
        ("health".into(), vec!["unhealthy"].into()),
        (
            "label".into(),
            vec![rewritten]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .into(),
        ),
    ]);

    serde_json::Value::Object(json)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::unhealthy_filters::build;

    #[test]
    fn build_filters_all() {
        let all_unhealthy = build(Some("all"));

        assert_eq!(
            all_unhealthy,
            json!({ "health": ["unhealthy"], "label": [] })
        );
    }

    #[test]
    fn build_filters_autoheal() {
        let autoheal_and_unhealthy = build(Some("autoheal"));

        assert_eq!(
            autoheal_and_unhealthy,
            json!({ "health": ["unhealthy"], "label": ["autoheal=true"] })
        );
    }

    #[test]
    fn build_filters_custom() {
        let custom_and_unhealthy = build(Some("custom"));

        assert_eq!(
            custom_and_unhealthy,
            json!({ "health": ["unhealthy"], "label": ["custom=true"] })
        );
    }

    #[test]
    fn build_filters_custom_and_value_1() {
        let custom_and_unhealthy = build(Some("custom=true"));

        assert_eq!(
            custom_and_unhealthy,
            json!({ "health": ["unhealthy"], "label": ["custom=true"] })
        );
    }

    #[test]
    fn build_filters_custom_and_value_2() {
        let custom_and_unhealthy = build(Some("custom=false"));

        assert_eq!(
            custom_and_unhealthy,
            json!({ "health": ["unhealthy"], "label": ["custom=false"] })
        );
    }

    #[test]
    fn build_filters_custom_and_value_3() {
        let custom_and_unhealthy = build(Some("custom=foobar"));

        assert_eq!(
            custom_and_unhealthy,
            json!({ "health": ["unhealthy"], "label": ["custom=foobar"] })
        );
    }
}
