use std::{error::Error, fmt::Display};

use handlebars::Handlebars;

use serde::Serialize;

use crate::{args, meta::Meta};

#[derive(Debug, Serialize)]
pub struct TemplateData {
    pub project: String,
    pub level: String,
    pub description: Option<String>,
    #[serde(rename = "msg_table_header")]
    pub msg_tbl_header: Option<String>,
    #[serde(rename = "subj_table_header")]
    pub subj_tbl_header: Option<String>,
    #[serde(rename = "desc_table_header")]
    pub desc_tbl_header: Option<String>,
    #[serde(rename = "metas")]
    pub metas: Vec<TemplateMeta>,
}

impl TemplateData {
    pub fn new<T, L>(project: T, level: L, description: &Option<String>) -> TemplateData
    where
        L: Display,
        T: AsRef<str>,
    {
        let level = format!("{}", level);
        let project = project.as_ref().to_string();
        TemplateData {
            project,
            level,
            msg_tbl_header: Some("message".to_owned()),
            subj_tbl_header: Some("subject".to_owned()),
            desc_tbl_header: Some("description".to_owned()),
            description: description.clone(),
            metas: vec![],
        }
    }
    pub fn add_meta(&mut self, tm: TemplateMeta) {
        self.metas.push(tm);
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateMeta {
    pub message: String,
    pub subject: String,
    pub description: String,
}

impl From<Meta> for TemplateMeta {
    fn from(value: Meta) -> Self {
        let message = value.message.format();
        let subject = value.subject.format();
        let description = value.description.format();

        TemplateMeta {
            message,
            subject,
            description,
        }
    }
}
impl From<&Meta> for TemplateMeta {
    fn from(value: &Meta) -> Self {
        let message = value.message.format();
        let subject = value.subject.format();
        let description = value.description.format();

        TemplateMeta {
            message,
            subject,
            description,
        }
    }
}

pub fn render(
    templ_data: TemplateData,
    save_type: &args::SaveType,
) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let templ_string = template(save_type);
    let result = reg.render_template(&templ_string, &templ_data)?;
    Ok(result)
}

fn template(save_type: &args::SaveType) -> String {
    match save_type {
        args::SaveType::MD => String::from(
            r#"# {{ project }} - {{ level }} logs

{{ description }}

|{{msg_table_header}}|{{subj_table_header}}|{{desc_table_header}}|
|---|---|---|
{{#each metas as |meta|}}
| {{meta.message}} | {{meta.subject}} |{{{meta.description}}} |
{{/each}}
"#,
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::args;
    use crate::meta::Level;

    use super::render;
    use super::{TemplateData, TemplateMeta};

    #[test]
    fn render_test() {
        let st = args::SaveType::MD;
        let tm1 = TemplateMeta {
            message: "test message".to_string(),
            subject: "do nothing".to_string(),
            description: "".to_string(),
        };
        let tm2 = TemplateMeta {
            message: "test message".to_string(),
            subject: "do nothing".to_string(),
            description: "".to_string(),
        };

        let mut td = TemplateData::new(
            "project",
            Level::Info,
            &Some("some description\nhere".to_owned()),
        );
        td.add_meta(TemplateMeta {
            message: "msg1".to_owned(),
            subject: "subj1".to_owned(),
            description: "desc1".to_owned(),
        });
        let result = render(td, &st).unwrap();
        assert_eq!(result.len(), 114);
    }
}
