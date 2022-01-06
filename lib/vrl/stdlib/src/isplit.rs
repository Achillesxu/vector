use vrl::prelude::*;
use std::str;

#[derive(Clone, Copy, Debug)]
pub struct Isplit;

impl Function for Isplit {
    fn identifier(&self) -> &'static str {
        "isplit"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "index",
                kind: kind::INTEGER,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "isplit non-ascii",
                source: r#"isplit!("中文测试", 2)"#,
                result: Ok(r#"["中文", "测试"]"#),
            },
            Example {
                title: "isplit ascii",
                source: r#"isplit!("english", 2)"#,
                result: Ok(r#"["en", "glish"]"#),
            },
        ]
    }

    fn compile(
        &self,
        _state: &state::Compiler,
        _ctx: &FunctionCompileContext,
        mut arguments: ArgumentList) -> Compiled {
        let value = arguments.required("value");
        let index = arguments.required("index");

        Ok(Box::new(IsplitFn {
            value,
            index
        }))
    }
}

#[derive(Debug, Clone)]
struct IsplitFn {
    value: Box<dyn Expression>,
    index: Box<dyn Expression>,
}

impl Expression for IsplitFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?.try_bytes()?;
        let index = self.index.resolve(ctx)?.try_integer()?;

        let value_str: &str = str::from_utf8(&value).unwrap();
        let index_usize: usize = index as usize;
        // let input_list = value_str.chars().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        // let front: String = input_list[..index_usize].concat();
        // let backed: String = input_list[index_usize..].concat();

        let front: String = value_str.chars().take(index_usize).collect();
        let front: &str = &front[..];
        let backed: &str = &value_str[front.len()..];

        let result = vec![front, backed];

        Ok(result.into())
    }

    fn type_def(&self, _: &state::Compiler) -> TypeDef {
        TypeDef::new()
            .infallible()
            .array_mapped::<(), Kind>(map! {(): Kind::Bytes})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        isplit => Isplit;

        bytes_0 {
            args: func_args![value: "中文测试",
                             index: 2
            ],
            want: Ok("["中文", "测试"]"),
            tdef: TypeDef::new().fallible().bytes(),
        }

    ];
}
