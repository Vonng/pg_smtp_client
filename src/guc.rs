use pgrx::*;
use std::ffi::CString;

pub static SMTP_SERVER: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static SMTP_PORT: GucSetting<i32> = GucSetting::<i32>::new(587);
pub static SMTP_TLS: GucSetting<bool> = GucSetting::<bool>::new(true);
pub static SMTP_USERNAME: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static SMTP_PASSWORD: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static SMTP_FROM: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);

pub fn init() {
    GucRegistry::define_string_guc(
        c"smtp_client.server",
        c"The SMTP server to use for sending emails",
        c"The SMTP server to use for sending emails.",
        &SMTP_SERVER,
        GucContext::Suset,
        GucFlags::default(),
    );

    GucRegistry::define_int_guc(
        c"smtp_client.port",
        c"The port to use for the SMTP server",
        c"The port to use for the SMTP server.",
        &SMTP_PORT,
        1,
        65535,
        GucContext::Suset,
        GucFlags::default(),
    );

    GucRegistry::define_bool_guc(
        c"smtp_client.tls",
        c"Whether to use TLS for the SMTP connection",
        c"Whether to use TLS for the SMTP connection.",
        &SMTP_TLS,
        GucContext::Suset,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"smtp_client.username",
        c"The username to use for the SMTP server",
        c"The username to use for the SMTP server.",
        &SMTP_USERNAME,
        GucContext::Suset,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"smtp_client.password",
        c"The password to use for the SMTP server",
        c"The password to use for the SMTP server.",
        &SMTP_PASSWORD,
        GucContext::Suset,
        GucFlags::SUPERUSER_ONLY,
    );

    GucRegistry::define_string_guc(
        c"smtp_client.from_address",
        c"The address the email should be sent from",
        c"The address the email should be sent from.",
        &SMTP_FROM,
        GucContext::Suset,
        GucFlags::default(),
    );
}

pub fn get_smtp_server() -> Option<String> {
    SMTP_SERVER.get().and_then(|cstr| cstr.to_str().ok().map(|s| s.to_string()))
}

pub fn get_smtp_port() -> u16 {
    SMTP_PORT.get() as u16
}

pub fn get_smtp_tls() -> bool {
    SMTP_TLS.get()
}

pub fn get_smtp_username() -> Option<String> {
    SMTP_USERNAME.get().and_then(|cstr| cstr.to_str().ok().map(|s| s.to_string()))
}

pub fn get_smtp_password() -> Option<String> {
    SMTP_PASSWORD.get().and_then(|cstr| cstr.to_str().ok().map(|s| s.to_string()))
}

pub fn get_smtp_from() -> Option<String> {
    SMTP_FROM.get().and_then(|cstr| cstr.to_str().ok().map(|s| s.to_string()))
}
