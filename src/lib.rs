use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FromInto)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(_input as DeriveInput);
    let _self = &ast.ident;
    let _other = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) => {
                    if fields.unnamed.is_empty() {
                        panic!("There is no unnamed parameter in struct")
                    }
                    if fields.unnamed.iter().count().ne(&1) {
                        panic!("There are more than one unnamed parameter in struct")
                    } else {
                        fields.unnamed.first().unwrap()
                    }
                }
                _ => panic!("There is no unnamed parameter in struct")
            }
        }
        _ => panic!("This is not a struct")
    };
    let gen = quote! {
        impl From<#_other> for #_self{
            fn from(value: #_other) -> Self{
                Self(value)
            }
        }
        impl Into<#_other> for #_self{
            fn into(self) -> #_other{
                self.0
            }
        }
    };
    gen.into()
}

#[allow(non_snake_case)]
#[proc_macro]
/// # 将字符串转换为PCWSTR类型
/// ## 依赖
/// windows 和 widestring
/// ## 参数
/// - input: `&str`和`vec<u16>`
/// - output: `PCWSTR`
/// ## 注意
/// 使用`vec<u16>`时，需要在前面加上vec以便区别开来
/// 例如:
/// ```
/// use windows_macro::PCWSTR;
/// let s:Vec<u16> = Vec::new();
/// PCWSTR!(vec s);
/// ```
pub fn PCWSTR(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    // 使用正则表达式或其他方法来判断输入形式
    if input_str.starts_with("vec") {
        // 处理输入是 vec 的情况
        let trim_input = input_str.trim_start_matches("vec").trim();  // 去除 "vec" 并去除空格
        let output = format!("windows::core::PCWSTR::from_raw(widestring::U16CString::from_vec({}).unwrap().into_raw())", trim_input);
        output.parse().unwrap()
    } else {
        // 处理其他情况
        let output = format!("windows::core::PCWSTR::from_raw(widestring::U16CString::from_str({}).unwrap().into_raw())", input_str);
        output.parse().unwrap()
    }
}

#[allow(non_snake_case)]
#[proc_macro]
/// # 将字符串转换为PWSTR类型
/// ## 依赖
/// windows 和 widestring
/// ## 参数
/// - input: `&str`和`vec<u16>`
/// - output: `PWSTR`
/// ## 注意
/// 使用`vec<u16>`时，需要在前面加上vec以便区别开来
/// 例如:
/// ```
/// use windows_macro::PWSTR;
/// let s:Vec<u16> = Vec::new();
/// PWSTR!(vec s);
/// ```
pub fn PWSTR(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    // 通过输入的形式进行区分
    if input_str.starts_with("vec") {
        let trim_input = input_str.trim_start_matches("vec").trim();
        let output = format!("windows::core::PWSTR::from_raw(widestring::U16CString::from_vec({}).unwrap().into_raw())", trim_input);
        output.parse().unwrap()
    } else {
        let output = format!("windows::core::PWSTR::from_raw(widestring::U16CString::from_str({}).unwrap().into_raw())", input_str);
        output.parse().unwrap()
    }
}