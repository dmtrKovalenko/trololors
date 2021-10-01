use wasm_bindgen::prelude::*;

fn colorize(text: String, start_code: i8, end_code: i8) -> String {
    format!(
        r#"\u001B[{start_code}m{text}\u001B[{end_code}m"#,
        text = text,
        start_code = start_code,
        end_code = end_code
    )
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn reset(text: String) -> String {
    colorize(text, 0, 0)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bold(text: String) -> String {
    colorize(text, 1, 22)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn dim(text: String) -> String {
    colorize(text, 2, 22)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn italic(text: String) -> String {
    colorize(text, 3, 23)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn underline(text: String) -> String {
    colorize(text, 4, 24)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn overline(text: String) -> String {
    colorize(text, 53, 55)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn inverse(text: String) -> String {
    colorize(text, 7, 27)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn hidden(text: String) -> String {
    colorize(text, 8, 28)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn strikethrough(text: String) -> String {
    colorize(text, 9, 29)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn black(text: String) -> String {
    colorize(text, 30, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn red(text: String) -> String {
    colorize(text, 31, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn green(text: String) -> String {
    colorize(text, 32, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn yellow(text: String) -> String {
    colorize(text, 33, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blue(text: String) -> String {
    colorize(text, 34, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn magenta(text: String) -> String {
    colorize(text, 35, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn cyan(text: String) -> String {
    colorize(text, 36, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn white(text: String) -> String {
    colorize(text, 37, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn gray(text: String) -> String {
    colorize(text, 90, 39)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgBlack(text: String) -> String {
    colorize(text, 40, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgRed(text: String) -> String {
    colorize(text, 41, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgGreen(text: String) -> String {
    colorize(text, 42, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgYellow(text: String) -> String {
    colorize(text, 43, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgBlue(text: String) -> String {
    colorize(text, 44, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgMagenta(text: String) -> String {
    colorize(text, 45, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgCyan(text: String) -> String {
    colorize(text, 46, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgWhite(text: String) -> String {
    colorize(text, 47, 49)
}
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn bgGray(text: String) -> String {
    colorize(text, 100, 49)
}
