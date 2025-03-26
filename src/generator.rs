use crate::schema::{
    Dependencies, TypeSchema, TypeSchemaForPrimitiveTypeIdentifier,
    TypeSchemaForResourceComplexTypeLogicalFieldsValue,
    TypeSchemaForResourceComplexTypeLogicalIdentifier,
    TypeSchemaForResourceComplexTypeLogicalNestedItem, TypeSchemaForValueSetConceptItem,
    TypeSchemaForValueSetIdentifier, TypeSchemaIdentifier, TypeSchemaIdentifierKind,
};
use convert_case::{Case, Casing};
use phf::phf_map;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::fs::{remove_dir_all, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::path::PathBuf;

static TYPE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "instant" => "string",
    "time" => "string",
     "date" => "string",
    "dateTime" => "string",

    "decimal" =>"number",
    "integer" => "number",
    "unsignedInt" => "number",
    "positiveInt"=> "number",
    "integer64"=> "number",
    "base64Binary"=> "string",

    "uri" =>"string",
    "url"=> "string",
    "canonical"=> "string",
    "oid" => "string",
    "uuid" =>"string",

    "code"=> "string",
    "markdown"=> "string",
    "id"=>"string",
    "xhtml"=> "string",
};

fn build_value_set_name(input: String) -> String {
    input.to_case(Case::Pascal)
}

fn process_primitive(
    base_path: &PathBuf,
    _base: TypeSchemaIdentifier,
    _dependencies: Option<Dependencies>,
    description: Option<String>,
    identifier: TypeSchemaForPrimitiveTypeIdentifier,
) {
    let type_name = identifier.name;

    if type_name == "string" || type_name == "boolean" || type_name == "number" {
        return;
    }

    let mut file_path = base_path.clone();
    file_path.push(format!("{}.ts", type_name));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&file_path)
        .expect("unable to open file");

    if let Err(e) = writeln!(
        file,
        "{}",
        format!(
            "// {};\nexport type {} = {};",
            description.unwrap_or_default(),
            type_name,
            TYPE_MAP.get(type_name.as_str()).unwrap_or(&"any")
        )
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn process_value_set(
    base_path: &PathBuf,
    identifier: TypeSchemaForValueSetIdentifier,
    description: Option<String>,
    concept: Vec<TypeSchemaForValueSetConceptItem>,
    compose: Map<String, Value>,
) {
    let mut base_dir = base_path.clone();
    base_dir.push("value-sets");
    create_dir_all(&base_dir).expect("unable to create dir");

    let name = build_value_set_name(identifier.name);

    if !concept.is_empty() {
        let values = concept
            .iter()
            .map(|it| format!("'{}'", it.code))
            .collect::<Vec<String>>()
            .join(" | ");
        let mut file_path = base_dir.clone();
        file_path.push(format!("{}.ts", name));
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_path)
            .expect("unable to open file");

        if let Err(e) = writeln!(
            file,
            "{}",
            format!(
                "// {};\nexport type {} = {};",
                description.unwrap_or_default(),
                name,
                values
            )
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn prepare_deps(deps: Vec<TypeSchemaIdentifier>) -> HashMap<String, Vec<String>> {
    let mut deps_map: HashMap<String, Vec<String>> = HashMap::new();
    for item in deps {
        let type_name = item.name;
        if type_name == "string" || type_name == "boolean" || type_name == "number" {
            continue;
        }
        if item.kind == TypeSchemaIdentifierKind::Nested {
            continue;
        }
        if item.kind == TypeSchemaIdentifierKind::ValueSet {
            let value_set_name = build_value_set_name(type_name.clone());
            let existed = deps_map.contains_key(&value_set_name);
            if existed {
                let current = deps_map.get_mut(&value_set_name).unwrap();
                current.push(value_set_name);
            } else {
                deps_map.insert(
                    value_set_name.clone(),
                    vec![format!("value-sets/{}", value_set_name)],
                );
            }
        } else {
            let existed = deps_map.contains_key(&type_name);
            if existed {
                let current = deps_map.get_mut(&type_name).unwrap();
                current.push(type_name);
            } else {
                deps_map.insert(type_name.clone(), vec![type_name]);
            }
        }
    }
    deps_map
}

fn build_fields(
    file: &mut File,
    fields: HashMap<String, TypeSchemaForResourceComplexTypeLogicalFieldsValue>,
    nested: Vec<TypeSchemaForResourceComplexTypeLogicalNestedItem>,
    parent: String,
    already_created: &mut HashMap<String, String>,
) -> Vec<String> {
    let mut result = vec![];

    for (field, field_value) in fields {
        match field_value {
            TypeSchemaForResourceComplexTypeLogicalFieldsValue::Regular(val) => {
                let mut field_type = "any".to_string();

                if val.type_.kind == TypeSchemaIdentifierKind::Nested {
                    let nested_filed = nested
                        .iter()
                        .find(|it| it.identifier.name == val.type_.name);
                    if let Some(nested_field) = nested_filed {
                        let nested_name = format!(
                            "{}{}",
                            parent,
                            nested_field.identifier.name.to_case(Case::Pascal)
                        );

                        if already_created
                            .contains_key::<String>(&nested_field.clone().identifier.url.into())
                        {
                            field_type = already_created
                                .get::<String>(&nested_field.clone().identifier.url.into())
                                .unwrap()
                                .into();
                        } else {
                            already_created.insert(
                                nested_field.clone().identifier.url.into(),
                                nested_name.clone(),
                            );

                            let sub_fields = build_fields(
                                file,
                                nested_field.clone().fields,
                                nested.clone(),
                                nested_name.clone(),
                                already_created,
                            );
                            file.write(format!("export interface {} {{\n", nested_name).as_bytes())
                                .expect("unable to write file");
                            file.write(sub_fields.join("\n").as_bytes())
                                .expect("unable to write file");
                            file.write("\n}\n\n".as_bytes())
                                .expect("unable to write file");

                            field_type = nested_name;
                        }
                    }
                } else if parent == "Reference" && field == "reference" {
                    field_type = "`${T}/${string}`".to_string();
                } else if !val.reference.is_empty() {
                    let refs = val
                        .reference
                        .iter()
                        .map(|x| format!("'{}'", x.name))
                        .collect::<Vec<String>>()
                        .join(" | ");
                    field_type = format!("Reference<{}>", refs);
                } else if !val.enum_.is_empty() && val.type_.name == "code" {
                    field_type = val
                        .enum_
                        .iter()
                        .map(|it| format!("'{}'", it))
                        .collect::<Vec<String>>()
                        .join(" | ");
                } else {
                    field_type = val.type_.name;
                }

                let required_symbol = match val.required {
                    true => "",
                    false => "?",
                };
                let array_symbol = match val.array {
                    true => "[]",
                    false => "",
                };

                result.push(format!(
                    "{}{}: {}{};",
                    field, required_symbol, field_type, array_symbol
                ))
            }
            TypeSchemaForResourceComplexTypeLogicalFieldsValue::PolymorphicInstance(val) => {
                let field_type = val.type_.name;
                let required_symbol = match val.required {
                    true => "",
                    false => "?",
                };
                let array_symbol = match val.array {
                    true => "[]",
                    false => "",
                };

                result.push(format!(
                    "{}{}:{}{};",
                    field, required_symbol, field_type, array_symbol
                ));
            }
            _ => {}
        }
    }

    result
}

fn process_complex_type(
    base_path: &PathBuf,
    base: Option<TypeSchemaIdentifier>,
    dependencies: Vec<TypeSchemaIdentifier>,
    description: Option<String>,
    fields: HashMap<String, TypeSchemaForResourceComplexTypeLogicalFieldsValue>,
    identifier: TypeSchemaForResourceComplexTypeLogicalIdentifier,
    nested: Vec<TypeSchemaForResourceComplexTypeLogicalNestedItem>,
) {
    let name = match identifier.clone() {
        TypeSchemaForResourceComplexTypeLogicalIdentifier::Variant0 {
            kind,
            name,
            package,
            url,
            version,
        } => name,
        TypeSchemaForResourceComplexTypeLogicalIdentifier::Variant1 {
            kind,
            name,
            package,
            url,
            version,
        } => name,
        TypeSchemaForResourceComplexTypeLogicalIdentifier::Variant2 {
            kind,
            name,
            package,
            url,
            version,
        } => name,
    };

    let mut file_path = base_path.clone();
    file_path.push(format!("{}.ts", name));

    let mut file = File::create(file_path).expect("unable to create file");

    let deps = prepare_deps(dependencies);

    if !deps.is_empty() {
        for (key, value) in deps.into_iter() {
            file.write(
                format!("import type {{ {} }} from './{}';\n", value.join(", "), key).as_bytes(),
            )
            .expect("unable to write to file");
        }
        file.write("\n".as_bytes())
            .expect("unable to write to file");
    }

    if description.is_some() {
        file.write(format!("// {}\n", description.unwrap()).as_bytes())
            .expect("unable to write to file");
    }

    let base_interface = match base {
        Some(val) => format!("extends {}", val.name),
        None => "".to_string(),
    };

    let type_fields = build_fields(&mut file, fields, nested, name.clone(), &mut HashMap::new());

    let name = match name.as_str() {
        "Reference" => "Reference<T extends string = string>".to_string(),
        _ => name,
    };

    let interface = vec![
        "export interface".to_string(),
        name.clone(),
        base_interface,
        "{\n".to_string(),
    ];
    file.write(interface.join(" ").as_bytes())
        .expect("unable to write to file");
    let is_resource = match identifier {
        TypeSchemaForResourceComplexTypeLogicalIdentifier::Variant0 { .. } => true,
        _ => false,
    };
    if is_resource {
        file.write(format!("resourceType: '{}';\n", name).as_bytes())
            .expect("unable to write to file");
    }

    file.write(type_fields.join("\n").as_bytes())
        .expect("unable to write to file");
    file.write("\n}\n".as_bytes())
        .expect("unable to write to file");
}

pub fn generate() -> Result<(), Error> {
    let path = PathBuf::from("../type-schema/output/hl7.fhir.r4.core@4.0.1.ndjson");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output_base_path = PathBuf::from("./output");

    remove_dir_all(&output_base_path)?;
    create_dir_all(&output_base_path)?;

    let mut types_path = output_base_path.clone();
    types_path.push("types");

    create_dir_all(&types_path)?;

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("constraint") {
                continue;
            }
            let type_schema =
                serde_json::from_str::<TypeSchema>(line.as_str()).expect("JSON parsing error");

            match type_schema {
                TypeSchema::PrimitiveType {
                    base,
                    dependencies,
                    description,
                    identifier,
                } => {
                    process_primitive(&types_path, base, dependencies, description, identifier);
                }
                TypeSchema::ValueSet {
                    compose,
                    concept,
                    description,
                    identifier,
                } => {
                    process_value_set(&types_path, identifier, description, concept, compose);
                }
                TypeSchema::ResourceComplexTypeLogical {
                    base,
                    dependencies,
                    description,
                    fields,
                    identifier,
                    nested,
                } => {
                    process_complex_type(
                        &types_path,
                        base,
                        dependencies,
                        description,
                        fields,
                        identifier,
                        nested,
                    );
                }
            }
        }
    }
    Ok(())
}
