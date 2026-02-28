use std::error::Error;

use handlebars::Handlebars;

use serde::Serialize;

use crate::{
    args::{self, Arg},
    meta::{Level, Meta},
};

#[derive(Debug, Serialize)]
pub struct TemplateData {
    pub project: String,
    pub level: String,
    pub description: Option<String>,
    #[serde(rename = "msg_table_header")]
    pub msg_tbl_header: String,
    #[serde(rename = "subj_table_header")]
    pub subj_tbl_header: String,
    #[serde(rename = "desc_table_header")]
    pub desc_tbl_header: String,
    #[serde(rename = "metas")]
    pub metas: Vec<TemplateMeta>,
    pub separator: String,
}

impl TemplateData {
    pub fn new(arg: &Arg, meta_level: Level) -> TemplateData {
        let level: String = meta_level.to_string();
        let project = arg.project_name.clone();
        let description = match meta_level {
            Level::Trace => arg.trace_desc.clone(),
            Level::Debug => arg.debug_desc.clone(),
            Level::Info => arg.info_desc.clone(),
            Level::Warn => arg.warn_desc.clone(),
            Level::Error => arg.error_desc.clone(),
            Level::Fatal => arg.fatal_desc.clone(),
        };
        let msg_tbl_header = arg
            .message_table_header
            .clone()
            .unwrap_or("message".to_string());
        let subj_tbl_header = arg
            .subject_table_header
            .clone()
            .unwrap_or("subject".to_owned());
        let desc_tbl_header = arg
            .description_table_header
            .clone()
            .unwrap_or("description".to_owned());
        TemplateData {
            project,
            level,
            msg_tbl_header,
            subj_tbl_header,
            desc_tbl_header,
            description,
            metas: vec![],
            separator: arg.new_line_separator.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.metas.is_empty()
    }
    pub fn add_meta(&mut self, tm: TemplateMeta) {
        self.metas.push(tm);
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateMeta {
    pub message: String,
    pub subject: String,
    pub description: Vec<String>,
}

impl From<&Meta> for TemplateMeta {
    fn from(value: &Meta) -> Self {
        let message = value.message.format();
        let subject = value.comments.format_subject();
        let description = value.comments.format_description();

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
    let mut handlebar_registry = Handlebars::new();
    handlebar_registry.register_escape_fn(handlebars::no_escape);
    let templ_string = template(save_type);
    let result = handlebar_registry.render_template(&templ_string, &templ_data)?;
    Ok(result)
}

fn template(save_type: &args::SaveType) -> String {
    match save_type {
        args::SaveType::MD => String::from(
            r#"# {{ project }} - {{ level }} logs
{{#if description}}{{ description }}{{/if}}

|{{msg_table_header}}|{{subj_table_header}}|{{desc_table_header}}|
|---|---|---|
{{#each metas}}
| {{message}} | {{subject}} | {{#each description}}{{this}}{{#unless @last}}{{{../../separator}}}{{/unless}}{{/each}} |
{{/each}}"#,
        ),
        args::SaveType::CSV => String::from(
            r#"{{ #each metas}}
{{message}};{{subject}};{{#each description}}{{this}}{{#unless @last}}{{{../../separator}}}{{/unless}}{{/each}}
{{/each}}"#,
        ),
    }
}
