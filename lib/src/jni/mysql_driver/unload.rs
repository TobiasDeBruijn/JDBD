use jni::sys::{JNIEnv, jlong};
use jni::objects::JObject;
use mysql::Pool;

/**
 * - Class:      MysqlDriver
 * - Method:     unloadNative
 * - Signature:  `(J)V`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_unloadNative(_env: JNIEnv, _obj: JObject<'_>, ptr: jlong) {
    let pool_ptr = ptr as *mut Pool;
    let pool = unsafe { Box::from_raw(pool_ptr) };
    drop(pool);
}