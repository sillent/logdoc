use std::collections::HashMap;

use clap::Parser;
use env_logger;
use log::debug;

use crate::args;
use crate::files;
use crate::meta::{Level, Message, Meta, Pos, Typo};
use crate::template::render;
use crate::template::TemplateData;

pub struct Application;

impl Application {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();
        let arg = args::Arg::parse();
        let mut parse = tree_sitter::Parser::new();
        let lang = &arg.language;
        parse.set_language(&lang.sitter_language()).or(Err(format!(
            "Failed to load {} tree-sitter language",
            &lang
        )))?;
        let files = files::form_list_files(&arg)?;

        let query = tree_sitter::Query::new(&lang.sitter_language(), &lang.query())?;
        let language_comment = lang.comment();

        let mut templates: HashMap<Level, TemplateData> = Level::ALL
            .iter()
            .map(|&lvl| (lvl, TemplateData::new(&arg, lvl)))
            .collect();

        for file in files {
            log::debug!("processing {file:?}");
            let file_bytes = std::fs::read_to_string(file)?;
            let tree = parse
                .parse(&file_bytes.as_bytes(), None)
                .ok_or("Failed to parse data")?;
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let query_matches =
                query_cursor.matches(&query, tree.root_node(), file_bytes.as_bytes());

            for query_match in query_matches {
                log::debug!("processing query match := {:?}", query_match);
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

                if let Some(tpl) = templates.get_mut(&m.level) {
                    tpl.add_meta(tmeta);
                }
            }
        }
        for (level, data) in templates {
            if data.is_empty() {
                debug!("skippking {:?}, no metadata found", level);
                continue;
            }
            let rendered = render(data, &arg.save_type)?;
            files::save_string_to_file(rendered, &level, &arg)?;
        }

        Ok(())
    }
}
