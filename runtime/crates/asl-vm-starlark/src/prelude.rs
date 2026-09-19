/// Helper script evaluated in the host stage with capability natives
/// to instantiate the unforgeable `asl_ctx` capability object.
pub fn host_prelude_script() -> &'static str {
    r#"
def _asl_make_resp(raw_resp):
    _status = raw_resp["status"]
    _body = raw_resp["body"]
    _headers = raw_resp["headers"]
    def _json_decode_body():
        if _body == None or _body.strip() == "":
            return None
        return json.decode(_body)
    return struct(
        status = _status,
        text = _body,
        headers = _headers,
        json = _json_decode_body,
    )

def _asl_http_call(method, url, headers=None, json_data=None, data=None):
    body_str = None
    if json_data != None:
        body_str = json.encode(json_data)
        if headers == None:
            headers = {"Content-Type": "application/json"}
        elif "Content-Type" not in headers and "content-type" not in headers:
            headers["Content-Type"] = "application/json"
    elif data != None:
        body_str = data

    headers_json = json.encode(headers if headers != None else {})
    raw_str = asl_native_http_request(method, url, headers_json, body_str)
    return _asl_make_resp(json.decode(raw_str))

def _asl_http_get(url, headers=None):
    return _asl_http_call("GET", url, headers=headers)

def _asl_http_post(url, headers=None, json=None, data=None):
    return _asl_http_call("POST", url, headers=headers, json_data=json, data=data)

def _asl_http_put(url, headers=None, json=None, data=None):
    return _asl_http_call("PUT", url, headers=headers, json_data=json, data=data)

def _asl_http_patch(url, headers=None, json=None, data=None):
    return _asl_http_call("PATCH", url, headers=headers, json_data=json, data=data)

def _asl_http_delete(url, headers=None):
    return _asl_http_call("DELETE", url, headers=headers)

def _asl_env_get(key, default=None):
    val = asl_native_env_get(key)
    return val if val != None else default

asl_ctx = struct(
    fs = struct(
        read = asl_native_fs_read,
        write = asl_native_fs_write,
        exists = asl_native_fs_exists,
        list = asl_native_fs_list,
    ),
    crypto = struct(
        sha256 = asl_native_sha256,
        base64_encode = asl_native_base64_encode,
        base64_decode = asl_native_base64_decode,
    ),
    env = struct(get = _asl_env_get),
    http = struct(
        get = _asl_http_get,
        post = _asl_http_post,
        put = _asl_http_put,
        patch = _asl_http_patch,
        delete = _asl_http_delete,
        call = _asl_http_call,
    ),
    fuel = struct(consumed = asl_native_fuel_consumed, remaining = asl_native_fuel_remaining),
)
"#
}

/// Pure utilities standard library script evaluated for user code.
/// Contains no I/O or ambient authority.
pub fn pure_stdlib_script() -> &'static str {
    r#"
def chars(s):
    if s == None:
        return []
    return asl_native_chars(str(s))

def is_digit(s):
    if s == None:
        return False
    return asl_native_is_digit(str(s))

def is_alpha(s):
    if s == None:
        return False
    return asl_native_is_alpha(str(s))

def is_alnum(s):
    if s == None:
        return False
    return asl_native_is_alnum(str(s))

def is_int(s):
    if s == None:
        return False
    st = str(s).strip()
    if len(st) == 0:
        return False
    if st.startswith("-"):
        st = st[1:]
    return asl_native_is_digit(st)

def to_int(s, default=None):
    if is_int(s):
        return int(str(s).strip())
    return default

def to_float(s, default=None):
    if s == None:
        return default
    st = str(s).strip()
    parts = st.split(".")
    if len(parts) == 1 and is_int(parts[0]):
        return float(st)
    if len(parts) == 2 and (is_int(parts[0]) or parts[0] == "" or parts[0] == "-") and asl_native_is_digit(parts[1]):
        return float(st)
    return default

def try_json(s, default=None):
    if s == None:
        return default
    st = str(s).strip()
    if len(st) == 0:
        return default
    if asl_native_is_json(st):
        return json.decode(st)
    return default

def strip(s):
    if s == None:
        return ""
    return str(s).strip()

def split(s, sep=None):
    if s == None:
        return []
    st = str(s)
    return st.split(sep) if sep != None else st.split()

def starts_with(s, prefix):
    if s == None or prefix == None:
        return False
    return str(s).startswith(str(prefix))

def ends_with(s, suffix):
    if s == None or suffix == None:
        return False
    return str(s).endswith(str(suffix))

def contains(s, sub):
    if s == None or sub == None:
        return False
    return str(sub) in str(s)
"#
}
