extern crate jni;

use jni::objects::{JClass, JObject, JString, JValue, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray};
use jni::JNIEnv;
use quiche::{h3, Config, Connection, Error, Header, StreamIter, Type};
use std::pin::Pin;
use std::slice;

type JNIResult<T> = Result<T, jni::errors::Error>;

fn h3_error_code(error: h3::Error) -> i32 {
    match error {
        h3::Error::Done => -1,
        h3::Error::BufferTooShort => -2,
        h3::Error::InternalError => -3,
        h3::Error::ExcessiveLoad => -4,
        h3::Error::IdError => -5,
        h3::Error::StreamCreationError => -6,
        h3::Error::ClosedCriticalStream => -7,
        h3::Error::MissingSettings => -8,
        h3::Error::FrameUnexpected => -9,
        h3::Error::FrameError => -10,
        h3::Error::QpackDecompressionFailed => -11,
        h3::Error::TransportError { .. } => -12,
        h3::Error::StreamBlocked => -13,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1new(
    _env: JNIEnv,
    _class: JClass,
    version: jint,
) -> jlong {
    let config = Config::new(version as u32).unwrap();
    Box::into_raw(Box::new(config)) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1load_1cert_1chain_1from_1pem_1file(
    env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    path: JString,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    let path_str: String = env.get_string(path).unwrap().into();
    config.load_cert_chain_from_pem_file(&path_str).unwrap();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1load_1priv_1key_1from_1pem_1file(
    env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    path: JString,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    let path_str: String = env.get_string(path).unwrap().into();
    config.load_priv_key_from_pem_file(&path_str).unwrap();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1verify_1peer(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jboolean,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.verify_peer(v != 0);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1grease(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jboolean,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.grease(v != 0);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1log_1keys(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.log_keys();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1enable_1early_1data(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.enable_early_data();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1application_1protos(
    env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    protos: jbyteArray,
) -> jint {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    let protos_bytes: Vec<u8> = env.convert_byte_array(protos).unwrap();
    match config.set_application_protos(&protos_bytes[..]) {
        Ok(_) => 0 as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1max_1idle_1timeout(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_max_idle_timeout(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1max_1udp_1payload_1size(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_max_udp_payload_size(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1data(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_data(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1stream_1data_1bidi_1local(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_stream_data_bidi_local(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1stream_1data_1bidi_1remote(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_stream_data_bidi_remote(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1stream_1data_1uni(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_stream_data_uni(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1streams_1bidi(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_streams_bidi(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1initial_1max_1streams_1uni(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_initial_max_streams_uni(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1ack_1delay_1exponent(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_ack_delay_exponent(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1max_1ack_1delay(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jlong,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_max_ack_delay(v as u64);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1disable_1active_1migration(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jboolean,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.set_disable_active_migration(v != 0);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1set_1cc_1algorithm_1name(
    env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    name: JString,
) -> jint {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    let name_str: String = env.get_string(name).unwrap().into();
    match config.set_cc_algorithm_name(&name_str) {
        Ok(_) => 0 as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1config_1enable_1hystart(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
    v: jboolean,
) {
    let config = unsafe { &mut *(config_ptr as *mut Config) };
    config.enable_hystart(v != 0);
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1accept(
    env: JNIEnv,
    _class: JClass,
    scid_java: jbyteArray,
    odcid_java: jbyteArray,
    config_ptr: jlong,
) -> jlong {
    let mut config = unsafe { &mut *(config_ptr as *mut Config) };
    let scid: Vec<u8> = env.convert_byte_array(scid_java).unwrap();
    let odcid: Vec<u8> = env.convert_byte_array(odcid_java).unwrap();
    // xxx(okachaiev): using None for odcid here but when using retry()
    // it should be an actual token
    let conn = quiche::accept(&scid[..], None, &mut config).unwrap();
    Box::into_raw(Pin::into_inner(conn)) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1connect(
    env: JNIEnv,
    _class: JClass,
    domain: JString,
    conn_id: jbyteArray,
    config_ptr: jlong,
) -> jlong {
    let mut domain: String = env.get_string(domain).unwrap().into();
    let mut config = unsafe { &mut *(config_ptr as *mut Config) };
    let scid: Vec<u8> = env.convert_byte_array(conn_id).unwrap();
    let conn = quiche::connect(Some(domain.as_mut_str()), &scid, &mut config).unwrap();
    Box::into_raw(Pin::into_inner(conn)) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1negotiate_1version(
    env: JNIEnv,
    _class: JClass,
    java_scid: jbyteArray,
    java_dcid: jbyteArray,
    java_buf: jbyteArray,
) -> jint {
    let scid = env.convert_byte_array(java_scid).unwrap();
    let dcid = env.convert_byte_array(java_dcid).unwrap();
    let buf_len = env.get_array_length(java_buf).unwrap() as usize;
    let (ptr, _is_copy) = env.get_byte_array_elements(java_buf).unwrap();
    let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, buf_len) };
    let len = quiche::negotiate_version(&scid[..], &dcid[..], buf);
    env.release_byte_array_elements(
        java_buf,
        unsafe { ptr.as_mut().unwrap() },
        ReleaseMode::CopyBack,
    )
    .unwrap();
    match len {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1version_1is_1supported(
    _env: JNIEnv,
    _class: JClass,
    version: jint,
) -> jboolean {
    quiche::version_is_supported(version as u32) as jboolean
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1retry(
    env: JNIEnv,
    _class: JClass,
    java_scid: jbyteArray,
    java_dcid: jbyteArray,
    java_new_scid: jbyteArray,
    java_token: jbyteArray,
    version: jint,
    java_buf: jbyteArray,
) -> jint {
    let scid = env.convert_byte_array(java_scid).unwrap();
    let dcid = env.convert_byte_array(java_dcid).unwrap();
    let new_scid = env.convert_byte_array(java_new_scid).unwrap();
    let token = env.convert_byte_array(java_token).unwrap();
    let buf_len = env.get_array_length(java_buf).unwrap() as usize;
    let (ptr, _is_copy) = env.get_byte_array_elements(java_buf).unwrap();
    let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, buf_len) };
    let len = quiche::retry(
        &scid[..],
        &dcid[..],
        &new_scid[..],
        &token[..],
        version as u32,
        buf,
    );
    env.release_byte_array_elements(
        java_buf,
        unsafe { ptr.as_mut().unwrap() },
        ReleaseMode::CopyBack,
    )
    .unwrap();
    match len {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1recv(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    java_buf: jbyteArray,
) -> jint {
    let conn = unsafe { &mut *(ptr as *mut Connection) };
    // internally executes GetByteArrayRegion
    let mut buf = env.convert_byte_array(java_buf).unwrap();
    match conn.recv(&mut buf) {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1send(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    java_buf: jbyteArray,
) -> jint {
    let conn = unsafe { &mut *(ptr as *mut Connection) };
    let buf_len = env.get_array_length(java_buf).unwrap() as usize;
    let (ptr, _is_copy) = env.get_byte_array_elements(java_buf).unwrap();
    let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, buf_len) };
    let sent_len = conn.send(buf);
    env.release_byte_array_elements(
        java_buf,
        unsafe { ptr.as_mut().unwrap() },
        ReleaseMode::CopyBack,
    )
    .unwrap();
    match sent_len {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1close(
    env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    app: jboolean,
    error: jlong,
    reason: jbyteArray,
) -> jint {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let reason_bytes = env.convert_byte_array(reason).unwrap();
    match conn.close(app != 0, error as u64, &reason_bytes[..]) {
        Ok(_) => 0 as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1on_1timeout(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    conn.on_timeout();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1is_1established(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) -> jboolean {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    conn.is_established() as jboolean
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1is_1in_1early_1data(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) -> jboolean {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    conn.is_in_early_data() as jboolean
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1is_1closed(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) -> jboolean {
    let conn = unsafe { &*(conn_ptr as *mut Connection) };
    conn.is_closed() as jboolean
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stats(
    env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    holder: jobject,
) {
    let conn = unsafe { &*(conn_ptr as *mut Connection) };
    let stats = conn.stats();
    env.call_method(
        holder,
        "setRecv",
        "(I)V",
        &[JValue::Int(stats.recv as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setSent",
        "(I)V",
        &[JValue::Int(stats.sent as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setLost",
        "(I)V",
        &[JValue::Int(stats.lost as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setRtt",
        "(J)V",
        &[JValue::Long(stats.rtt.as_millis() as jlong)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setCwnd",
        "(I)V",
        &[JValue::Int(stats.cwnd as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setDeliveryRate",
        "(J)V",
        &[JValue::Long(stats.delivery_rate as jlong)],
    )
    .unwrap();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1free(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) {
    unsafe { Box::from_raw(conn_ptr as *mut Connection) };
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stream_1recv(
    env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    stream_id: jlong,
    java_buf: jbyteArray,
) -> jint {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    // internally executes GetByteArrayRegion
    let mut buf = env.convert_byte_array(java_buf).unwrap();
    match conn.stream_recv(stream_id as u64, &mut buf) {
        // xxx(okachaiev): find a way to convey this information
        Ok((out_len, _out_fin)) => out_len as i32,
        Err(e) => e as jint,
    }
}

// xxx(okachaiev): impelemnt "send with priority"
#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stream_1send(
    env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    stream_id: jlong,
    java_buf: jbyteArray,
    fin: jboolean,
) -> jint {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let buf_len = env.get_array_length(java_buf).unwrap() as usize;
    let (ptr, _is_copy) = env.get_byte_array_elements(java_buf).unwrap();
    let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, buf_len) };
    let sent_len = conn.stream_send(stream_id as u64, buf, fin != 0);
    env.release_byte_array_elements(
        java_buf,
        unsafe { ptr.as_mut().unwrap() },
        ReleaseMode::CopyBack,
    )
    .unwrap();
    match sent_len {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stream_1shutdown(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    stream_id: jlong,
    direction: jint,
    err: jlong,
) {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let dir = match direction {
        0 => quiche::Shutdown::Read,
        _ => quiche::Shutdown::Write,
    };
    conn.stream_shutdown(stream_id as u64, dir, err as u64)
        .unwrap();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stream_1capacity(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    stream_id: jlong,
) -> jint {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    match conn.stream_capacity(stream_id as u64) {
        Ok(v) => v as jint,
        Err(e) => e as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1stream_1finished(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    stream_id: jlong,
) -> jboolean {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    conn.stream_finished(stream_id as u64) as jboolean
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1readable(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) -> jlong {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    Box::into_raw(Box::new(conn.readable())) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1conn_1writable(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
) -> jlong {
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    Box::into_raw(Box::new(conn.writable())) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1stream_1iter_1next(
    _env: JNIEnv,
    _class: JClass,
    stream_iter_ptr: jlong,
) -> jlong {
    let stream_iter = unsafe { &mut *(stream_iter_ptr as *mut StreamIter) };
    match stream_iter.next() {
        Some(stream_id) => stream_id as jlong,
        None => Error::Done as jlong,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1config_1new(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let h3_config = h3::Config::new().unwrap();
    Box::into_raw(Box::new(h3_config)) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1conn_1new_1with_1transport(
    _env: JNIEnv,
    _class: JClass,
    conn_ptr: jlong,
    h3_config_ptr: jlong,
) -> jlong {
    let mut conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let h3_config = unsafe { &mut *(h3_config_ptr as *mut h3::Config) };
    let h3_conn = h3::Connection::with_transport(&mut conn, &h3_config).unwrap();
    Box::into_raw(Box::new(h3_conn)) as jlong
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1conn_1free(
    _env: JNIEnv,
    _class: JClass,
    h3_conn_ptr: jlong,
) {
    unsafe { Box::from_raw(h3_conn_ptr as *mut h3::Connection) };
}

fn convert_to_string<'e, V>(env: &JNIEnv<'e>, val: V) -> JNIResult<String>
where
    V: Into<JString<'e>>,
{
    Ok(env.get_string(val.into())?.into())
}

fn headers_from_java<'e>(env: &JNIEnv<'e>, headers: jobjectArray) -> JNIResult<Vec<h3::Header>> {
    let len = env.get_array_length(headers)? as i32;
    let mut buf = Vec::<h3::Header>::with_capacity(len as usize);
    for i in 0..len {
        let jobj = env.get_object_array_element(headers, i)?;
        let name = env
            .call_method(jobj, "getName", "()Ljava/lang/String;", &[])?
            .l()?;
        let value = env
            .call_method(jobj, "getValue", "()Ljava/lang/String;", &[])?
            .l()?;
        buf.push(h3::Header::new(
            &convert_to_string(&env, name)?,
            &convert_to_string(&env, value)?,
        ));
    }
    Ok(buf)
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1send_1request(
    env: JNIEnv,
    _class: JClass,
    h3_ptr: jlong,
    conn_ptr: jlong,
    headers: jobjectArray,
) {
    let h3_conn = unsafe { &mut *(h3_ptr as *mut h3::Connection) };
    let mut conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let req = headers_from_java(&env, headers).unwrap();
    h3_conn.send_request(&mut conn, &req, true).unwrap();
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1send_1response(
    env: JNIEnv,
    _class: JClass,
    h3_ptr: jlong,
    conn_ptr: jlong,
    stream_id: jlong,
    headers: jobjectArray,
    fin: jboolean,
) -> jint {
    let h3_conn = unsafe { &mut *(h3_ptr as *mut h3::Connection) };
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let req = headers_from_java(&env, headers).unwrap();
    match h3_conn.send_response(conn, stream_id as u64, &req, fin != 0) {
        Ok(_) => 0 as jint,
        // xxx(okachaiev): it's probably better not to lie about the code
        Err(h3::Error::StreamBlocked) => Error::Done as jint,
        Err(e) => h3_error_code(e) as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1send_1body(
    env: JNIEnv,
    _class: JClass,
    h3_ptr: jlong,
    conn_ptr: jlong,
    stream_id: jlong,
    java_body: jbyteArray,
    fin: jboolean,
) -> jlong {
    let h3_conn = unsafe { &mut *(h3_ptr as *mut h3::Connection) };
    let conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let body: Vec<u8> = env.convert_byte_array(java_body).unwrap();
    match h3_conn.send_body(conn, stream_id as u64, &body[..], fin != 0) {
        Ok(v) => v as jlong,
        Err(e) => h3_error_code(e) as jlong,
    }
}

fn call_on_header(
    env: &JNIEnv,
    handler: jobject,
    stream_id: u64,
    name: &str,
    value: &str,
) -> JNIResult<()> {
    env.call_method(
        handler,
        "onHeader",
        "(JLjava/lang/String;Ljava/lang/String;)V",
        &[
            JValue::Long(stream_id as jlong),
            JValue::Object(JObject::from(env.new_string(name)?)),
            JValue::Object(JObject::from(env.new_string(value)?)),
        ],
    )?;
    Ok(())
}

fn call_on_data(env: &JNIEnv, handler: jobject, stream_id: u64) -> JNIResult<()> {
    env.call_method(
        handler,
        "onData",
        "(J)V",
        &[JValue::Long(stream_id as jlong)],
    )?;
    Ok(())
}

fn call_on_finished(env: &JNIEnv, handler: jobject, stream_id: u64) -> JNIResult<()> {
    env.call_method(
        handler,
        "onFinished",
        "(J)V",
        &[JValue::Long(stream_id as jlong)],
    )?;
    Ok(())
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1conn_1poll(
    env: JNIEnv,
    _class: JClass,
    h3_conn_ptr: jlong,
    conn_ptr: jlong,
    handler: jobject,
) -> jlong {
    let h3_conn = unsafe { &mut *(h3_conn_ptr as *mut h3::Connection) };
    let mut conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    match h3_conn.poll(&mut conn) {
        Ok((stream_id, h3::Event::Headers { list, .. })) => {
            list.iter().for_each(|header| {
                call_on_header(&env, handler, stream_id, header.name(), header.value()).unwrap();
            });
            stream_id as jlong
        }
        Ok((stream_id, h3::Event::Data)) => {
            call_on_data(&env, handler, stream_id).unwrap();
            stream_id as jlong
        }
        Ok((stream_id, h3::Event::Finished)) => {
            call_on_finished(&env, handler, stream_id).unwrap();
            stream_id as jlong
        }
        Err(h3::Error::Done) => Error::Done as jlong,
        Err(e) => h3_error_code(e) as jlong,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1h3_1recv_1body(
    env: JNIEnv,
    _class: JClass,
    h3_conn_ptr: jlong,
    conn_ptr: jlong,
    stream_id: jlong,
    java_buf: jbyteArray,
) -> jint {
    let h3_conn = unsafe { &mut *(h3_conn_ptr as *mut h3::Connection) };
    let mut conn = unsafe { &mut *(conn_ptr as *mut Connection) };
    let buf_len = env.get_array_length(java_buf).unwrap() as usize;
    let (ptr, _is_copy) = env.get_byte_array_elements(java_buf).unwrap();
    let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, buf_len) };
    let body_len = h3_conn.recv_body(&mut conn, stream_id as u64, buf);
    env.release_byte_array_elements(
        java_buf,
        unsafe { ptr.as_mut().unwrap() },
        ReleaseMode::CopyBack,
    )
    .unwrap();
    match body_len {
        Ok(v) => v as jint,
        Err(e) => h3_error_code(e) as jint,
    }
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_io_quiche4j_Native_quiche_1header_1from_1slice(
    env: JNIEnv,
    _class: JClass,
    java_buf: jbyteArray,
    dcid_len: jint,
    holder: jobject,
) {
    let mut buf: Vec<u8> = env.convert_byte_array(java_buf).unwrap();
    let hdr = Header::from_slice(&mut buf, dcid_len as usize).unwrap();
    let ty_java = match hdr.ty {
        Type::Initial => 1,
        Type::Retry => 2,
        Type::Handshake => 3,
        Type::ZeroRTT => 4,
        Type::Short => 5,
        Type::VersionNegotiation => 6,
    };
    env.call_method(
        holder,
        "setPacketType",
        "(I)V",
        &[JValue::Int(ty_java as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setVersion",
        "(I)V",
        &[JValue::Int(hdr.version as jint)],
    )
    .unwrap();
    env.call_method(
        holder,
        "setDcid",
        "([B)V",
        &[env.byte_array_from_slice(&hdr.dcid).unwrap().into()],
    )
    .unwrap();
    env.call_method(
        holder,
        "setScid",
        "([B)V",
        &[env.byte_array_from_slice(&hdr.scid).unwrap().into()],
    )
    .unwrap();
    match hdr.token {
        Some(token) => {
            env.call_method(
                holder,
                "setToken",
                "([B)V",
                &[env.byte_array_from_slice(&token).unwrap().into()],
            )
            .unwrap();
        }
        None => {}
    }
    match hdr.versions {
        Some(versions) => {
            let versions_java = env.new_int_array(versions.len() as i32).unwrap();
            // xxx(okachaiev): fix the problem
            // env.set_int_array_region(versions_java, 0, &versions[..])
            //    .unwrap();
            env.call_method(holder, "setVersions", "([I)V", &[versions_java.into()])
                .unwrap();
        }
        None => {}
    }
}
