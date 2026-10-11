use crate::android::jni::{self as jni_helpers, JniExt};
use jni::objects::JValue;

pub fn launch_url(url: &str) -> Result<bool, String> {
    jni_helpers::launch_view_intent(url)
}

pub fn can_launch_url(url: &str) -> Result<bool, String> {
    let url = url.to_owned();
    jni_helpers::with_env(|env| {
        let activity = jni_helpers::activity(env)?;

        let intent = jni_helpers::view_intent(env, &url)?;

        // activity.getPackageManager()
        let pm = env
            .call_method(
                &activity,
                jni::jni_str!("getPackageManager"),
                jni::jni_sig!("()Landroid/content/pm/PackageManager;"),
                &[],
            )
            .and_then(|v| v.l())
            .e()?;
        if pm.is_null() {
            return Err("getPackageManager returned null".into());
        }

        // pm.resolveActivity(intent, 0)
        let resolved = env
            .call_method(
                &pm,
                jni::jni_str!("resolveActivity"),
                jni::jni_sig!("(Landroid/content/Intent;I)Landroid/content/pm/ResolveInfo;"),
                &[JValue::Object(&intent), JValue::Int(0)],
            )
            .and_then(|v| v.l());

        match resolved {
            Ok(r) => Ok(!r.is_null()),
            Err(_) => {
                env.exception_clear();
                Ok(false)
            }
        }
    })
}
