use clap::Parser;

use crate::args;
use crate::files;
use crate::meta::{Level, Message, Meta, Pos, Typo};
use crate::template::render;
use crate::template::TemplateData;

pub struct Application;

impl Application {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let arg = args::Arg::parse();
        let mut parse = tree_sitter::Parser::new();
        let lang = crate::language::Language::from(&arg.language);
        parse.set_language(&lang.sitter_language()).or(Err(format!(
            "Failed to load {} tree-sitter language",
            &lang
        )))?;
        let files = files::form_list_files(&arg)?;

        let query = tree_sitter::Query::new(&lang.sitter_language(), &lang.query())?;
        let language_comment = lang.comment();

        let mut template_data_info = TemplateData::new(&arg, Level::Info);

        let mut template_data_debug = TemplateData::new(&arg, Level::Debug);

        let mut template_data_trace = TemplateData::new(&arg, Level::Trace);

        let mut template_data_warn = TemplateData::new(&arg, Level::Warn);

        let mut template_data_error = TemplateData::new(&arg, Level::Error);

        let mut template_data_fatal = TemplateData::new(&arg, Level::Fatal);

        for file in files {
            let file_bytes = std::fs::read_to_string(file)?;
            let tree = parse
                .parse(&file_bytes.as_bytes(), None)
                .ok_or("Failed to parse data")?;
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let query_matches =
                query_cursor.matches(&query, tree.root_node(), file_bytes.as_bytes());

            for query_match in query_matches {
                let mut m = Meta::default();
                for query_capture in query_match.captures {
                    let position = Pos::from(query_capture);
                    let query_bytes = files::search_in_file_dyn(&file_bytes.as_bytes(), &position);
                    let data = String::from_utf8_lossy(&query_bytes).to_string();
                    if position.typo == Typo::Level {
                        let level = Level::from(&data);
                        m.level = level;
                    }
                    if position.typo == Typo::Comments {
                        if m.comments.subject.is_empty() {
                            m.comments.subject = language_comment.remove(&data);
                        } else {
                            m.comments.description.push(language_comment.remove(&data));
                        }
                    }
                    if position.typo == Typo::Content {
                        m.message = Message::from(&data);
                    }
                }
                let tmeta = crate::template::TemplateMeta::from(&m);
                match m.level {
                    Level::Trace => template_data_trace.add_meta(tmeta),
                    Level::Debug => template_data_debug.add_meta(tmeta),
                    Level::Info => template_data_info.add_meta(tmeta),
                    Level::Warn => template_data_warn.add_meta(tmeta),
                    Level::Error => template_data_error.add_meta(tmeta),
                    Level::Fatal => template_data_fatal.add_meta(tmeta),
                }
            }
        }
        let template_str_trace = render(template_data_trace, &arg.save_type)?;
        let template_str_debug = render(template_data_debug, &arg.save_type)?;
        let template_str_info = render(template_data_info, &arg.save_type)?;
        let template_str_warn = render(template_data_warn, &arg.save_type)?;
        let template_str_error = render(template_data_error, &arg.save_type)?;
        let template_str_fatal = render(template_data_fatal, &arg.save_type)?;
        files::save_string_to_file(template_str_trace, &Level::Trace, &arg)?;
        files::save_string_to_file(template_str_debug, &Level::Debug, &arg)?;
        files::save_string_to_file(template_str_info, &Level::Info, &arg)?;
        files::save_string_to_file(template_str_warn, &Level::Warn, &arg)?;
        files::save_string_to_file(template_str_error, &Level::Error, &arg)?;
        files::save_string_to_file(template_str_fatal, &Level::Fatal, &arg)?;

        Ok(())
    }
}
