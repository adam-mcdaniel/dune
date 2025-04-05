use common_macros::b_tree_map;
use dune::{Error, Expression};
use regex_lite::Regex;

pub fn get() -> Expression {
    (b_tree_map! {
        // 编译正则表达式（实际使用时直接传字符串更高效，这里提供兼容接口）
        String::from("new") => Expression::builtin("new", |args, env| {
            super::check_exact_args_len("new", &args, 1)?;
            let pattern = args[0].eval(env)?.to_string();
            Regex::new(&pattern)
                .map(|_| Expression::String(pattern)) // 仅验证正则有效性
                .map_err(|e| Error::CustomError(e.to_string()))
        }, "compile regex pattern (validation only)"),

        // 检查是否匹配
        String::from("is-match") => Expression::builtin("is-match", |args, env| {
            super::check_exact_args_len("is-match", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;
            Ok(Expression::Boolean(regex.is_match(&text)))
        }, "check if text matches pattern"),

        // 查找第一个匹配
        String::from("find") => Expression::builtin("find", |args, env| {
            super::check_exact_args_len("find", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            regex.find(&text).map_or(
                Ok(Expression::None),
                |m| Ok(Expression::List(vec![
                    Expression::Integer(m.start() as i64),
                    Expression::Integer(m.end() as i64),
                    Expression::String(m.as_str().to_string())
                ]))
            )
        }, "find first match with [start, end, text]"),

        // 查找所有匹配
        String::from("find-all") => Expression::builtin("find-all", |args, env| {
            super::check_exact_args_len("find-all", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            let matches: Vec<Expression> = regex.find_iter(&text).map(|m| {
                Expression::List(vec![
                    Expression::Integer(m.start() as i64),
                    Expression::Integer(m.end() as i64),
                    Expression::String(m.as_str().to_string())
                ])
            }).collect();

            Ok(Expression::List(matches))
        }, "find all matches as [[start, end, text], ...]"),

        // 获取第一个捕获组
        String::from("captures") => Expression::builtin("captures", |args, env| {
            super::check_exact_args_len("captures", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            regex.captures(&text).map_or(
                Ok(Expression::None),
                |caps| {
                    let groups: Vec<Expression> = (0..caps.len()).map(|i| {
                        caps.get(i).map(|m| Expression::String(m.as_str().to_string()))
                            .unwrap_or(Expression::None)
                    }).collect();
                    Ok(Expression::List(groups))
                }
            )
        }, "get first capture groups as [full, group1, group2, ...]"),

        // 获取所有捕获组
        String::from("captures-all") => Expression::builtin("captures-all", |args, env| {
            super::check_exact_args_len("captures-all", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            let all_caps: Vec<Expression> = regex.captures_iter(&text).map(|caps| {
                let groups: Vec<Expression> = (0..caps.len()).map(|i| {
                    caps.get(i).map(|m| Expression::String(m.as_str().to_string()))
                        .unwrap_or(Expression::None)
                }).collect();
                Expression::List(groups)
            }).collect();

            Ok(Expression::List(all_caps))
        }, "get all captures as [[full, group1, ...], ...]"),

        // 正则分割
        String::from("split") => Expression::builtin("split", |args, env| {
            super::check_exact_args_len("split", &args, 2)?;
            let pattern = args[0].eval(env)?.to_string();
            let text = args[1].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            let parts: Vec<Expression> = regex.split(&text)
                .map(|s| Expression::String(s.to_string()))
                .collect();

            Ok(Expression::List(parts))
        }, "split text by pattern"),

        // 替换所有匹配
        String::from("replace-all") => Expression::builtin("replace-all", |args, env| {
            super::check_exact_args_len("replace-all", &args, 3)?;
            let pattern = args[0].eval(env)?.to_string();
            let replacement = args[1].eval(env)?.to_string();
            let text = args[2].eval(env)?.to_string();
            let regex = Regex::new(&pattern).map_err(|e| Error::CustomError(e.to_string()))?;

            Ok(Expression::String(
                regex.replace_all(&text, replacement.as_str()).to_string()
            ))
        }, "replace all matches in text")
    })
    .into()
}
