#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Other types that this type depends on"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Other types that this type depends on\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Dependencies(pub ::std::vec::Vec<TypeSchemaIdentifier>);
impl ::std::ops::Deref for Dependencies {
    type Target = ::std::vec::Vec<TypeSchemaIdentifier>;
    fn deref(&self) -> &::std::vec::Vec<TypeSchemaIdentifier> {
        &self.0
    }
}
impl ::std::convert::From<Dependencies> for ::std::vec::Vec<TypeSchemaIdentifier> {
    fn from(value: Dependencies) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Dependencies> for Dependencies {
    fn from(value: &Dependencies) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<TypeSchemaIdentifier>> for Dependencies {
    fn from(value: ::std::vec::Vec<TypeSchemaIdentifier>) -> Self {
        Self(value)
    }
}
#[doc = "Defines how a field is bound to a terminology value set"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines how a field is bound to a terminology value set\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"strength\","]
#[doc = "    \"valueset\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"strength\": {"]
#[doc = "      \"description\": \"The binding strength: required (must be from valueset), extensible (should be from valueset), preferred (encouraged), example (illustration only)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"required\","]
#[doc = "        \"extensible\","]
#[doc = "        \"preferred\","]
#[doc = "        \"example\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueset\": {"]
#[doc = "      \"description\": \"Reference to the value set containing allowed values\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/with-valueset-kind\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$comment\": \"https://build.fhir.org/terminologies-binding-examples.html\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldBinding {
    #[doc = "The binding strength: required (must be from valueset), extensible (should be from valueset), preferred (encouraged), example (illustration only)"]
    pub strength: FieldBindingStrength,
    pub valueset: FieldBindingValueset,
}
impl ::std::convert::From<&FieldBinding> for FieldBinding {
    fn from(value: &FieldBinding) -> Self {
        value.clone()
    }
}
impl FieldBinding {
    pub fn builder() -> builder::FieldBinding {
        Default::default()
    }
}
#[doc = "The binding strength: required (must be from valueset), extensible (should be from valueset), preferred (encouraged), example (illustration only)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The binding strength: required (must be from valueset), extensible (should be from valueset), preferred (encouraged), example (illustration only)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"required\","]
#[doc = "    \"extensible\","]
#[doc = "    \"preferred\","]
#[doc = "    \"example\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FieldBindingStrength {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "extensible")]
    Extensible,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "example")]
    Example,
}
impl ::std::convert::From<&Self> for FieldBindingStrength {
    fn from(value: &FieldBindingStrength) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FieldBindingStrength {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Required => write!(f, "required"),
            Self::Extensible => write!(f, "extensible"),
            Self::Preferred => write!(f, "preferred"),
            Self::Example => write!(f, "example"),
        }
    }
}
impl ::std::str::FromStr for FieldBindingStrength {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "required" => Ok(Self::Required),
            "extensible" => Ok(Self::Extensible),
            "preferred" => Ok(Self::Preferred),
            "example" => Ok(Self::Example),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FieldBindingStrength {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FieldBindingStrength {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FieldBindingStrength {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reference to the value set containing allowed values"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Reference to the value set containing allowed values\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/with-valueset-kind\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldBindingValueset {
    pub kind: FieldBindingValuesetKind,
    #[doc = "Computer friendly identifier for the element"]
    pub name: ::std::string::String,
    #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
    pub package: ::std::string::String,
    #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
    pub url: FieldBindingValuesetUrl,
    #[doc = "The version of the package"]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&FieldBindingValueset> for FieldBindingValueset {
    fn from(value: &FieldBindingValueset) -> Self {
        value.clone()
    }
}
impl FieldBindingValueset {
    pub fn builder() -> builder::FieldBindingValueset {
        Default::default()
    }
}
#[doc = "FieldBindingValuesetKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"value-set\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FieldBindingValuesetKind {
    #[serde(rename = "value-set")]
    ValueSet,
}
impl ::std::convert::From<&Self> for FieldBindingValuesetKind {
    fn from(value: &FieldBindingValuesetKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FieldBindingValuesetKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ValueSet => write!(f, "value-set"),
        }
    }
}
impl ::std::str::FromStr for FieldBindingValuesetKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "value-set" => Ok(Self::ValueSet),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FieldBindingValuesetKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FieldBindingValuesetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FieldBindingValuesetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FieldBindingValuesetUrl(::std::string::String);
impl ::std::ops::Deref for FieldBindingValuesetUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FieldBindingValuesetUrl> for ::std::string::String {
    fn from(value: FieldBindingValuesetUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FieldBindingValuesetUrl> for FieldBindingValuesetUrl {
    fn from(value: &FieldBindingValuesetUrl) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FieldBindingValuesetUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FieldBindingValuesetUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FieldBindingValuesetUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FieldBindingValuesetUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FieldBindingValuesetUrl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The base declaration for a FHIR choice type (e.g., value[x])"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Polymorphic value[x] field declaration\","]
#[doc = "  \"description\": \"The base declaration for a FHIR choice type (e.g., value[x])\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"choices\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"array\": {"]
#[doc = "      \"description\": \"Whether the selected choice can contain multiple values\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"choices\": {"]
#[doc = "      \"description\": \"The names of all concrete type options for this choice field\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"excluded\": {"]
#[doc = "      \"description\": \"Whether all choices are prohibited\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether at least one choice must be provided\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldPolymorphicDeclaration {
    #[doc = "Whether the selected choice can contain multiple values"]
    #[serde(default)]
    pub array: bool,
    #[doc = "The names of all concrete type options for this choice field"]
    pub choices: ::std::vec::Vec<::std::string::String>,
    #[doc = "Whether all choices are prohibited"]
    #[serde(default)]
    pub excluded: bool,
    #[doc = "Whether at least one choice must be provided"]
    #[serde(default)]
    pub required: bool,
}
impl ::std::convert::From<&FieldPolymorphicDeclaration> for FieldPolymorphicDeclaration {
    fn from(value: &FieldPolymorphicDeclaration) -> Self {
        value.clone()
    }
}
impl FieldPolymorphicDeclaration {
    pub fn builder() -> builder::FieldPolymorphicDeclaration {
        Default::default()
    }
}
#[doc = "A specific type option for a FHIR choice type (e.g., valueString, valueInteger)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Polymorphic value[x] field instance\","]
#[doc = "  \"description\": \"A specific type option for a FHIR choice type (e.g., valueString, valueInteger)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"choiceOf\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"array\": {"]
#[doc = "      \"description\": \"Whether this choice can contain multiple values\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"binding\": {"]
#[doc = "      \"description\": \"For coded choices, information about the value set binding\","]
#[doc = "      \"$ref\": \"#/definitions/field.*.binding\""]
#[doc = "    },"]
#[doc = "    \"choiceOf\": {"]
#[doc = "      \"description\": \"The name of the choice field this instance belongs to (e.g., 'value' for valueString)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"description\": \"For code fields, the set of valid values when bound to a required value set\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"excluded\": {"]
#[doc = "      \"description\": \"Whether this specific choice is prohibited\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"reference\": {"]
#[doc = "      \"description\": \"Reference to other types that this field can point to\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether this specific choice must be provided\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The data type of this choice option\","]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldPolymorphicInstance {
    #[doc = "Whether this choice can contain multiple values"]
    #[serde(default)]
    pub array: bool,
    #[doc = "For coded choices, information about the value set binding"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub binding: ::std::option::Option<FieldBinding>,
    #[doc = "The name of the choice field this instance belongs to (e.g., 'value' for valueString)"]
    #[serde(rename = "choiceOf")]
    pub choice_of: ::std::string::String,
    #[doc = "For code fields, the set of valid values when bound to a required value set"]
    #[serde(
        rename = "enum",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[doc = "Whether this specific choice is prohibited"]
    #[serde(default)]
    pub excluded: bool,
    #[doc = "Reference to other types that this field can point to"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub reference: ::std::vec::Vec<TypeSchemaIdentifier>,
    #[doc = "Whether this specific choice must be provided"]
    #[serde(default)]
    pub required: bool,
    #[doc = "The data type of this choice option"]
    #[serde(rename = "type")]
    pub type_: TypeSchemaIdentifier,
}
impl ::std::convert::From<&FieldPolymorphicInstance> for FieldPolymorphicInstance {
    fn from(value: &FieldPolymorphicInstance) -> Self {
        value.clone()
    }
}
impl FieldPolymorphicInstance {
    pub fn builder() -> builder::FieldPolymorphicInstance {
        Default::default()
    }
}
#[doc = "A standard field with a single type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Regular field\","]
#[doc = "  \"description\": \"A standard field with a single type\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"array\": {"]
#[doc = "      \"description\": \"Whether this field can contain multiple values (cardinality > 1)\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"binding\": {"]
#[doc = "      \"description\": \"For coded fields, information about the value set binding\","]
#[doc = "      \"$ref\": \"#/definitions/field.*.binding\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"description\": \"For code fields, the set of valid values when bound to a required value set\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"excluded\": {"]
#[doc = "      \"description\": \"Whether this field is prohibited in valid instances\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"reference\": {"]
#[doc = "      \"description\": \"Reference to other types that this field can point to\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether this field must be provided in valid instances\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The data type of this field\","]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldRegular {
    #[doc = "Whether this field can contain multiple values (cardinality > 1)"]
    #[serde(default)]
    pub array: bool,
    #[doc = "For coded fields, information about the value set binding"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub binding: ::std::option::Option<FieldBinding>,
    #[doc = "For code fields, the set of valid values when bound to a required value set"]
    #[serde(
        rename = "enum",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[doc = "Whether this field is prohibited in valid instances"]
    #[serde(default)]
    pub excluded: bool,
    #[doc = "Reference to other types that this field can point to"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub reference: ::std::vec::Vec<TypeSchemaIdentifier>,
    #[doc = "Whether this field must be provided in valid instances"]
    #[serde(default)]
    pub required: bool,
    #[doc = "The data type of this field"]
    #[serde(rename = "type")]
    pub type_: TypeSchemaIdentifier,
}
impl ::std::convert::From<&FieldRegular> for FieldRegular {
    fn from(value: &FieldRegular) -> Self {
        value.clone()
    }
}
impl FieldRegular {
    pub fn builder() -> builder::FieldRegular {
        Default::default()
    }
}
#[doc = "A code generation friendly representation of FHIR StructureDefinition and FHIR Schema designed to simplify SDK resource classes/types generation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Type Schema\","]
#[doc = "  \"description\": \"A code generation friendly representation of FHIR StructureDefinition and FHIR Schema designed to simplify SDK resource classes/types generation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Type Schema for Primitive Type\","]
#[doc = "      \"description\": \"Schema for basic FHIR data types like string, boolean, decimal\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"base\","]
#[doc = "        \"identifier\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base\": {"]
#[doc = "          \"description\": \"The base type this primitive type extends (typically Element)\","]
#[doc = "          \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"description\": \"Other types that this primitive type depends on\","]
#[doc = "          \"$ref\": \"#/definitions/dependencies\""]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"description\": \"Human-readable description of the primitive type\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"identifier\": {"]
#[doc = "          \"description\": \"The unique identifier for this primitive type\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/with-primitive-type-kind\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Type Schema for Resource, Complex Type, Logical\","]
#[doc = "      \"description\": \"Schema for FHIR resources, complex types, and logical types\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"identifier\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base\": {"]
#[doc = "          \"description\": \"The base type this resource or type extends\","]
#[doc = "          \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"description\": \"Other types that this resource or type depends on\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"description\": \"Human-readable description of the resource or type\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"fields\": {"]
#[doc = "          \"description\": \"The fields contained in this resource or type\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/field.regular\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/field.polymorphic-declaration\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/field.polymorphic-instance\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"identifier\": {"]
#[doc = "          \"description\": \"The unique identifier for this resource or type\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/definitions/with-resource-kind\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/definitions/with-complex-type-kind\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/definitions/with-logical-kind\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"nested\": {"]
#[doc = "          \"description\": \"BackboneElement types nested within this resource or type\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"base\","]
#[doc = "              \"identifier\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"base\": {"]
#[doc = "                \"description\": \"The base type this nested type extends (typically BackboneElement)\","]
#[doc = "                \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "              },"]
#[doc = "              \"fields\": {"]
#[doc = "                \"description\": \"The fields contained in this nested type\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"additionalProperties\": {"]
#[doc = "                  \"anyOf\": ["]
#[doc = "                    {"]
#[doc = "                      \"$ref\": \"#/definitions/field.regular\""]
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"$ref\": \"#/definitions/field.polymorphic-declaration\""]
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"$ref\": \"#/definitions/field.polymorphic-instance\""]
#[doc = "                    }"]
#[doc = "                  ]"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"identifier\": {"]
#[doc = "                \"description\": \"The unique identifier for this nested type\","]
#[doc = "                \"allOf\": ["]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "                  },"]
#[doc = "                  {"]
#[doc = "                    \"$ref\": \"#/definitions/with-nested-kind\""]
#[doc = "                  }"]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Type Schema for ValueSet\","]
#[doc = "      \"description\": \"Schema for FHIR value sets that define terminology bindings\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"identifier\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"compose\": {"]
#[doc = "          \"description\": \"Complex value set composition rules when the value set is defined as a composition of other value sets\","]
#[doc = "          \"type\": \"object\""]
#[doc = "        },"]
#[doc = "        \"concept\": {"]
#[doc = "          \"description\": \"The list of coded concepts contained in this value set\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"code\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"code\": {"]
#[doc = "                \"description\": \"The code value\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"display\": {"]
#[doc = "                \"description\": \"The human-readable display text for this code\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"system\": {"]
#[doc = "                \"description\": \"The code system URL that defines this code\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"description\": \"Human-readable description of the value set\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"identifier\": {"]
#[doc = "          \"description\": \"The unique identifier for this value set\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/with-valueset-kind\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"$id: http://health-samurai.io/schemas/type-schema.schema.json/0.0.0.1\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum TypeSchema {
    PrimitiveType {
        #[doc = "The base type this primitive type extends (typically Element)"]
        base: TypeSchemaIdentifier,
        #[doc = "Other types that this primitive type depends on"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        dependencies: ::std::option::Option<Dependencies>,
        #[doc = "Human-readable description of the primitive type"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        identifier: TypeSchemaForPrimitiveTypeIdentifier,
    },
    ResourceComplexTypeLogical {
        #[doc = "The base type this resource or type extends"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        base: ::std::option::Option<TypeSchemaIdentifier>,
        #[doc = "Other types that this resource or type depends on"]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        dependencies: ::std::vec::Vec<TypeSchemaIdentifier>,
        #[doc = "Human-readable description of the resource or type"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[doc = "The fields contained in this resource or type"]
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        fields: ::std::collections::HashMap<
            ::std::string::String,
            TypeSchemaForResourceComplexTypeLogicalFieldsValue,
        >,
        identifier: TypeSchemaForResourceComplexTypeLogicalIdentifier,
        #[doc = "BackboneElement types nested within this resource or type"]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        nested: ::std::vec::Vec<TypeSchemaForResourceComplexTypeLogicalNestedItem>,
    },
    ValueSet {
        #[doc = "Complex value set composition rules when the value set is defined as a composition of other value sets"]
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        compose: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[doc = "The list of coded concepts contained in this value set"]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        concept: ::std::vec::Vec<TypeSchemaForValueSetConceptItem>,
        #[doc = "Human-readable description of the value set"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        identifier: TypeSchemaForValueSetIdentifier,
    },
}
impl ::std::convert::From<&Self> for TypeSchema {
    fn from(value: &TypeSchema) -> Self {
        value.clone()
    }
}
#[doc = "The unique identifier for this primitive type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The unique identifier for this primitive type\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/with-primitive-type-kind\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaForPrimitiveTypeIdentifier {
    pub kind: TypeSchemaForPrimitiveTypeIdentifierKind,
    #[doc = "Computer friendly identifier for the element"]
    pub name: ::std::string::String,
    #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
    pub package: ::std::string::String,
    #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
    pub url: TypeSchemaForPrimitiveTypeIdentifierUrl,
    #[doc = "The version of the package"]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&TypeSchemaForPrimitiveTypeIdentifier>
    for TypeSchemaForPrimitiveTypeIdentifier
{
    fn from(value: &TypeSchemaForPrimitiveTypeIdentifier) -> Self {
        value.clone()
    }
}
impl TypeSchemaForPrimitiveTypeIdentifier {
    pub fn builder() -> builder::TypeSchemaForPrimitiveTypeIdentifier {
        Default::default()
    }
}
#[doc = "TypeSchemaForPrimitiveTypeIdentifierKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"primitive-type\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForPrimitiveTypeIdentifierKind {
    #[serde(rename = "primitive-type")]
    PrimitiveType,
}
impl ::std::convert::From<&Self> for TypeSchemaForPrimitiveTypeIdentifierKind {
    fn from(value: &TypeSchemaForPrimitiveTypeIdentifierKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForPrimitiveTypeIdentifierKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PrimitiveType => write!(f, "primitive-type"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForPrimitiveTypeIdentifierKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "primitive-type" => Ok(Self::PrimitiveType),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaForPrimitiveTypeIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaForPrimitiveTypeIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaForPrimitiveTypeIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForPrimitiveTypeIdentifierUrl(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForPrimitiveTypeIdentifierUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForPrimitiveTypeIdentifierUrl> for ::std::string::String {
    fn from(value: TypeSchemaForPrimitiveTypeIdentifierUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForPrimitiveTypeIdentifierUrl>
    for TypeSchemaForPrimitiveTypeIdentifierUrl
{
    fn from(value: &TypeSchemaForPrimitiveTypeIdentifierUrl) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForPrimitiveTypeIdentifierUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaForPrimitiveTypeIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaForPrimitiveTypeIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaForPrimitiveTypeIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeSchemaForPrimitiveTypeIdentifierUrl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalFieldsValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.regular\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.polymorphic-declaration\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.polymorphic-instance\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeSchemaForResourceComplexTypeLogicalFieldsValue {
    Regular(FieldRegular),
    PolymorphicDeclaration(FieldPolymorphicDeclaration),
    PolymorphicInstance(FieldPolymorphicInstance),
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalFieldsValue {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalFieldsValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<FieldRegular> for TypeSchemaForResourceComplexTypeLogicalFieldsValue {
    fn from(value: FieldRegular) -> Self {
        Self::Regular(value)
    }
}
impl ::std::convert::From<FieldPolymorphicDeclaration>
    for TypeSchemaForResourceComplexTypeLogicalFieldsValue
{
    fn from(value: FieldPolymorphicDeclaration) -> Self {
        Self::PolymorphicDeclaration(value)
    }
}
impl ::std::convert::From<FieldPolymorphicInstance>
    for TypeSchemaForResourceComplexTypeLogicalFieldsValue
{
    fn from(value: FieldPolymorphicInstance) -> Self {
        Self::PolymorphicInstance(value)
    }
}
#[doc = "The unique identifier for this resource or type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The unique identifier for this resource or type\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/with-resource-kind\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/with-complex-type-kind\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/with-logical-kind\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum TypeSchemaForResourceComplexTypeLogicalIdentifier {
    Variant0 {
        kind: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind,
        #[doc = "Computer friendly identifier for the element"]
        name: ::std::string::String,
        #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
        package: ::std::string::String,
        #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
        url: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url,
        #[doc = "The version of the package"]
        version: ::std::string::String,
    },
    Variant1 {
        kind: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind,
        #[doc = "Computer friendly identifier for the element"]
        name: ::std::string::String,
        #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
        package: ::std::string::String,
        #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
        url: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url,
        #[doc = "The version of the package"]
        version: ::std::string::String,
    },
    Variant2 {
        kind: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind,
        #[doc = "Computer friendly identifier for the element"]
        name: ::std::string::String,
        #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
        package: ::std::string::String,
        #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
        url: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url,
        #[doc = "The version of the package"]
        version: ::std::string::String,
    },
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalIdentifier {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifier) -> Self {
        value.clone()
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"resource\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind {
    #[serde(rename = "resource")]
    Resource,
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Resource => write!(f, "resource"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "resource" => Ok(Self::Resource),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url>
    for ::std::string::String
{
    fn from(value: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant0Url
{
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"complex-type\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind {
    #[serde(rename = "complex-type")]
    ComplexType,
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComplexType => write!(f, "complex-type"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "complex-type" => Ok(Self::ComplexType),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url>
    for ::std::string::String
{
    fn from(value: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant1Url
{
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"logical\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind {
    #[serde(rename = "logical")]
    Logical,
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logical => write!(f, "logical"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logical" => Ok(Self::Logical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Kind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url>
    for ::std::string::String
{
    fn from(value: TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for TypeSchemaForResourceComplexTypeLogicalIdentifierVariant2Url
{
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalNestedItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base\","]
#[doc = "    \"identifier\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base\": {"]
#[doc = "      \"description\": \"The base type this nested type extends (typically BackboneElement)\","]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    \"fields\": {"]
#[doc = "      \"description\": \"The fields contained in this nested type\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/field.regular\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/field.polymorphic-declaration\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/field.polymorphic-instance\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"identifier\": {"]
#[doc = "      \"description\": \"The unique identifier for this nested type\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/with-nested-kind\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaForResourceComplexTypeLogicalNestedItem {
    #[doc = "The base type this nested type extends (typically BackboneElement)"]
    pub base: TypeSchemaIdentifier,
    #[doc = "The fields contained in this nested type"]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub fields: ::std::collections::HashMap<
        ::std::string::String,
        TypeSchemaForResourceComplexTypeLogicalFieldsValue,
    >,
    pub identifier: TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier,
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalNestedItem>
    for TypeSchemaForResourceComplexTypeLogicalNestedItem
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalNestedItem) -> Self {
        value.clone()
    }
}
impl TypeSchemaForResourceComplexTypeLogicalNestedItem {
    pub fn builder() -> builder::TypeSchemaForResourceComplexTypeLogicalNestedItem {
        Default::default()
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.regular\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.polymorphic-declaration\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field.polymorphic-instance\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue {
    Regular(FieldRegular),
    PolymorphicDeclaration(FieldPolymorphicDeclaration),
    PolymorphicInstance(FieldPolymorphicInstance),
}
impl ::std::convert::From<&Self> for TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue {
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<FieldRegular>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue
{
    fn from(value: FieldRegular) -> Self {
        Self::Regular(value)
    }
}
impl ::std::convert::From<FieldPolymorphicDeclaration>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue
{
    fn from(value: FieldPolymorphicDeclaration) -> Self {
        Self::PolymorphicDeclaration(value)
    }
}
impl ::std::convert::From<FieldPolymorphicInstance>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemFieldsValue
{
    fn from(value: FieldPolymorphicInstance) -> Self {
        Self::PolymorphicInstance(value)
    }
}
#[doc = "The unique identifier for this nested type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The unique identifier for this nested type\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/with-nested-kind\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
    pub kind: TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind,
    #[doc = "Computer friendly identifier for the element"]
    pub name: ::std::string::String,
    #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
    pub package: ::std::string::String,
    #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
    pub url: TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl,
    #[doc = "The version of the package"]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier) -> Self {
        value.clone()
    }
}
impl TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
    pub fn builder() -> builder::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
        Default::default()
    }
}
#[doc = "TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"nested\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind {
    #[serde(rename = "nested")]
    Nested,
}
impl ::std::convert::From<&Self>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Nested => write!(f, "nested"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "nested" => Ok(Self::Nested),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl>
    for ::std::string::String
{
    fn from(value: TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl
{
    fn from(value: &TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl
{
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaForValueSetConceptItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The code value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"display\": {"]
#[doc = "      \"description\": \"The human-readable display text for this code\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"system\": {"]
#[doc = "      \"description\": \"The code system URL that defines this code\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaForValueSetConceptItem {
    #[doc = "The code value"]
    pub code: ::std::string::String,
    #[doc = "The human-readable display text for this code"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display: ::std::option::Option<::std::string::String>,
    #[doc = "The code system URL that defines this code"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TypeSchemaForValueSetConceptItem> for TypeSchemaForValueSetConceptItem {
    fn from(value: &TypeSchemaForValueSetConceptItem) -> Self {
        value.clone()
    }
}
impl TypeSchemaForValueSetConceptItem {
    pub fn builder() -> builder::TypeSchemaForValueSetConceptItem {
        Default::default()
    }
}
#[doc = "The unique identifier for this value set"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The unique identifier for this value set\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/type-schema-identifier\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/with-valueset-kind\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaForValueSetIdentifier {
    pub kind: TypeSchemaForValueSetIdentifierKind,
    #[doc = "Computer friendly identifier for the element"]
    pub name: ::std::string::String,
    #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
    pub package: ::std::string::String,
    #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
    pub url: TypeSchemaForValueSetIdentifierUrl,
    #[doc = "The version of the package"]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&TypeSchemaForValueSetIdentifier> for TypeSchemaForValueSetIdentifier {
    fn from(value: &TypeSchemaForValueSetIdentifier) -> Self {
        value.clone()
    }
}
impl TypeSchemaForValueSetIdentifier {
    pub fn builder() -> builder::TypeSchemaForValueSetIdentifier {
        Default::default()
    }
}
#[doc = "TypeSchemaForValueSetIdentifierKind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"value-set\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaForValueSetIdentifierKind {
    #[serde(rename = "value-set")]
    ValueSet,
}
impl ::std::convert::From<&Self> for TypeSchemaForValueSetIdentifierKind {
    fn from(value: &TypeSchemaForValueSetIdentifierKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaForValueSetIdentifierKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ValueSet => write!(f, "value-set"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaForValueSetIdentifierKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "value-set" => Ok(Self::ValueSet),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaForValueSetIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaForValueSetIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaForValueSetIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaForValueSetIdentifierUrl(::std::string::String);
impl ::std::ops::Deref for TypeSchemaForValueSetIdentifierUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaForValueSetIdentifierUrl> for ::std::string::String {
    fn from(value: TypeSchemaForValueSetIdentifierUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaForValueSetIdentifierUrl>
    for TypeSchemaForValueSetIdentifierUrl
{
    fn from(value: &TypeSchemaForValueSetIdentifierUrl) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaForValueSetIdentifierUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaForValueSetIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaForValueSetIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaForValueSetIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeSchemaForValueSetIdentifierUrl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "TypeSchemaIdentifier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"package\","]
#[doc = "    \"url\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The category of FHIR element this identifier represents\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"Basic FHIR data types like string, boolean, decimal\","]
#[doc = "          \"const\": \"primitive-type\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"A set of coded values that can be used for a field\","]
#[doc = "          \"const\": \"value-set\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"Composite data types like Address, HumanName, CodeableConcept\","]
#[doc = "          \"const\": \"complex-type\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"Top-level FHIR resources like Patient, Observation, Encounter\","]
#[doc = "          \"const\": \"resource\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"A complex type embedded within a resource or another type\","]
#[doc = "          \"const\": \"nested\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"Abstract types used for inheritance and specialization\","]
#[doc = "          \"const\": \"logical\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Computer friendly identifier for the element\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"package\": {"]
#[doc = "      \"description\": \"The FHIR package identifier (e.g., hl7.fhir.r4.core)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"The version of the package\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeSchemaIdentifier {
    #[doc = "The category of FHIR element this identifier represents"]
    pub kind: TypeSchemaIdentifierKind,
    #[doc = "Computer friendly identifier for the element"]
    pub name: ::std::string::String,
    #[doc = "The FHIR package identifier (e.g., hl7.fhir.r4.core)"]
    pub package: ::std::string::String,
    #[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
    pub url: TypeSchemaIdentifierUrl,
    #[doc = "The version of the package"]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&TypeSchemaIdentifier> for TypeSchemaIdentifier {
    fn from(value: &TypeSchemaIdentifier) -> Self {
        value.clone()
    }
}
impl TypeSchemaIdentifier {
    pub fn builder() -> builder::TypeSchemaIdentifier {
        Default::default()
    }
}
#[doc = "The category of FHIR element this identifier represents"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The category of FHIR element this identifier represents\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Basic FHIR data types like string, boolean, decimal\","]
#[doc = "      \"const\": \"primitive-type\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A set of coded values that can be used for a field\","]
#[doc = "      \"const\": \"value-set\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Composite data types like Address, HumanName, CodeableConcept\","]
#[doc = "      \"const\": \"complex-type\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Top-level FHIR resources like Patient, Observation, Encounter\","]
#[doc = "      \"const\": \"resource\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A complex type embedded within a resource or another type\","]
#[doc = "      \"const\": \"nested\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Abstract types used for inheritance and specialization\","]
#[doc = "      \"const\": \"logical\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeSchemaIdentifierKind {
    #[doc = "Basic FHIR data types like string, boolean, decimal"]
    #[serde(rename = "primitive-type")]
    PrimitiveType,
    #[doc = "A set of coded values that can be used for a field"]
    #[serde(rename = "value-set")]
    ValueSet,
    #[doc = "Composite data types like Address, HumanName, CodeableConcept"]
    #[serde(rename = "complex-type")]
    ComplexType,
    #[doc = "Top-level FHIR resources like Patient, Observation, Encounter"]
    #[serde(rename = "resource")]
    Resource,
    #[doc = "A complex type embedded within a resource or another type"]
    #[serde(rename = "nested")]
    Nested,
    #[doc = "Abstract types used for inheritance and specialization"]
    #[serde(rename = "logical")]
    Logical,
}
impl ::std::convert::From<&Self> for TypeSchemaIdentifierKind {
    fn from(value: &TypeSchemaIdentifierKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeSchemaIdentifierKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PrimitiveType => write!(f, "primitive-type"),
            Self::ValueSet => write!(f, "value-set"),
            Self::ComplexType => write!(f, "complex-type"),
            Self::Resource => write!(f, "resource"),
            Self::Nested => write!(f, "nested"),
            Self::Logical => write!(f, "logical"),
        }
    }
}
impl ::std::str::FromStr for TypeSchemaIdentifierKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "primitive-type" => Ok(Self::PrimitiveType),
            "value-set" => Ok(Self::ValueSet),
            "complex-type" => Ok(Self::ComplexType),
            "resource" => Ok(Self::Resource),
            "nested" => Ok(Self::Nested),
            "logical" => Ok(Self::Logical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaIdentifierKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical URL identifying this resource uniquely in the FHIR ecosystem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A canonical URL identifying this resource uniquely in the FHIR ecosystem\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(https?://|urn:)[^\\\\s]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeSchemaIdentifierUrl(::std::string::String);
impl ::std::ops::Deref for TypeSchemaIdentifierUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeSchemaIdentifierUrl> for ::std::string::String {
    fn from(value: TypeSchemaIdentifierUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeSchemaIdentifierUrl> for TypeSchemaIdentifierUrl {
    fn from(value: &TypeSchemaIdentifierUrl) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TypeSchemaIdentifierUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(https?://|urn:)[^\\s]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(https?://|urn:)[^\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeSchemaIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeSchemaIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeSchemaIdentifierUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeSchemaIdentifierUrl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Constraint helper to ensure the kind is complex-type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is complex-type\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"complex-type\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithComplexTypeKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithComplexTypeKind> for WithComplexTypeKind {
    fn from(value: &WithComplexTypeKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithComplexTypeKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithComplexTypeKind {
    pub fn builder() -> builder::WithComplexTypeKind {
        Default::default()
    }
}
#[doc = "Constraint helper to ensure the kind is logical"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is logical\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"logical\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithLogicalKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithLogicalKind> for WithLogicalKind {
    fn from(value: &WithLogicalKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithLogicalKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithLogicalKind {
    pub fn builder() -> builder::WithLogicalKind {
        Default::default()
    }
}
#[doc = "Constraint helper to ensure the kind is nested"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is nested\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"nested\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithNestedKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithNestedKind> for WithNestedKind {
    fn from(value: &WithNestedKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithNestedKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithNestedKind {
    pub fn builder() -> builder::WithNestedKind {
        Default::default()
    }
}
#[doc = "Constraint helper to ensure the kind is primitive-type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is primitive-type\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"primitive-type\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithPrimitiveTypeKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithPrimitiveTypeKind> for WithPrimitiveTypeKind {
    fn from(value: &WithPrimitiveTypeKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithPrimitiveTypeKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithPrimitiveTypeKind {
    pub fn builder() -> builder::WithPrimitiveTypeKind {
        Default::default()
    }
}
#[doc = "Constraint helper to ensure the kind is resource"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is resource\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"resource\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithResourceKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithResourceKind> for WithResourceKind {
    fn from(value: &WithResourceKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithResourceKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithResourceKind {
    pub fn builder() -> builder::WithResourceKind {
        Default::default()
    }
}
#[doc = "Constraint helper to ensure the kind is value-set"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Constraint helper to ensure the kind is value-set\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"const\": \"value-set\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WithValuesetKind {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&WithValuesetKind> for WithValuesetKind {
    fn from(value: &WithValuesetKind) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for WithValuesetKind {
    fn default() -> Self {
        Self {
            kind: Default::default(),
        }
    }
}
impl WithValuesetKind {
    pub fn builder() -> builder::WithValuesetKind {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct FieldBinding {
        strength: ::std::result::Result<super::FieldBindingStrength, ::std::string::String>,
        valueset: ::std::result::Result<super::FieldBindingValueset, ::std::string::String>,
    }
    impl ::std::default::Default for FieldBinding {
        fn default() -> Self {
            Self {
                strength: Err("no value supplied for strength".to_string()),
                valueset: Err("no value supplied for valueset".to_string()),
            }
        }
    }
    impl FieldBinding {
        pub fn strength<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FieldBindingStrength>,
            T::Error: ::std::fmt::Display,
        {
            self.strength = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strength: {}", e));
            self
        }
        pub fn valueset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FieldBindingValueset>,
            T::Error: ::std::fmt::Display,
        {
            self.valueset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valueset: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FieldBinding> for super::FieldBinding {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FieldBinding,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                strength: value.strength?,
                valueset: value.valueset?,
            })
        }
    }
    impl ::std::convert::From<super::FieldBinding> for FieldBinding {
        fn from(value: super::FieldBinding) -> Self {
            Self {
                strength: Ok(value.strength),
                valueset: Ok(value.valueset),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldBindingValueset {
        kind: ::std::result::Result<super::FieldBindingValuesetKind, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        package: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<super::FieldBindingValuesetUrl, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FieldBindingValueset {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                package: Err("no value supplied for package".to_string()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl FieldBindingValueset {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FieldBindingValuesetKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn package<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FieldBindingValuesetUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FieldBindingValueset> for super::FieldBindingValueset {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FieldBindingValueset,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                name: value.name?,
                package: value.package?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::FieldBindingValueset> for FieldBindingValueset {
        fn from(value: super::FieldBindingValueset) -> Self {
            Self {
                kind: Ok(value.kind),
                name: Ok(value.name),
                package: Ok(value.package),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldPolymorphicDeclaration {
        array: ::std::result::Result<bool, ::std::string::String>,
        choices:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        excluded: ::std::result::Result<bool, ::std::string::String>,
        required: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for FieldPolymorphicDeclaration {
        fn default() -> Self {
            Self {
                array: Ok(Default::default()),
                choices: Err("no value supplied for choices".to_string()),
                excluded: Ok(Default::default()),
                required: Ok(Default::default()),
            }
        }
    }
    impl FieldPolymorphicDeclaration {
        pub fn array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.array = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for array: {}", e));
            self
        }
        pub fn choices<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.choices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choices: {}", e));
            self
        }
        pub fn excluded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.excluded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for excluded: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FieldPolymorphicDeclaration> for super::FieldPolymorphicDeclaration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FieldPolymorphicDeclaration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                array: value.array?,
                choices: value.choices?,
                excluded: value.excluded?,
                required: value.required?,
            })
        }
    }
    impl ::std::convert::From<super::FieldPolymorphicDeclaration> for FieldPolymorphicDeclaration {
        fn from(value: super::FieldPolymorphicDeclaration) -> Self {
            Self {
                array: Ok(value.array),
                choices: Ok(value.choices),
                excluded: Ok(value.excluded),
                required: Ok(value.required),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldPolymorphicInstance {
        array: ::std::result::Result<bool, ::std::string::String>,
        binding: ::std::result::Result<
            ::std::option::Option<super::FieldBinding>,
            ::std::string::String,
        >,
        choice_of: ::std::result::Result<::std::string::String, ::std::string::String>,
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        excluded: ::std::result::Result<bool, ::std::string::String>,
        reference: ::std::result::Result<
            ::std::vec::Vec<super::TypeSchemaIdentifier>,
            ::std::string::String,
        >,
        required: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::TypeSchemaIdentifier, ::std::string::String>,
    }
    impl ::std::default::Default for FieldPolymorphicInstance {
        fn default() -> Self {
            Self {
                array: Ok(Default::default()),
                binding: Ok(Default::default()),
                choice_of: Err("no value supplied for choice_of".to_string()),
                enum_: Ok(Default::default()),
                excluded: Ok(Default::default()),
                reference: Ok(Default::default()),
                required: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FieldPolymorphicInstance {
        pub fn array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.array = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for array: {}", e));
            self
        }
        pub fn binding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FieldBinding>>,
            T::Error: ::std::fmt::Display,
        {
            self.binding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for binding: {}", e));
            self
        }
        pub fn choice_of<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.choice_of = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choice_of: {}", e));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn excluded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.excluded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for excluded: {}", e));
            self
        }
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TypeSchemaIdentifier>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaIdentifier>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FieldPolymorphicInstance> for super::FieldPolymorphicInstance {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FieldPolymorphicInstance,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                array: value.array?,
                binding: value.binding?,
                choice_of: value.choice_of?,
                enum_: value.enum_?,
                excluded: value.excluded?,
                reference: value.reference?,
                required: value.required?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FieldPolymorphicInstance> for FieldPolymorphicInstance {
        fn from(value: super::FieldPolymorphicInstance) -> Self {
            Self {
                array: Ok(value.array),
                binding: Ok(value.binding),
                choice_of: Ok(value.choice_of),
                enum_: Ok(value.enum_),
                excluded: Ok(value.excluded),
                reference: Ok(value.reference),
                required: Ok(value.required),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldRegular {
        array: ::std::result::Result<bool, ::std::string::String>,
        binding: ::std::result::Result<
            ::std::option::Option<super::FieldBinding>,
            ::std::string::String,
        >,
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        excluded: ::std::result::Result<bool, ::std::string::String>,
        reference: ::std::result::Result<
            ::std::vec::Vec<super::TypeSchemaIdentifier>,
            ::std::string::String,
        >,
        required: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::TypeSchemaIdentifier, ::std::string::String>,
    }
    impl ::std::default::Default for FieldRegular {
        fn default() -> Self {
            Self {
                array: Ok(Default::default()),
                binding: Ok(Default::default()),
                enum_: Ok(Default::default()),
                excluded: Ok(Default::default()),
                reference: Ok(Default::default()),
                required: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FieldRegular {
        pub fn array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.array = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for array: {}", e));
            self
        }
        pub fn binding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FieldBinding>>,
            T::Error: ::std::fmt::Display,
        {
            self.binding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for binding: {}", e));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn excluded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.excluded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for excluded: {}", e));
            self
        }
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TypeSchemaIdentifier>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaIdentifier>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FieldRegular> for super::FieldRegular {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FieldRegular,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                array: value.array?,
                binding: value.binding?,
                enum_: value.enum_?,
                excluded: value.excluded?,
                reference: value.reference?,
                required: value.required?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FieldRegular> for FieldRegular {
        fn from(value: super::FieldRegular) -> Self {
            Self {
                array: Ok(value.array),
                binding: Ok(value.binding),
                enum_: Ok(value.enum_),
                excluded: Ok(value.excluded),
                reference: Ok(value.reference),
                required: Ok(value.required),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaForPrimitiveTypeIdentifier {
        kind: ::std::result::Result<
            super::TypeSchemaForPrimitiveTypeIdentifierKind,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        package: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<
            super::TypeSchemaForPrimitiveTypeIdentifierUrl,
            ::std::string::String,
        >,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TypeSchemaForPrimitiveTypeIdentifier {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                package: Err("no value supplied for package".to_string()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl TypeSchemaForPrimitiveTypeIdentifier {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaForPrimitiveTypeIdentifierKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn package<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaForPrimitiveTypeIdentifierUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaForPrimitiveTypeIdentifier>
        for super::TypeSchemaForPrimitiveTypeIdentifier
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaForPrimitiveTypeIdentifier,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                name: value.name?,
                package: value.package?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaForPrimitiveTypeIdentifier>
        for TypeSchemaForPrimitiveTypeIdentifier
    {
        fn from(value: super::TypeSchemaForPrimitiveTypeIdentifier) -> Self {
            Self {
                kind: Ok(value.kind),
                name: Ok(value.name),
                package: Ok(value.package),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaForResourceComplexTypeLogicalNestedItem {
        base: ::std::result::Result<super::TypeSchemaIdentifier, ::std::string::String>,
        fields: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                super::TypeSchemaForResourceComplexTypeLogicalFieldsValue,
            >,
            ::std::string::String,
        >,
        identifier: ::std::result::Result<
            super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TypeSchemaForResourceComplexTypeLogicalNestedItem {
        fn default() -> Self {
            Self {
                base: Err("no value supplied for base".to_string()),
                fields: Ok(Default::default()),
                identifier: Err("no value supplied for identifier".to_string()),
            }
        }
    }
    impl TypeSchemaForResourceComplexTypeLogicalNestedItem {
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaIdentifier>,
            T::Error: ::std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::TypeSchemaForResourceComplexTypeLogicalFieldsValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
        pub fn identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for identifier: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaForResourceComplexTypeLogicalNestedItem>
        for super::TypeSchemaForResourceComplexTypeLogicalNestedItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaForResourceComplexTypeLogicalNestedItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base: value.base?,
                fields: value.fields?,
                identifier: value.identifier?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaForResourceComplexTypeLogicalNestedItem>
        for TypeSchemaForResourceComplexTypeLogicalNestedItem
    {
        fn from(value: super::TypeSchemaForResourceComplexTypeLogicalNestedItem) -> Self {
            Self {
                base: Ok(value.base),
                fields: Ok(value.fields),
                identifier: Ok(value.identifier),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
        kind: ::std::result::Result<
            super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        package: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<
            super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl,
            ::std::string::String,
        >,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                package: Err("no value supplied for package".to_string()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierKind,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn package<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifierUrl,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier>
        for super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                name: value.name?,
                package: value.package?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier>
        for TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier
    {
        fn from(value: super::TypeSchemaForResourceComplexTypeLogicalNestedItemIdentifier) -> Self {
            Self {
                kind: Ok(value.kind),
                name: Ok(value.name),
                package: Ok(value.package),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaForValueSetConceptItem {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        display: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TypeSchemaForValueSetConceptItem {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                display: Ok(Default::default()),
                system: Ok(Default::default()),
            }
        }
    }
    impl TypeSchemaForValueSetConceptItem {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.display = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display: {}", e));
            self
        }
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaForValueSetConceptItem>
        for super::TypeSchemaForValueSetConceptItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaForValueSetConceptItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                display: value.display?,
                system: value.system?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaForValueSetConceptItem>
        for TypeSchemaForValueSetConceptItem
    {
        fn from(value: super::TypeSchemaForValueSetConceptItem) -> Self {
            Self {
                code: Ok(value.code),
                display: Ok(value.display),
                system: Ok(value.system),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaForValueSetIdentifier {
        kind: ::std::result::Result<
            super::TypeSchemaForValueSetIdentifierKind,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        package: ::std::result::Result<::std::string::String, ::std::string::String>,
        url:
            ::std::result::Result<super::TypeSchemaForValueSetIdentifierUrl, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TypeSchemaForValueSetIdentifier {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                package: Err("no value supplied for package".to_string()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl TypeSchemaForValueSetIdentifier {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaForValueSetIdentifierKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn package<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaForValueSetIdentifierUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaForValueSetIdentifier>
        for super::TypeSchemaForValueSetIdentifier
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaForValueSetIdentifier,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                name: value.name?,
                package: value.package?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaForValueSetIdentifier>
        for TypeSchemaForValueSetIdentifier
    {
        fn from(value: super::TypeSchemaForValueSetIdentifier) -> Self {
            Self {
                kind: Ok(value.kind),
                name: Ok(value.name),
                package: Ok(value.package),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeSchemaIdentifier {
        kind: ::std::result::Result<super::TypeSchemaIdentifierKind, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        package: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<super::TypeSchemaIdentifierUrl, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TypeSchemaIdentifier {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                package: Err("no value supplied for package".to_string()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl TypeSchemaIdentifier {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaIdentifierKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn package<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.package = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for package: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeSchemaIdentifierUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeSchemaIdentifier> for super::TypeSchemaIdentifier {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeSchemaIdentifier,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                name: value.name?,
                package: value.package?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::TypeSchemaIdentifier> for TypeSchemaIdentifier {
        fn from(value: super::TypeSchemaIdentifier) -> Self {
            Self {
                kind: Ok(value.kind),
                name: Ok(value.name),
                package: Ok(value.package),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithComplexTypeKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithComplexTypeKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithComplexTypeKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithComplexTypeKind> for super::WithComplexTypeKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithComplexTypeKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithComplexTypeKind> for WithComplexTypeKind {
        fn from(value: super::WithComplexTypeKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithLogicalKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithLogicalKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithLogicalKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithLogicalKind> for super::WithLogicalKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithLogicalKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithLogicalKind> for WithLogicalKind {
        fn from(value: super::WithLogicalKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithNestedKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithNestedKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithNestedKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithNestedKind> for super::WithNestedKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithNestedKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithNestedKind> for WithNestedKind {
        fn from(value: super::WithNestedKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithPrimitiveTypeKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithPrimitiveTypeKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithPrimitiveTypeKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithPrimitiveTypeKind> for super::WithPrimitiveTypeKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithPrimitiveTypeKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithPrimitiveTypeKind> for WithPrimitiveTypeKind {
        fn from(value: super::WithPrimitiveTypeKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithResourceKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithResourceKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithResourceKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithResourceKind> for super::WithResourceKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithResourceKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithResourceKind> for WithResourceKind {
        fn from(value: super::WithResourceKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WithValuesetKind {
        kind: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WithValuesetKind {
        fn default() -> Self {
            Self {
                kind: Ok(Default::default()),
            }
        }
    }
    impl WithValuesetKind {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WithValuesetKind> for super::WithValuesetKind {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WithValuesetKind,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { kind: value.kind? })
        }
    }
    impl ::std::convert::From<super::WithValuesetKind> for WithValuesetKind {
        fn from(value: super::WithValuesetKind) -> Self {
            Self {
                kind: Ok(value.kind),
            }
        }
    }
}
