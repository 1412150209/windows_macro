#[allow(non_snake_case)]
#[macro_export]
/// # 将字符串转换为PWSTR类型
/// ## 参数
/// - input: literal(字面类型)|expr(表达式,支持`&str`和`vec<u16>`)
/// - output: `PCWSTR`
/// ## 注意
/// 使用`vec<u16>`时，需要在前面加上vec以便区别开来
/// 例如:
/// ```
/// use windows_macro::PCWSTR;
/// let s:Vec<u16> = Vec::new();
/// PCWSTR!(vec s);
/// ```
macro_rules! PCWSTR {
        ($s:literal) => {
            windows::core::PCWSTR::from_raw(
                widestring::U16CString::from_str($s).unwrap().into_raw(),
            )
        };
        ($s:expr) => {
            windows::core::PCWSTR::from_raw(
                widestring::U16CString::from_str($s).unwrap().into_raw(),
            )
        };
        (vec $s:expr) => {
            windows::core::PCWSTR::from_raw(
                widestring::U16CString::from_vec($s).unwrap().into_raw(),
            )
        };
    }

#[allow(non_snake_case)]
#[macro_export]
/// # 将字符串转换为PWSTR类型
/// ## 参数
/// - input: literal(字面类型)|expr(表达式,支持`&str`)
/// - output: `PWSTR`
macro_rules! PWSTR {
        ($s:literal) => {
            windows::core::PWSTR::from_raw(widestring::U16CString::from_str($s).unwrap().into_raw())
        };
        ($s:expr) => {
            windows::core::PWSTR::from_raw(widestring::U16CString::from_str($s).unwrap().into_raw())
        };
    }