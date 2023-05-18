#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Country {
    pub name: String,
    pub code: String,
}

impl Country {
    pub fn from_google_location(location: &Option<serde_json::Value>) -> Option<Country> {
        let a = location.as_ref()?.as_str()?;
        let b: serde_json::Value = serde_json::from_str(a).ok()?;

        // j.place.address_components.find(c => c.types.includes("country")).short_name
        let address_components = b.get("place")?.get("address_components")?.as_array()?;

        // doing the Option<()> dance because Try isn't implemented for bool.
        fn is_country_component(c: &serde_json::Value) -> Option<()> {
            let is_country = c
                .get("types")?
                .as_array()?
                .iter()
                .any(|t| t.as_str().unwrap_or_default() == "country");
            match is_country {
                true => Some(()),
                false => None,
            }
        }
        let country_component = address_components
            .iter()
            .find(|c| is_country_component(c).is_some())?;

        let code = country_component.get("short_name")?.as_str()?;
        let name = country_component.get("long_name")?.as_str()?;

        Some(Country {
            name: name.to_string(),
            code: code.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::Country;

    #[test]
    fn gets_country_code_and_name() {
        let location_json: serde_json::Value = r#"
            {
                "input": "Fort Lee, NJ, USA",
                "place": {
                    "address_components": [
                        {
                            "long_name": "Fort Lee",
                            "short_name": "Fort Lee",
                            "types": [
                                "locality",
                                "political"
                            ]
                        },
                        {
                            "long_name": "Bergen County",
                            "short_name": "Bergen County",
                            "types": [
                                "administrative_area_level_2",
                                "political"
                            ]
                        },
                        {
                            "long_name": "New Jersey",
                            "short_name": "NJ",
                            "types": [
                                "administrative_area_level_1",
                                "political"
                            ]
                        },
                        {
                            "long_name": "United States",
                            "short_name": "US",
                            "types": [
                                "country",
                                "political"
                            ]
                        },
                        {
                            "long_name": "07024",
                            "short_name": "07024",
                            "types": [
                                "postal_code"
                            ]
                        }
                    ],
                    "html_attributions": []
                }
            }
        "#
        .into();

        let country = Country::from_google_location(&Some(location_json));

        assert_eq!(
            Some(Country {
                name: "United States".to_string(),
                code: "US".to_string()
            }),
            country
        )
    }
}
