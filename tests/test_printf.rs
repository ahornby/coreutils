use common::util::*;

static UTIL_NAME: &'static str = "printf";
fn new_ucmd() -> UCommand {
    TestScenario::new(UTIL_NAME).ucmd()
}

#[test]
fn basic_literal() {
    new_ucmd().args(&["hello world"]).succeeds().stdout_only("hello world");
}

#[test]
fn escaped_tab() {
    new_ucmd().args(&["hello\\t world"]).succeeds().stdout_only("hello\t world");
}

#[test]
fn escaped_newline() {
    new_ucmd().args(&["hello\\n world"]).succeeds().stdout_only("hello\n world");
}

#[test]
fn escaped_slash() {
    new_ucmd().args(&["hello\\\\ world"]).succeeds().stdout_only("hello\\ world");
}

#[test]
fn escaped_hex() {
    new_ucmd().args(&["\\x41"]).succeeds().stdout_only("A");
}

#[test]
fn escaped_octal() {
    new_ucmd().args(&["\\101"]).succeeds().stdout_only("A");
}

#[test]
fn escaped_unicode_fourdigit() {
    new_ucmd().args(&["\\u0125"]).succeeds().stdout_only("ĥ");
}

#[test]
fn escaped_unicode_eightdigit() {
    new_ucmd().args(&["\\U00000125"]).succeeds().stdout_only("ĥ");
}

#[test]
fn escaped_percent_sign() {
    new_ucmd().args(&["hello%% world"]).succeeds().stdout_only("hello% world");
}

#[test]
fn escaped_unrecognized() {
    new_ucmd().args(&["c\\d"]).succeeds().stdout_only("c\\d");
}

#[test]
fn sub_string() {
    new_ucmd().args(&["hello %s", "world"]).succeeds().stdout_only("hello world");
}

#[test]
fn sub_multifield() {
    new_ucmd().args(&["%s %s", "hello", "world"]).succeeds().stdout_only("hello world");
}

#[test]
fn sub_repeat_formatstr() {
    new_ucmd().args(&["%s.", "hello", "world"]).succeeds().stdout_only("hello.world.");
}

#[test]
fn sub_string_ignore_escapes() {
    new_ucmd().args(&["hello %s", "\\tworld"]).succeeds().stdout_only("hello \\tworld");
}

#[test]
fn sub_bstring_handle_escapes() {
    new_ucmd().args(&["hello %b", "\\tworld"]).succeeds().stdout_only("hello \tworld");
}

#[test]
fn sub_bstring_ignore_subs() {
    new_ucmd().args(&["hello %b", "world %% %i"]).succeeds().stdout_only("hello world %% %i");
}

#[test]
fn sub_char() {
    new_ucmd().args(&["the letter %c", "A"]).succeeds().stdout_only("the letter A");
}

#[test]
fn sub_num_int() {
    new_ucmd().args(&["twenty is %i", "20"]).succeeds().stdout_only("twenty is 20");
}

#[test]
fn sub_num_int_minwidth() {
    new_ucmd().args(&["twenty is %1i", "20"]).succeeds().stdout_only("twenty is 20");
}

#[test]
fn sub_num_int_neg() {
    new_ucmd().args(&["neg. twenty is %i", "-20"]).succeeds().stdout_only("neg. twenty is -20");
}

#[test]
fn sub_num_int_oct_in() {
    new_ucmd().args(&["twenty is %i", "024"]).succeeds().stdout_only("twenty is 20");
}

#[test]
fn sub_num_int_oct_in_neg() {
    new_ucmd().args(&["neg. twenty is %i", "-024"]).succeeds().stdout_only("neg. twenty is -20");
}

#[test]
fn sub_num_int_hex_in() {
    new_ucmd().args(&["twenty is %i", "0x14"]).succeeds().stdout_only("twenty is 20");
}

#[test]
fn sub_num_int_hex_in_neg() {
    new_ucmd().args(&["neg. twenty is %i", "-0x14"]).succeeds().stdout_only("neg. twenty is -20");
}

#[test]
fn sub_num_int_charconst_in() {
    new_ucmd().args(&["ninetyseven is %i", "'a"]).succeeds().stdout_only("ninetyseven is 97");
}

#[test]
fn sub_num_uint() {
    new_ucmd().args(&["twenty is %u", "20"]).succeeds().stdout_only("twenty is 20");
}

#[test]
fn sub_num_octal() {
    new_ucmd().args(&["twenty in octal is %o", "20"]).succeeds().stdout_only("twenty in octal is 24");
}

#[test]
fn sub_num_hex_lower() {
    new_ucmd().args(&["thirty in hex is %x", "30"]).succeeds().stdout_only("thirty in hex is 1e");
}

#[test]
fn sub_num_hex_upper() {
    new_ucmd().args(&["thirty in hex is %X", "30"]).succeeds().stdout_only("thirty in hex is 1E");
}

#[test]
fn sub_num_float() {
    new_ucmd().args(&["twenty is %f", "20"]).succeeds().stdout_only("twenty is 20.000000");
}

#[test]
fn sub_num_float_round() {
    new_ucmd().args(&["two is %f", "1.9999995"]).succeeds().stdout_only("two is 2.000000");
}

#[test]
fn sub_num_sci_lower() {
    new_ucmd().args(&["twenty is %e", "20"]).succeeds().stdout_only("twenty is 2.000000e+01");
}

#[test]
fn sub_num_sci_upper() {
    new_ucmd().args(&["twenty is %E", "20"]).succeeds().stdout_only("twenty is 2.000000E+01");
}

#[test]
fn sub_num_sci_trunc() {
    new_ucmd().args(&["pi is ~ %e", "3.1415926535"]).succeeds().stdout_only("pi is ~ 3.141593e+00");
}

#[test]
fn sub_num_dec_trunc() {
    new_ucmd().args(&["pi is ~ %g", "3.1415926535"]).succeeds().stdout_only("pi is ~ 3.141593");
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn sub_num_hex_float_lower() {
    new_ucmd().args(&["%a", ".875"]).succeeds().stdout_only("0xep-4");
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn sub_num_hex_float_upper() {
    new_ucmd().args(&["%A", ".875"]).succeeds().stdout_only("0XEP-4");
}

#[test]
fn sub_minwidth() {
    new_ucmd().args(&["hello %7s", "world"]).succeeds().stdout_only("hello   world");
}

#[test]
fn sub_minwidth_negative() {
    new_ucmd().args(&["hello %-7s", "world"]).succeeds().stdout_only("hello world  ");
}

#[test]
fn sub_str_max_chars_input() {
    new_ucmd().args(&["hello %7.2s", "world"]).succeeds().stdout_only("hello      wo");
}

#[test]
fn sub_int_decimal() {
    new_ucmd().args(&["%0.i", "11"]).succeeds().stdout_only("11");
}

#[test]
fn sub_int_leading_zeroes() {
    new_ucmd().args(&["%.4i", "11"]).succeeds().stdout_only("0011");
}

#[test]
fn sub_int_leading_zeroes_prio() {
    new_ucmd().args(&["%5.4i", "11"]).succeeds().stdout_only(" 0011");
}

#[test]
fn sub_float_dec_places() {
    new_ucmd().args(&["pi is ~ %.11f", "3.1415926535"]).succeeds().stdout_only("pi is ~ 3.14159265350");
}

#[test]
fn sub_float_hex_in() {
    new_ucmd().args(&["%f", "0xF1.1F"]).succeeds().stdout_only("241.121094");
}

#[test]
fn sub_float_no_octal_in() {
    new_ucmd().args(&["%f", "077"]).succeeds().stdout_only("77.000000");
}

#[test]
fn sub_any_asterisk_firstparam() {
    new_ucmd().args(&["%*i", "3", "11", "4", "12"]).succeeds().stdout_only(" 11  12");
}

#[test]
fn sub_any_asterisk_second_param() {
    new_ucmd().args(&["%.*i", "3", "11", "4", "12"]).succeeds().stdout_only("0110012");
}

#[test]
fn sub_any_asterisk_both_params() {
    new_ucmd().args(&["%*.*i", "4", "3", "11", "5", "4", "12"]).succeeds().stdout_only(" 011 0012");
}

#[test]
fn sub_any_asterisk_octal_arg() {
    new_ucmd().args(&["%.*i", "011", "12345678"]).succeeds().stdout_only("012345678");
}

#[test]
fn sub_any_asterisk_hex_arg() {
    new_ucmd().args(&["%.*i", "0xA", "123456789"]).succeeds().stdout_only("0123456789");
}

#[test]
fn sub_any_specifiers_no_params() {
    new_ucmd().args(&["%ztlhLji", "3"]).succeeds().stdout_only("3");
}

#[test]
fn sub_any_specifiers_after_first_param() {
    new_ucmd().args(&["%0ztlhLji", "3"]).succeeds().stdout_only("3");
}

#[test]
fn sub_any_specifiers_after_period() {
    new_ucmd().args(&["%0.ztlhLji", "3"]).succeeds().stdout_only("3");
}

#[test]
fn sub_any_specifiers_after_second_param() {
    new_ucmd().args(&["%0.0ztlhLji", "3"]).succeeds().stdout_only("3");
}