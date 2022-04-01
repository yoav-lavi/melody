use neon::prelude::*;

fn compiler(mut cx: FunctionContext) -> JsResult<JsString> {
    let source: Handle<JsString> = cx.argument(0)?;

    let compiled = match melody_compiler::compiler(&source.value(&mut cx)) {
        Ok(value) => value,
        Err(error) => return cx.throw_error(error.to_string()),
    };
    Ok(cx.string(compiled))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("compiler", compiler)?;
    Ok(())
}
